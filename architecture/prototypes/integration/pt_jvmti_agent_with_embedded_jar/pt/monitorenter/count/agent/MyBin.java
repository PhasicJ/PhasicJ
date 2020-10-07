package pt.monitorenter.count.agent;

import pt.monitorenter.count.agent.rt.AgentRuntime;

class MyBin {
  public static void main(String[] args) {
    int count = AgentRuntime.monitorEnterInstrCounter.getAndIncrement();
    count = AgentRuntime.monitorEnterInstrCounter.incrementAndGet();
    System.out.println("count: " + count);
  }
}
