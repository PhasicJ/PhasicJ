#![cfg(test)]

use phasicj_agent_conf::{
    PjAgentConfParser,
    Rule,
};
use pest::Parser;

fn num_tokens_when_parsed_and_flattened(s: &str) -> usize {
    let pairs = PjAgentConfParser::parse(Rule::agent_options_list, s).unwrap_or_else(|e| panic!("{}", e));
    let tokens: Vec<_> = pairs.flatten().into_iter().collect();
    assert_eq!(tokens[0].as_rule(), Rule::agent_options_list);
    return tokens.len();
}


#[test]
fn test_options_parsing() {
    // TODO(dwtj): Test that the right number of options was parsed.
    assert_eq!(num_tokens_when_parsed_and_flattened(""), 3);
    assert_eq!(num_tokens_when_parsed_and_flattened("hello"), 3);
    assert_eq!(num_tokens_when_parsed_and_flattened(" hello "), 3);
    assert_eq!(num_tokens_when_parsed_and_flattened("hello,world"), 4);
    assert_eq!(num_tokens_when_parsed_and_flattened("hello, world"), 4);
}
