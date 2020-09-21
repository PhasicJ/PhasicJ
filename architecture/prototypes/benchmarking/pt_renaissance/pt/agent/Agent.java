package pt.agent;

import java.lang.instrument.Instrumentation;

class Agent {
  public static void premain(String args, Instrumentation instr) {
    System.out.println("Hello, from `pt.agent.Agent`.");
  }
}
