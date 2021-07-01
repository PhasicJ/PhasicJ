fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NOTE(dwtj): Execution of this script starts from this package. For
    // comparison, I think that Cargo will run its build script starting from
    // the root of the Cargo project. So, we first change directory to the root
    // of the Bazel workspace to make behavior more similar.
    std::env::set_current_dir("../../../..")?;
    std::process::Command::new("find").args(&["."]).spawn()?;

    println!("OUT_DIR: {}", std::env::var("OUT_DIR")?);

    tonic_build::configure()
        .compile(
            &["phasicj/services/rtevents.proto"],
            &["phasicj/services"],
        )?;
    Ok(())
}
