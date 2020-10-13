package phasicj.agent.instr;

// NOTE(dwtj): These GraalVM types are available implicitly when compiling using `javac` provided
//  with the GraalVM distribution or from `graal-sdk`.
import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;

public class Instrumenter {

  // TODO(dwtj): Figure out how to remove this unnecessary array allocation and 0-initialization.
  private static byte[] makeByteArray(int numBytes, CCharPointer bytePtr) {
    byte[] arr = new byte[numBytes];
    for (int idx = 0; idx < numBytes; idx++) {
      arr[idx] = bytePtr.read(idx);
    }
    return arr;
  }

  // TODO(dwtj): Figure out how to return the byte array.
  @CEntryPoint(name = "phasicj_agent_instr_Instrumenter_instrument")
  public static void instrument(IsolateThread isolateThread, int numBytes, CCharPointer bytePtr) {
    byte[] arr = makeByteArray(numBytes, bytePtr);
    MonitorInsnInstrumenter.instrument(arr);
  }
}
