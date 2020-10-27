package phasicj.agent.instr;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.FieldVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.commons.ClassRemapper;
import org.objectweb.asm.commons.Remapper;
import org.objectweb.asm.commons.SimpleRemapper;

/**
 * This class supports our hack to "patch" or "amend" {@link java.lang.Object}.
 *
 * <p>This class is currently very limited and unsafe. There are lots of unspecified ways in which
 * its use can go wrong. It should not be used in other contexts besides our hack to amend {@link
 * java.lang.Object}.
 *
 * <p>The amendment process is designed to add members (i.e. fields and methods) from a pre-compiled
 * {@link #amendWithClass} to a {@link #classToAmend}. Only members from {@link #amendWithClass}
 * with names which start with {@link #amendmentMembersPrefix} will be amended to the {@link
 * #classToAmend}.
 *
 * <p>The are lots of other constraints on valid {@link #amendWithClass}, but they have not yet been
 * fully specified. Here is a list of speculative initial guidelines on {@link #classToAmend} which
 * we have identified so far:
 *
 * <ul>
 *   <li>The {@link #classToAmend} must be a class, not an interface.
 *   <li>No inner classes.
 *   <li>No anonymous classes.
 *   <li>No inheritance.
 *   <li>No use of classes outside of {@code java.base} module.
 *   <li>Member names should be prefixed by {@code phasicj$agent$rt$} to prevent name collisions.
 *   <li>No instance members (only static members).
 *   <li>This class's static initialization method (i.e. {@code <cinit>}) will not be amended.
 *       Therefore, correct behavior of the amendment must not depend upon either a {@code static}
 *       initialization block or on field initializers. (Note that fields will be initialized by the
 *       JVM to their default values implicitly. E.g., a {@code boolean} field is implicitly
 *       initially {@code false} event without {@code <cinit>}.)
 * </ul>
 */
public class Amendment {

  public final String classToAmend;
  public final String amendmentMembersPrefix;
  private final byte[] amendWithClass;
  public final String amendWithClassName;

  public Amendment(
      String classToAmend,
      String amendmentMembersPrefix,
      byte[] amendWithClass,
      String amendWithClassName) {
    this.classToAmend = classToAmend;
    this.amendmentMembersPrefix = amendmentMembersPrefix;
    this.amendWithClass = renameClass(amendWithClassName, amendWithClass, classToAmend);
    this.amendWithClassName = amendWithClassName;
  }

  private static byte[] renameClass(String oldClassName, byte[] cls, String newClassName) {
    ClassReader reader = new ClassReader(cls);
    ClassWriter writer = new ClassWriter(AsmConfig.DEFAULT_CLASS_WRITER_BEHAVIOR);
    Remapper remapper = new SimpleRemapper(oldClassName, newClassName);
    ClassVisitor visitor = new ClassRemapper(writer, remapper);
    reader.accept(visitor, AsmConfig.DEFAULT_CLASS_READER_PARSING_BEHAVIOR);
    return writer.toByteArray();
  }

  public ClassMembersAmendment getClassMembersAmendment() {
    return new ClassMembersAmendment();
  }

  public class ClassMembersAmendment {

    private ClassMembersAmendment() {}

    /**
     * This will forward the methods and fields of the {@link #amendWithClass} by calling the
     * argument's {@link ClassVisitor#visitMethod} and {@link ClassVisitor#visitField} methods.
     */
    public void accept(final ClassVisitor sink) {
      ClassReader reader = new ClassReader(amendWithClass);
      ClassVisitor forwardMembersToSink =
          new ClassVisitor(AsmConfig.API_VERSION) {
            @Override
            public FieldVisitor visitField(
                int access, String name, String descriptor, String signature, Object value) {
              if (name.startsWith(amendmentMembersPrefix)) {
                return sink.visitField(access, name, descriptor, signature, value);
              } else {
                return null;
              }
            }

            @Override
            public MethodVisitor visitMethod(
                int access, String name, String descriptor, String signature, String[] exceptions) {
              if (name.startsWith(amendmentMembersPrefix)) {
                return sink.visitMethod(access, name, descriptor, signature, exceptions);
              } else {
                return null;
              }
            }
          };
      reader.accept(forwardMembersToSink, AsmConfig.DEFAULT_CLASS_READER_PARSING_BEHAVIOR);
    }
  }
}
