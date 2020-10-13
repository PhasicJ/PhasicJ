package phasicj.agent.instr;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.Type;

/**
 * The only public interface of this class is the {@link #instrument} method. In this method we use
 * the ASM library to instrument all {@code MONITORENTER} instructions. Four ASM objects are used in
 * the transformation:
 *
 * <ol>
 *   <li>A {@link ClassReader} reads the bytecode and emits "events".
 *   <li>These events are "caught" by our {@link ClassVisitor} subclass, {@link ClassInstrumenter}.
 *       This visitor mostly re-emits the events which it receives, but with a few events added at
 *       the instrumentation sites.
 *   <li>Essentially, the only modification to the {@link ClassVisitor} is that we add our own
 *       custom {@link MethodVisitor}.
 *   <li>These events (both pass-through and instrumenter-added) are caught by a {@link
 *       ClassWriter}. This object's method {@link ClassWriter#toByteArray()} is returned as the
 *       {@link #instrument} function's output.
 * </ol>
 */
class MonitorEnterInstrInstrumenter {

  static final int ASM_API_VERSION = Opcodes.ASM8;
  static final int ASM_DEFAULT_CLASS_WRITER_BEHAVIOR = 0;
  static final int ASM_DEFAULT_CLASS_READER_PARSING_BEHAVIOR = 0;

  public static byte[] instrument(byte[] classFileBuffer) {
    final var writer = new ClassWriter(ASM_DEFAULT_CLASS_WRITER_BEHAVIOR);
    final var instrumenter = new ClassInstrumenter(ASM_API_VERSION, writer);
    final var reader = new ClassReader(classFileBuffer);
    reader.accept(instrumenter, ASM_DEFAULT_CLASS_READER_PARSING_BEHAVIOR);
    // TODO(dwtj): Optimize this such that we return `null` if no actual instrumentation was
    //  performed. One possible approach: add a flag to `ClassInstrumenter`, make
    //  `MethodInstrumenter` an inner class of `ClassInstrumenter`, and change the
    //  `MethodInstrumenter` to set this flag.
    return writer.toByteArray();
  }
}

class ClassInstrumenter extends ClassVisitor {
  ClassInstrumenter(int api, ClassVisitor cv) {
    super(api, cv);
  }

  @Override
  public MethodVisitor visitMethod(
      int access, String name, String descriptor, String signature, String[] exceptions) {
    var delegateTo = cv.visitMethod(access, name, descriptor, signature, exceptions);
    return new MethodInstrumenter(api, delegateTo);
  }
}

class MethodInstrumenter extends MethodVisitor {
  static final String printMethodOwnerInternalName = "phasicj/agent/rt/ApplicationEvents";
  static final String printMethodName = "monitorEnter";
  static final String printMethodDescriptor = Type.getMethodType(Type.VOID_TYPE).getDescriptor();
  static final boolean printMethodOwnerIsInterface = false;

  MethodInstrumenter(int api, MethodVisitor mv) {
    super(api, mv);
  }

  @Override
  public void visitInsn(int opcode) {
    // Re-emit all instructions to delegate, but for `MONITORENTER` instructions, emit an
    // `INVOKESTATIC` of the print method first.
    if (opcode == Opcodes.MONITORENTER) {
      mv.visitMethodInsn(
          Opcodes.INVOKESTATIC,
          printMethodOwnerInternalName,
          printMethodName,
          printMethodDescriptor,
          printMethodOwnerIsInterface);
    }
    mv.visitInsn(opcode);
  }
}
