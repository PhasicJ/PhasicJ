package phasicj.agent.rt;

/**
 * The members of this class are meant to be added to {@link java.lang.Object} by the PhasicJ agent.
 *
 * <p>These methods are meant to forward events to the PhasicJ runtime only after instrumentation.
 * Instrumentation isn't meant to be disabled.
 *
 * <p>There are lots of ways in which something can go wrong when the members of this class are
 * grafted onto {@link java.lang.Object}. The amendment code is not robust or safe or conservative.
 * See the documentation for {@code phasicj.agent.instr.Amendment} for some guidelines to follow.
 */
public class JavaLangObjectAmendment {

  private static volatile boolean phasicj$agent$rt$instrumentationIsEnabled;

  public static void phasicj$agent$rt$enableInstrumentation() {
    phasicj$agent$rt$instrumentationIsEnabled = true;
  }

  public static void phasicj$agent$rt$monitorEnter() {
    if (phasicj$agent$rt$instrumentationIsEnabled) {
      ApplicationEvents.phasicj$agent$rt$monitorEnter();
    }
  }

  public static void phasicj$agent$rt$monitorExit() {
    if (phasicj$agent$rt$instrumentationIsEnabled) {
      ApplicationEvents.phasicj$agent$rt$monitorExit();
    }
  }
}
