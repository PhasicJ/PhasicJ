// Adapted from here:
// https://github.com/grpc/grpc-java/blob/master/examples/src/main/java/io/grpc/examples/helloworld/HelloWorldClient.java

package pt.my_client;

import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;
import pt.my_services.GreeterGrpc;
import pt.my_services.GreeterOuterClass.Greeting;

class MyClient {
  private final Channel channel;
  private final GreeterGrpc.GreeterBlockingStub blockingStub;

  MyClient(Channel channel) {
    this.channel = channel;
    this.blockingStub = GreeterGrpc.newBlockingStub(channel);
  }

  void greet() {
    var greeting = Greeting.newBuilder().setGreeting("Hello, world!").build();
    blockingStub.sayHello(greeting);
  }

  public static void main(String[] args) {
    System.out.println("Starting client...");
    String target = "localhost:50051";
    System.out.println("Opening channel to " + target);
    var channel = ManagedChannelBuilder.forTarget(target)
                                       .usePlaintext()
                                       .build();
    var client = new MyClient(channel);
    client.greet();
  }
}
