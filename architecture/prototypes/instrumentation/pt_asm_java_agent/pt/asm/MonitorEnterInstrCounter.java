package pt.asm;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

public class MonitorEnterInstrCounter {

  static final int ASM_API_VERSION = Opcodes.ASM8;

  private static final int DEFAULT_ASM_PARSING_OPTIONS = 0;

  /**
   * Counts the number of {@code MONITORENTER} JVM bytecode instructions found in the given class.
   *
   * @param classFile A byte array to parse as a JVM class.
   * @return The number of instructions found.
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
  private final CountingMethodVisitor methodVisitor;

  CountingClassVisitor() {
    super(MonitorEnterInstrCounter.ASM_API_VERSION);
    count = 0;
    methodVisitor = new CountingMethodVisitor();
  }

  int getCount() {
    return count;
  }

  @Override
  public MethodVisitor visitMethod(
      int access, String name, String descriptor, String signature, String[] exceptions) {
    return methodVisitor;
  }

  class CountingMethodVisitor extends MethodVisitor {
    CountingMethodVisitor() {
      super(MonitorEnterInstrCounter.ASM_API_VERSION);
    }

    @Override
    public void visitInsn(int opcode) {
      if (opcode == Opcodes.MONITORENTER) {
        count++;
      }
    }
  }
}
