package phasicj.cli.cmd;

import picocli.CommandLine.Command;

@Command(
    name = "agent",
    subcommands = {PjAgentInstrumentClass.class})
public class PjAgent {
  // TODO(dwtj): Everything!
}
