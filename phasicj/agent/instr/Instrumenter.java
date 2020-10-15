package phasicj.agent.instr;

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

  @CEntryPoint(name = "phasicj_agent_instr_Instrumenter_free")
  public static void free(IsolateThread isolateThread, CCharPointer ptr) {
    UnmanagedMemory.free(ptr);
  }

  @CEntryPoint(name = "phasicj_agent_instr_Instrumenter_instrument")
  public static void instrument(
      IsolateThread isolateThread,
      int inBufSize,
      CCharPointer inBuf,
      CIntPointer outBufSize,
      CCharPointerPointer outBufPtr) {
    byte[] inBytes = makeByteArrayFrom(inBufSize, inBuf);
    byte[] outBytes = MonitorInsnInstrumenter.instrument(inBytes);
    if (outBytes == null) {
      outBufPtr.write(null);
      outBufSize.write(0);
    } else {
      outBufPtr.write(makeCCharBufferFrom(outBytes));
      outBufSize.write(outBytes.length);
    }
  }
}
