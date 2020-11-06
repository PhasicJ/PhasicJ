package phasicj.agent.svm.test;

public class TestClass {
  public static void main(String[] args) {
    synchronized (TestClass.class) {
      System.out.println("Hello, world!");
    }
  }
}
