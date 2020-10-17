package phasicj.agent.test;

import phasicj.agent.rt.ApplicationEvents;

class TestApp {
  private static final int NUM_LOOPS = 5;

  public static void main(String[] args) {
    System.out.println("Hello, from `phasicj.agent.test.TestApp`, implemented in Java.");
    for (int idx = 0; idx < NUM_LOOPS; idx++) {
      synchronized (TestApp.class) {
        System.out.println("Loop");
      }
    }

    System.out.print("phasicj.agent.rt.ApplicationEvents.numMonitorEnterEvents(): ");
    System.out.println(ApplicationEvents.numMonitorEnterEvents());
  }
}
