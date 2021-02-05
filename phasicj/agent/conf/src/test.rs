#![cfg(test)]

use phasicj_agent_conf::{
    PjAgentConfParser,
    Rule,
};
use pest::Parser;

fn parse_flat_rule_vec(s: &str) -> Vec<Rule> {
    let pairs = PjAgentConfParser::parse(Rule::agent_options_list, s).unwrap_or_else(|e| panic!("{}", e));
    let flattened: Vec<Rule> = pairs.flatten().into_iter().map(|token| token.as_rule()).collect();
    return flattened;
}

#[test]
fn test_options_parsing() {
    assert_eq!(
        parse_flat_rule_vec(""),
        vec![
            Rule::agent_options_list,
            Rule::EOI,
        ]
    );

    assert_eq!(
        parse_flat_rule_vec("hello"),
        vec![
            Rule::agent_options_list,
            Rule::agent_option,
            Rule::EOI,
        ]
    );

    assert_eq!(
        parse_flat_rule_vec(" hello "),
        vec![
            Rule::agent_options_list,
            Rule::agent_option,
            Rule::EOI,
        ]
    );

    assert_eq!(
        parse_flat_rule_vec("hello,world"),
        vec![
            Rule::agent_options_list,
            Rule::agent_option,
            Rule::agent_option,
            Rule::EOI,
        ]
    );

    assert_eq!(
        parse_flat_rule_vec("hello, world"),
        vec![
            Rule::agent_options_list,
            Rule::agent_option,
            Rule::agent_option,
            Rule::EOI,
        ]
    );
}
