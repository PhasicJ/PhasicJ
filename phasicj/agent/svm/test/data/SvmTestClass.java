package phasicj.agent.svm.test;

public class SvmTestClass {
  public static void main(String[] args) {
    synchronized (SvmTestClass.class) {
      System.out.println("Hello, world!");
    }
  }
}
