package phasicj.agent.instr.cli;

import com.google.common.io.ByteStreams;
import java.io.IOException;
import phasicj.agent.instr.Instrumenter;

public class Cli {

  private static final byte[] MAGIC_NUMBER = {(byte) 0xCA, (byte) 0xFE, (byte) 0xBA, (byte) 0xBE};

  private static boolean hasMagicNumber(byte[] data) {
    if (data.length < MAGIC_NUMBER.length) {
      return false;
    }
    for (int idx = 0; idx < MAGIC_NUMBER.length; idx++) {
      if (data[idx] != MAGIC_NUMBER[idx]) {
        return false;
      }
    }
    return true;
  }

  private static boolean appearsToBeClassData(byte[] data) {
    return hasMagicNumber(data);
  }

  public static void main(String[] args) {
    byte[] class_data;
    try {
      class_data = ByteStreams.toByteArray(System.in);
    } catch (IOException ex) {
      throw new RuntimeException("An `IOException` occurred while reading from stdin.", ex);
    }

    if (!appearsToBeClassData(class_data)) {
      System.err.println("Data from stdin does not appear to be a JVM class file.");
      System.err.println("It does not begin with the expected magic number, 0xCAFEBABE.");
      System.exit(-1);
    }

    var instr_class_data = Instrumenter.getInstrumenter().instrument(class_data);

    System.out.write(instr_class_data, 0, instr_class_data.length);
  }
}
