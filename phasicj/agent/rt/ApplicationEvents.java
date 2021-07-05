package phasicj.agent.rt;

import java.util.concurrent.atomic.AtomicInteger;

public class ApplicationEvents {

  private static native void monitorEnter();

  private static native void monitorExit();

  private static final AtomicInteger monitorEnterCounter = new AtomicInteger(0);
  private static final AtomicInteger monitorExitCounter = new AtomicInteger(0);

  public static void phasicj$agent$rt$monitorEnter() {
    monitorEnterCounter.incrementAndGet();
    monitorEnter();
  }

  public static void phasicj$agent$rt$monitorExit() {
    monitorExitCounter.incrementAndGet();
    monitorExit();
  }

  public static int numMonitorEnterEvents() {
    return monitorEnterCounter.get();
  }

  public static int numMonitorExitEvents() {
    return monitorExitCounter.get();
  }
}
