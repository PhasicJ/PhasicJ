package phasicj.cli.cmd;

import java.util.concurrent.Callable;
import picocli.CommandLine.Command;

@Command(
    name = "main",
    mixinStandardHelpOptions = true,
    version = "PhasicJ v0.0.1",
    description = "Analyze and check Java programs.",
    subcommands = {
      PjAgent.class,
    })
public class Pj implements Callable<Integer> {

  @Override
  public Integer call() {
    throw new UnsupportedOperationException("TODO(dwtj): Not yet implemented.");
  }
}
