fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let out = std::process::Command::new("find").arg("../../..").output();
    // println!("{:?}", out);
    // tonic_build::compile_protos("helloworld_service.proto")?;
    std::env::set_current_dir("../../..")?;
    tonic_build::configure()
        .format(false)  // TODO(dwtj): Help find `rustfmt` in Bazel environment.
        .compile(
            &["pt/grpc/helloworld_service.proto"],
            &["pt/grpc"]
        )?;
    Ok(())
}
