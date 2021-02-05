use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "PjAgentConfLang.pest"]
struct PjAgentConfParser;
