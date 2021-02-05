use std::io::{self, BufRead};
use phasicj_agent_conf::{
    PjAgentConfParser as ConfParser,
    Rule,
};
use pest::Parser;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let pairs = ConfParser::parse(Rule::agent_options_list, &line).unwrap();
        let tokens: Vec<_> = pairs.flatten().into_iter().collect();
        for token in tokens {
            println!("{:?}: {:?}", token.as_rule(), token.as_span());
        }
    }
}
