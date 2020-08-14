package pt.jvm.unmodifiableclasses;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.lang.instrument.UnmodifiableClassException;
import java.security.ProtectionDomain;

class MyAgent {

  public static void premain(String agentArgs, Instrumentation instr) {
    System.out.println("Hello, from `MyAgent.premain()`.");

    ClassFileTransformer doNothingTransformer =
        new ClassFileTransformer() {
          private final byte[] DO_NOT_TRANSFORM_CLASS = null;

          public byte[] transform(
              Module module,
              ClassLoader loader,
              String className,
              Class<?> classBeingRedefined,
              ProtectionDomain protectionDomain,
              byte[] classFileBuffer) {
            return DO_NOT_TRANSFORM_CLASS;
          }
        };

    instr.addTransformer(doNothingTransformer);

    int numModified = 0;
    int numUnmodifiable = 0;

    var classes = instr.getAllLoadedClasses();
    for (var cls : classes) {
      try {
        instr.retransformClasses(cls);
        numModified++;
      } catch (UnmodifiableClassException ex) {
        System.out.println("Could not retransform class: " + cls.getName());
        numUnmodifiable++;
      }
    }

    System.out.print("Number of Retransformed Classes: ");
    System.out.println(numModified);
    System.out.print("Number of Unmodifiable Classes: ");
    System.out.println(numUnmodifiable);
  }
}
