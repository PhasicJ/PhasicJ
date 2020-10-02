package pt.agent;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;

class Agent {

  private static final ClassFileTransformer monitorEnterInstrInstrumenter =
      new ClassFileTransformer() {
        @Override
        public byte[] transform(
            Module module,
            ClassLoader loader,
            String className,
            Class<?> classBeingRedefined,
            ProtectionDomain protectionDomain,
            byte[] classFileBuffer)
            throws IllegalClassFormatException {
          try {
            byte[] instrumented = MonitorEnterInstrInstrumenter.instrument(classFileBuffer);
            System.out.println(className + ": Constructed instrumented class.");
            return instrumented;
          } catch (Throwable ex) {
            System.out.println(
                className + ": Caught a `Throwable` while trying to instrument class: ");
            ex.printStackTrace();
            throw ex;
          }
        }
      };

  public static void premain(String agentArgs, Instrumentation instr) {
    System.out.println("Hello, from `MyAgent.premain()`.");

    instr.addTransformer(monitorEnterInstrInstrumenter);
  }
}
