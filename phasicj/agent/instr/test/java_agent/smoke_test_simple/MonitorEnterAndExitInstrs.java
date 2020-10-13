package phasicj.agent.instr.test.java_agent.smoke_test_simple;

class MonitorEnterAndExitInstrs {
  public static void main(String[] args) {
    for (int idx = 0; idx < 10; idx++) {
      synchronized (MonitorEnterAndExitInstrs.class) {
        System.out.println("Hello, world!");
      }
    }
    System.out.println(phasicj.agent.rt.ApplicationEvents.getNumMonitorEnterEvents());
  }
}
