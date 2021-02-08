use std::io::{self, BufRead};
use phasicj_agent_conf::PjAgentConf;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let conf = PjAgentConf::new_from_agent_options_list_str(&line);
        println!("{:?}", conf);
    }
}
