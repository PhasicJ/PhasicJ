use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "PjAgentConfLang.pest"]
pub struct PjAgentConfParser;
