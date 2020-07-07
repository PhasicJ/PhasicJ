# Java Agent Instrument Prototype

This directory implements a [Java Agent][1] which instruments JVM bytecode
on-the-fly as the JVM loads and links JVM classes. Bytecode manipulation is
performed using [ASM][2].

---

[1]: https://docs.oracle.com/en/java/javase/14/docs/api/java.instrument/java/lang/instrument/package-summary.html
[2]: https://asm.ow2.io/
