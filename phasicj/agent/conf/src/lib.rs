pub mod parser {

    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "PjAgentConfLang.pest"]
    pub struct PjAgentConfParser;

}

use pest::Parser;
use pest::iterators::Pair;
use parser::Rule;
use parser::PjAgentConfParser;

#[derive(Debug)]
pub struct PjAgentConf {
    pub debug_dump_classes_before_instr: bool,
    pub debug_dump_classes_after_instr: bool,
}

impl PjAgentConf {
    pub fn new_with_defaults() -> PjAgentConf {
        return PjAgentConf {
            debug_dump_classes_before_instr: false,
            debug_dump_classes_after_instr: false,
        };
    }

    pub fn new_from_agent_options_list_str(options_str: &str) -> PjAgentConf {
        let mut conf = PjAgentConf::new_with_defaults();
        conf.apply_agent_options_list_str(options_str);
        return conf;
    }

    /// Parses the given string as an PhasicJ agent options list string and
    /// applies any configuration flags and arguments to this `PjAgentConf`.
    pub fn apply_agent_options_list_str(&mut self, options_str: &str) {
        let pairs = PjAgentConfParser::parse(
            Rule::agent_options_list,
            options_str
        );

        // TODO(dwtj): Handle and describe parsing failures better.
        let pairs = pairs.expect("Failed to parse PhasicJ Agent's options.");
        let options_list = pairs.peek().unwrap();
        assert_eq!(options_list.as_rule(), Rule::agent_options_list);
        for option in options_list.into_inner() {
            match option.as_rule() {
                Rule::agent_option => self.apply_agent_option_pair(option),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
    }

    fn apply_agent_option_pair(&mut self, agent_option: Pair<Rule>) {
        assert_eq!(agent_option.as_rule(), Rule::agent_option);
        let comp: Vec<_> = agent_option.into_inner().collect();
        match comp.len() {
            1 => self.apply_agent_option(comp[0].as_str(), Option::None),
            2 => self.apply_agent_option(comp[0].as_str(), Option::Some(comp[1].as_str())),
            _ => panic!("Unexpected agent option parsing state."),
        }
    }

    fn apply_agent_option(
            &mut self,
            option_name: &str,
            option_value: Option<&str>) {
        match option_name {
            "debug_dump_classes_before_instr" => {
                self.debug_dump_classes_before_instr = Self::eval_bool_option_value(option_value);
            },
            "debug_dump_classes_after_instr" => {
                self.debug_dump_classes_after_instr = Self::eval_bool_option_value(option_value);
            },
            _ => panic!("Unrecognized agent option name: {}", option_name)
        }
    }

    fn eval_bool_option_value(opt: Option<&str>) -> bool {
        match opt {
            // NOTE(dwtj): When there is no value, then it is treated treated as
            // "true" implicitly.
            None => true,
            Some("true") => true,
            Some("false") => false,
            Some(otherwise) => panic!("Unexpected agent option value: {}", otherwise),
        }
    }
}
