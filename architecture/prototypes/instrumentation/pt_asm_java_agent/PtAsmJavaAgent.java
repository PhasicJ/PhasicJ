import java.lang.instrument.Instrumentation;
import org.objectweb.asm.Opcodes;

class PtAsmJavaAgent {
  public static void premain(String agentArgs, Instrumentation instr) {
    System.out.println("Hello, from `PtAsmJavaAgent.premain()`");
    System.out.println("org.objectweb.asm.Opcodes.AALOAD = " + Opcodes.AALOAD);
  }
}
