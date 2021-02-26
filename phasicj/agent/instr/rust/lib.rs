use std::process::{
    Command,
    Stdio,
    Output,
};
use std::io::{
    self,
    Write,
};
use std::option::Option;

static DEFAULT_PHASICJ_EXEC: &'static str = "phasicj";

pub struct Instrumenter <'a> {
    phasicj_exec: Option<&'a str>,
}

impl Instrumenter <'static> {
    pub fn using_system_path() -> Instrumenter<'static> {
        Instrumenter {
            phasicj_exec: None,
        }
    }
}

impl <'a> Instrumenter<'a> {
    pub fn wrap(phasicj_exec: &'a str) -> Instrumenter<'a> {
        Instrumenter {
            phasicj_exec: Some(phasicj_exec)
        }
    }

    // TODO(dwtj): Improve error handling throughout this function.
    pub fn instrument(&self, class_data: &[u8]) -> io::Result<Vec<u8>> {
        let output = self.invoke_phasicj_agent_instrument_class(class_data)?;
        if !output.status.success() {
            // TODO(dwtj): Consider collecting stderr.
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invocation of `phasicj agent instrument-class` failed.",
            ))
        }
        return Ok(output.stdout);
    }

    fn invoke_phasicj_agent_instrument_class(&self, class_data: &[u8]) -> io::Result<Output> {
        let mut child = match &self.phasicj_exec {
            None => Command::new(DEFAULT_PHASICJ_EXEC),
            Some(path) => Command::new(&path),
        };
        let mut child = child
            .arg("agent")
            .arg("instrument-class")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // TODO(dwtj): Improve error handling here.
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(class_data)?;

        // NOTE(dwtj): [`wait_with_output()`][1] closes stdin before waiting.
        //
        // [1]: https://doc.rust-lang.org/std/process/struct.Child.html#method.wait_with_output
        return child.wait_with_output();
    }
}
