package phasicj.cli.cmd;

import java.util.concurrent.Callable;
import phasicj.agent.instr.cli.Cli;
import picocli.CommandLine.Command;

@Command(name = "instrument-class")
public class PjAgentInstrumentClass implements Callable<Integer> {

  @Override
  public Integer call() {
    Cli.main(null);
    return 0;
  }
}
