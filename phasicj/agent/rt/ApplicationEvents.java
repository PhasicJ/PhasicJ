package phasicj.agent.rt;

import java.util.concurrent.atomic.AtomicInteger;

public class ApplicationEvents {
  private static final AtomicInteger monitorEnterCounter = new AtomicInteger(0);

  public static void monitorEnter() {
    monitorEnterCounter.incrementAndGet();
  }

  public static int getNumMonitorEnterEvents() {
    return monitorEnterCounter.get();
  }
}
