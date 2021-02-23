package phasicj.cli.cmd;

import java.util.concurrent.Callable;
import picocli.CommandLine.Command;

@Command(name = "instrument-class")
public class PjAgentInstrumentClass implements Callable<Integer> {

  @Override
  public Integer call() {
    // TODO(dwtj): Everything!
    // throw new UnsupportedOperationException("TODO(dwtj): Not yet implemented.");
    return 0;
  }
}
