package phasicj.cli;

import picocli.CommandLine;

public class Main {
  public static void main(String... args) {
    var cmd = new CommandLine(new PjCommand());
    int exitCode = cmd.execute(args);
    System.exit(exitCode);
  }
}
