package pt;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;

class AsmJavaAgent {
  public static void premain(String agentArgs, Instrumentation instr) {
    System.out.println("Hello, from `AsmJavaAgent.premain()`.");

    ClassFileTransformer monitorEnterCounter =
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
              System.out.println(className + ": " + MonitorEnterCounter.count(classFileBuffer));
            } catch (Throwable ex) {
              System.out.println("Caught a throwable while transforming Class files:");
              System.out.println(ex);
            }

            return DO_NOT_TRANSFORM_CLASS;
          }
        };

    instr.addTransformer(monitorEnterCounter);

    // var classes = instr.getAllLoadedClasses();
    // for (var cls : classes) {
    //  try {
    //    instr.retransformClasses(cls);
    //  } catch (UnmodifiableClassException ex) {
    //    System.out.println("Could not modify class: " + cls.getName());
    //  }
    // }
  }
}
