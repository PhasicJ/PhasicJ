package pt;

class MyTestApp {
  public static void main(String[] args) {
    synchronized (MyTestApp.class) {
      System.out.println("Hello, from `MyApp.main()`.");
    }
  }
}
