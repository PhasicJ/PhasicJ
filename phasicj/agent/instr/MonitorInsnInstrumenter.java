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
public class MonitorInsnInstrumenter {

  private final Amendment[] amendments;
  private final String eventHandlerClass;

  public MonitorInsnInstrumenter(String eventHandlerClass, Amendment... amendments) {
    this.eventHandlerClass = eventHandlerClass;
    this.amendments = amendments;
  }

  public byte[] instrument(byte[] classFileBuffer) {
    final var writer = new ClassWriter(AsmConfig.DEFAULT_CLASS_WRITER_BEHAVIOR);
    final var instrumenter = new ClassInstrumenter(AsmConfig.API_VERSION, writer);
    final var reader = new ClassReader(classFileBuffer);
    reader.accept(instrumenter, AsmConfig.DEFAULT_CLASS_READER_PARSING_BEHAVIOR);
    // TODO(dwtj): Optimize this such that we return `null` if no actual instrumentation was
    //  performed. One possible approach: add a flag to `ClassInstrumenter`, make
    //  `MethodInstrumenter` an inner class of `ClassInstrumenter`, and change the
    //  `MethodInstrumenter` to set this flag.
    return writer.toByteArray();
  }

  /** Warning: This class is not thread safe. E.g., the {@link #className} field. */
  class ClassInstrumenter extends ClassVisitor {

    private String className;

    ClassInstrumenter(int api, ClassVisitor cv) {
      super(api, cv);
    }

    @Override
    public void visit(
        int version,
        int access,
        String name,
        String signature,
        String superName,
        String[] interfaces) {
      className = name;
      super.visit(version, access, name, signature, superName, interfaces);
    }

    @Override
    public MethodVisitor visitMethod(
        int access, String name, String descriptor, String signature, String[] exceptions) {
      var delegateTo = cv.visitMethod(access, name, descriptor, signature, exceptions);
      return new MethodInstrumenter(api, delegateTo);
    }

    // NOTE(dwtj): Amendments are not instrumented. They are copied without modification.
    @Override
    public void visitEnd() {
      for (Amendment a : amendments) {
        if (this.className.equals(a.classToAmend)) {
          a.getClassMembersAmendment().accept(this);
        }
      }
      super.visitEnd();
    }
  }

  class MethodInstrumenter extends MethodVisitor {

    private final boolean applicationEventsIsInterface = false;
    private final String voidMethodDescr = Type.getMethodType(Type.VOID_TYPE).getDescriptor();

    MethodInstrumenter(int api, MethodVisitor mv) {
      super(api, mv);
    }

    @Override
    public void visitInsn(int opcode) {
      // Re-emit all instructions to delegate, but for `MONITORENTER` and `MONITOREXIT`
      // instructions,
      // emit a appropriate `INVOKESTATIC` call first.
      if (opcode == Opcodes.MONITORENTER) {
        mv.visitMethodInsn(
            Opcodes.INVOKESTATIC,
            eventHandlerClass,
            "phasicj$agent$rt$monitorEnter",
            voidMethodDescr,
            applicationEventsIsInterface);
      }
      if (opcode == Opcodes.MONITOREXIT) {
        mv.visitMethodInsn(
            Opcodes.INVOKESTATIC,
            eventHandlerClass,
            "phasicj$agent$rt$monitorExit",
            voidMethodDescr,
            applicationEventsIsInterface);
      }
      mv.visitInsn(opcode);
    }
  }
}
