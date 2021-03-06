package phasicj.agent.instr.test.java_agent;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;
import phasicj.agent.instr.MonitorInsnInstrumenter;

public final class InstrTestJavaAgent {

  // NOTE(dwtj): For now we don't allow re-transformations. I don't yet have any idea why we should
  //  or shouldn't disallow re-transformations. We disallow it here just out of an over-abundance of
  //  caution.
  private static final boolean TRANSFORMED_CLASSES_CAN_BE_RETRANSFORMED = false;

  public static void premain(String agent_options, Instrumentation instr) {
    instr.addTransformer(
        InstrTestTransformer.getInstance(), TRANSFORMED_CLASSES_CAN_BE_RETRANSFORMED);
  }

  static class InstrTestTransformer implements ClassFileTransformer {

    private static final InstrTestTransformer singletonInstance = new InstrTestTransformer();
    private static final MonitorInsnInstrumenter instrumenter =
        new MonitorInsnInstrumenter("phasicj/agent/rt/ApplicationEvents");

    private InstrTestTransformer() {}

    public static InstrTestTransformer getInstance() {
      return singletonInstance;
    }

    @Override
    public byte[] transform(
        ClassLoader classLoader,
        String className,
        Class<?> cls,
        ProtectionDomain protectionDomain,
        byte[] bytes)
        throws IllegalClassFormatException {
      try {
        // Do not instrument classes in the `phasicj/agent/rt` package.
        if (className.startsWith("phasicj/agent/rt/")) {
          return null;
        } else {
          System.out.println(className);
          return instrumenter.instrument(bytes);
        }
      } catch (Throwable t) {
        System.err.println("Failed to transform class: " + className);
        t.printStackTrace();
        return null;
      }
    }
  }
}
