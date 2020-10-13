package phasicj.agent.instr.test.java_agent.smoke_test_simple;

class MonitorEnterAndExitInstrs {

  public static final int NUM_LOOPS = 5;

  public static final int ERROR_EXIT_CODE = 1;

  public static void main(String[] args) {
    for (int idx = 0; idx < NUM_LOOPS; idx++) {
      synchronized (MonitorEnterAndExitInstrs.class) {
        System.out.println("Loop.");
      }
    }

    int numEnter = phasicj.agent.rt.ApplicationEvents.numMonitorEnterEvents();
    int numExit = phasicj.agent.rt.ApplicationEvents.numMonitorExitEvents();

    System.out.println("Number of MONITORENTER events: " + numEnter);
    System.out.println("Number of MONITOREXIT events: " + numExit);

    if (numEnter < NUM_LOOPS || numExit < NUM_LOOPS) {
      System.err.println("An instruction event counter value was not as expected.");
      System.exit(ERROR_EXIT_CODE);
    }
  }
}
