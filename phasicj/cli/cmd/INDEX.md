# PhasicJ CLI Executable Command & Sub-Commands

The PhasicJ command-line interface executable's is intended to have various
features. These various features are organized by sub-commands (in a style
similar to Git).

Command line arguments are parsed using the picocli library. This directory
holds the declarative configuration of picocli. The different classes here
define the root command, `Pj`, its sub-commands (e.g. `PjAgent`), and its
sub-sub-commands (e.g., `PjAgentInstrumentClass`).

The picocli documentation on how to create [subcommands][1] and
[sub-sub-commands][2].

---

[1]: https://picocli.info/#_subcommands
[2]: https://picocli.info/#_nested_sub_subcommands
