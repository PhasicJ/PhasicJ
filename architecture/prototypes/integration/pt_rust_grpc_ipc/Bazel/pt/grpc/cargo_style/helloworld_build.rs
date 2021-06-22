fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NOTE(dwtj): Execution of this script starts from this package. For
    // comparison, I think that Cargo will run its build script starting from
    // the root of the Cargo project. So, we first change directory to the root
    // of the Bazel workspace to make behavior more similar.
    std::env::set_current_dir("../../..")?;

    tonic_build::configure()
        .format(false)  // TODO(dwtj): Help find `rustfmt` in Bazel environment.
        .compile(
            &["pt/grpc/helloworld_service.proto"],
            &["pt/grpc"]
        )?;
    Ok(())
}
