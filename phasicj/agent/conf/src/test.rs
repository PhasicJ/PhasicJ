#![cfg(test)]

mod test_parser {

    use phasicj_agent_conf::parser::Rule;

    fn parse_flat_rule_vec(s: &str) -> Vec<Rule> {
        use phasicj_agent_conf::parser::PjAgentConfParser;
        use pest::Parser;
        let pairs = PjAgentConfParser::parse(Rule::agent_options_list, s).unwrap_or_else(|e| panic!("{}", e));
        let flattened: Vec<Rule> = pairs.flatten().into_iter().map(|token| token.as_rule()).collect();
        return flattened;
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(
            parse_flat_rule_vec(""),
            vec![
                Rule::agent_options_list,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_whitespace_only() {
        assert_eq!(
            parse_flat_rule_vec(" "),
            vec![
                Rule::agent_options_list,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_one_simple_option() {
        assert_eq!(
            parse_flat_rule_vec("hello"),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_surrounding_spaces() {
        assert_eq!(
            parse_flat_rule_vec(" hello "),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_option_list() {
        assert_eq!(
            parse_flat_rule_vec("hello,world"),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_option_list_with_space() {
        assert_eq!(
            parse_flat_rule_vec("hello, world"),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_option_value() {
        assert_eq!(
            parse_flat_rule_vec("foo=bar"),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::conf_option_arg,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_quoted_option_value() {
        assert_eq!(
            parse_flat_rule_vec(r#"foo="Bar and Baz""#),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::conf_option_arg,
                Rule::EOI,
            ]
        );
    }

    #[test]
    fn test_quoted_option_value_in_list() {
        assert_eq!(
            parse_flat_rule_vec(r#"foo="Bar and Baz", zap"#),
            vec![
                Rule::agent_options_list,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::conf_option_arg,
                Rule::agent_option,
                Rule::conf_option_name,
                Rule::EOI,
            ]
        );
    }

}

mod test_conf_defaults {

    use phasicj_agent_conf::PjAgentConf;

    #[test]
    fn test_default_conf_value() {
        let conf = PjAgentConf::new_with_defaults();
        assert_eq!(conf.debug_dump_classes_before_instr, false);
        assert_eq!(conf.debug_dump_classes_after_instr, false);
    }
}

mod test_conf_options_apply {

    use phasicj_agent_conf::PjAgentConf;

    #[test]
    fn test_empty_string() {
        let conf = PjAgentConf::new_from_agent_options_list_str("");
        assert_eq!(conf.debug_dump_classes_before_instr, false);
        assert_eq!(conf.debug_dump_classes_after_instr, false);
    }

    #[test]
    fn test_list_of_bool_options() {
        let conf = PjAgentConf::new_from_agent_options_list_str(
            "debug_dump_classes_before_instr=true,debug_dump_classes_after_instr"
        );
        assert_eq!(conf.debug_dump_classes_before_instr, true);
        assert_eq!(conf.debug_dump_classes_after_instr, true);
    }

    #[test]
    fn test_list_overriding() {
        let conf = PjAgentConf::new_from_agent_options_list_str(
            "debug_dump_classes_before_instr=true,debug_dump_classes_before_instr=false"
        );
        assert_eq!(conf.debug_dump_classes_before_instr, false);
    }

}
