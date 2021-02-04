package phasicj.agent.instr.svm;

// NOTE(dwtj): These GraalVM types are available implicitly when compiling using `javac` provided
//  with the GraalVM distribution or from `graal-sdk`.
import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.UnmanagedMemory;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CCharPointerPointer;
import org.graalvm.nativeimage.c.type.CIntPointer;

public class Instrumenter {

  // TODO(dwtj): Figure out how to remove this unnecessary array allocation and 0-initialization.
  private static byte[] makeByteArrayFrom(int numBytes, CCharPointer bytePtr) {
    byte[] bytes = new byte[numBytes];
    for (int idx = 0; idx < numBytes; idx++) {
      bytes[idx] = bytePtr.read(idx);
    }
    return bytes;
  }

  private static CCharPointer makeCCharBufferFrom(byte[] bytes) {
    CCharPointer buf = UnmanagedMemory.malloc(bytes.length);
    for (int idx = 0; idx < bytes.length; idx++) {
      buf.write(idx, bytes[idx]);
    }
    return buf;
  }

  @CEntryPoint(name = "svm_instr_free")
  public static void free(IsolateThread isolateThread, CCharPointer ptr) {
    UnmanagedMemory.free(ptr);
  }

  /**
   * Instruments and returns the given class file data.
   *
   * <p>This method will not write to `inBuf`. It will only read from it.
   *
   * <p>The given `outBufSize` is expected to point to an `int` which this method will overwrite.
   *
   * <p>The given `outBufPtr` is expected to point to a `char *` which this method will overwrite.
   * This `char *` may be overwritten with the null pointer, indicating that no instrumentation
   * should be performed. Otherwise, This `char *` will be overwritten with a buffer of memory
   * managed by the SVM, specifically, allocated from `UnmanagedMemory.malloc()`.
   *
   * <p>The SVM will neither read from nor write to this output buffer until it is passed to `free`.
   * This buffer should be passed to `free` with the same `IsolateThread` from which it was
   * obtained.
   *
   * <p>Returns 0 if this method was successful and non-zero otherwise.
   */
  @CEntryPoint(name = "svm_instr_instrument")
  public static int instrument(
      IsolateThread isolateThread,
      int inBufSize,
      CCharPointer inBuf,
      CIntPointer outBufSize,
      CCharPointerPointer outBufPtr) {

    try {
      byte[] inBytes = makeByteArrayFrom(inBufSize, inBuf);
      // TODO(dwtj): Consider reusing a statically initialized instrumenter. Re-loading the system
      //  resource for every class instrumentation is bad. But be careful about silencing
      //  exceptions which arise during initialization.
      byte[] outBytes = phasicj.agent.instr.Instrumenter.getInstrumenter().instrument(inBytes);

      if (outBytes == null) {
        outBufPtr.write(null);
        outBufSize.write(0);
      } else {
        outBufPtr.write(makeCCharBufferFrom(outBytes));
        outBufSize.write(outBytes.length);
      }
    } catch (Throwable t) {
      System.err.println("An error occurred while instrumenting a class.");
      t.printStackTrace();
      outBufPtr.write(null);
      outBufSize.write(0);
      return -1;
    }

    return 0;
  }
}
