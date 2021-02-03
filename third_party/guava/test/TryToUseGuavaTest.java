import com.google.common.base.Charsets;

public class TryToUseGuavaTest {
  public static void main(String[] args) {
    if (Charsets.UTF_8.canEncode()) {
      System.out.println("We can encode UTF-8.");
    } else {
      System.out.println("We cannot encode UTF-8.");
    }
  }
}
