package phasicj.agent.instr;

import java.io.IOException;
import java.io.InputStream;

public class Instrumenter {
  public static MonitorInsnInstrumenter getInstrumenter() {
    return new MonitorInsnInstrumenter("java/lang/Object", getJavaLangObjectAmendment());
  }

  private static final String JAVA_LANG_OBJECT_AMENDMENT_RESOURCE_NAME =
      "phasicj/agent/rt/JavaLangObjectAmendment.class";

  private static Amendment getJavaLangObjectAmendment() {
    InputStream amendmentClass =
        ClassLoader.getSystemResourceAsStream(JAVA_LANG_OBJECT_AMENDMENT_RESOURCE_NAME);
    if (amendmentClass == null) {
      String msg =
          "Failed to get a system resource named " + JAVA_LANG_OBJECT_AMENDMENT_RESOURCE_NAME;
      throw new RuntimeException(msg);
    }

    try {
      return new Amendment(
          "java/lang/Object",
          "phasicj$agent$rt$",
          amendmentClass.readAllBytes(),
          "phasicj/agent/rt/JavaLangObjectAmendment");
    } catch (IOException ex) {
      // TODO(dwtj): Consider handling this better.
      throw new RuntimeException(ex);
    }
  }
}
