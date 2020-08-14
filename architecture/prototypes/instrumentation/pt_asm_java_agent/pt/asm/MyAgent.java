package pt.asm;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;

class MyAgent {

  public static void premain(String agentArgs, Instrumentation instr) {
    System.out.println("Hello, from `MyAgent.premain()`.");

    ClassFileTransformer monitorEnterInstrCounter =
        new ClassFileTransformer() {
          private final byte[] DO_NOT_TRANSFORM_CLASS = null;

          public byte[] transform(
              Module module,
              ClassLoader loader,
              String className,
              Class<?> classBeingRedefined,
              ProtectionDomain protectionDomain,
              byte[] classFileBuffer) {
            try {
              System.out.println(
                  className + ": " + MonitorEnterInstrCounter.count(classFileBuffer));
            } catch (Throwable ex) {
              System.out.println("Caught a `Throwable` while transforming class: " + className);
              ex.printStackTrace();
            }

            return DO_NOT_TRANSFORM_CLASS;
          }
        };

    instr.addTransformer(monitorEnterInstrCounter);
  }
}
