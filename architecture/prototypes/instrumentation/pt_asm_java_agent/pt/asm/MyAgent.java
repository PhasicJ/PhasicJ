package pt.asm;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;

class MyAgent {
  private static final byte[] DO_NOT_TRANSFORM_CLASS = null;

  private static final ClassFileTransformer monitorEnterInstrCounter =
      new ClassFileTransformer() {
        @Override
        public byte[] transform(
            Module module,
            ClassLoader loader,
            String className,
            Class<?> classBeingRedefined,
            ProtectionDomain protectionDomain,
            byte[] classFileBuffer) {
          try {
            int count = MonitorEnterInstrCounter.count(classFileBuffer);
            System.out.println(
                className + ": Number of monitor enter instructions found: " + count);
          } catch (Throwable ex) {
            System.out.println(
                className + ": Caught a `Throwable` while counting monitor enter instructions: ");
            ex.printStackTrace();
          }

          return DO_NOT_TRANSFORM_CLASS;
        }
      };

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

    instr.addTransformer(monitorEnterInstrCounter);
    instr.addTransformer(monitorEnterInstrInstrumenter);
  }
}
