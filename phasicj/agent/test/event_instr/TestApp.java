package phasicj.agent.test.event_instr;

import phasicj.agent.rt.ApplicationEvents;

class TestApp {
  private static final int NUM_LOOPS = 5;

  public static void main(String[] args) {
    System.out.println(
        "Hello, from `phasicj.agent.test.event_instr.TestApp`, implemented in Java.");
    for (int idx = 0; idx < NUM_LOOPS; idx++) {
      synchronized (TestApp.class) {
        System.out.println("Loop");
      }
    }

    int num = ApplicationEvents.numMonitorEnterEvents();
    System.out.print("phasicj.agent.rt.ApplicationEvents.numMonitorEnterEvents(): ");
    System.out.println(num);

    if (num >= NUM_LOOPS) {
      System.exit(0);
    } else {
      System.exit(-1);
    }
  }
}
