// This example was adapted from the gRPC C++ `helloworld` example:
// https://github.com/grpc/grpc/blob/f05d6ed3b9495b71c7b2c5922af834abe809103d/examples/cpp/helloworld/greeter_server.cc

#include <iostream>
#include <string>

#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>

#include "pt/my_protos/a.pb.h"
#include "pt/my_services/greeter.pb.h"
#include "pt/my_services/greeter.grpc.pb.h"

using std::string;
using std::cout;
using std::endl;
using std::unique_ptr;

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::Status;

using pt::my_services::Greeting;
using pt::my_services::Greeter;

class GreeterServiceImpl final : public Greeter::Service {
  Status SayHello(ServerContext* ctx, const Greeting* req, Greeting* reply) override {
    string prefix("Hello.");
    cout << req->greeting() << endl;
    reply->set_greeting(prefix);
    return Status::OK;
  }
};

void RunServer() {
  cout << "Starting server..." << endl;

  string server_address("0.0.0.0:50051");
  GreeterServiceImpl service;

  grpc::EnableDefaultHealthCheckService(true);
  grpc::reflection::InitProtoReflectionServerBuilderPlugin();

  ServerBuilder builder;
  builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
  builder.RegisterService(&service);
  unique_ptr<Server> server(builder.BuildAndStart());

  cout << "Server listening on " << server_address << endl;

  server->Wait();
}

int main(int argc, char* argv[]) {
  RunServer();
  return 0;
}
