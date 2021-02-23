package phasicj.cli;

import phasicj.cli.cmd.Pj;
import picocli.CommandLine;

public class Main {
  public static void main(String... args) {
    var cmd = new CommandLine(new Pj());
    int exitCode = cmd.execute(args);
    System.exit(exitCode);
  }
}
