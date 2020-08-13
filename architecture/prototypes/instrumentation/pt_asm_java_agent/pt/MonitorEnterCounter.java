package pt;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

public class MonitorEnterCounter {

  static final int ASM_API_VERSION = Opcodes.ASM8;

  private static final int DEFAULT_ASM_PARSING_OPTIONS = 0;

  /**
   * @param classFile A byte array to parse as a JVM class.
   * @return The number of Monitor Enter JVM bytecode instructions found in the given class.
   */
  public static int count(byte[] classFile) {
    var reader = new ClassReader(classFile);
    var counter = new CountingClassVisitor();
    reader.accept(counter, DEFAULT_ASM_PARSING_OPTIONS);
    return counter.getCount();
  }
}

/** This is only meant to be used by the `MonitorEnterCounter` class. */
class CountingClassVisitor extends ClassVisitor {

  private int count;
  private CountingMethodVisitor methodVisitor;

  CountingClassVisitor() {
    super(MonitorEnterCounter.ASM_API_VERSION);
    count = 0;
    methodVisitor = new CountingMethodVisitor();
  }

  int getCount() {
    return count;
  }

  public MethodVisitor visitMethod(
      int access,
      java.lang.String name,
      java.lang.String descriptor,
      java.lang.String signature,
      java.lang.String[] exceptions) {
    return methodVisitor;
  }

  class CountingMethodVisitor extends MethodVisitor {
    CountingMethodVisitor() {
      super(MonitorEnterCounter.ASM_API_VERSION);
    }

    public void visitInsn(int opcode) {
      if (opcode == Opcodes.MONITORENTER) {
        count++;
      }
    }
  }
}
