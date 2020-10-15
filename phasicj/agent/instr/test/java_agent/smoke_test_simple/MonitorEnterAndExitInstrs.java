package phasicj.agent.instr.test.java_agent.smoke_test_simple;

import phasicj.agent.rt.ApplicationEvents;

class MonitorEnterAndExitInstrs {

  public static final int NUM_LOOPS = 5;

  public static final int ERROR_EXIT_CODE = 1;

  public static void main(String[] args) {
    for (int idx = 0; idx < NUM_LOOPS; idx++) {
      synchronized (MonitorEnterAndExitInstrs.class) {
        System.out.println("Loop.");
      }
    }

    int numEnter = ApplicationEvents.numMonitorEnterEvents();
    int numExit = ApplicationEvents.numMonitorExitEvents();

    System.out.print("Number of MONITORENTER events: ");
    System.out.println(numEnter);
    System.out.print("Number of MONITOREXIT events: ");
    System.out.println(numExit);

    if (numEnter < NUM_LOOPS || numExit < NUM_LOOPS) {
      System.err.println("An instruction event counter value was not as expected.");
      System.exit(ERROR_EXIT_CODE);
    }
  }
}
