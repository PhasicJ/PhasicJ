package phasicj.cli;

import java.util.concurrent.Callable;
import picocli.CommandLine.Command;

@Command(
    name = "phasicj",
    mixinStandardHelpOptions = true,
    version = "PhasicJ v0.0.1",
    description = "Analyze and check Java programs.")
class PjCommand implements Callable<Integer> {

  @Override
  public Integer call() {
    return PjEvaluator.eval(this);
  }
}
