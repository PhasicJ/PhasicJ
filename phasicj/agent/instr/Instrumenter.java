package phasicj.agent.instr;

public class Instrumenter {
  public static byte[] instrument(byte[] cls) {
    return MonitorInsnInstr.instrument(cls);
  }
}
