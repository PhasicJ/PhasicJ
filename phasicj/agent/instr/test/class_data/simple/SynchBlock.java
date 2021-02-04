package phasicj.agent.instr.test.class_data.simple;

public class SynchBlock {
  private static final int NUM_LOOPS = 5;

  public static void main(String[] args) {
    for (int idx = 0; idx < NUM_LOOPS; idx++) {
      synchronized (SynchBlock.class) {
        System.out.println("Hello, world!");
      }
    }
  }
}
