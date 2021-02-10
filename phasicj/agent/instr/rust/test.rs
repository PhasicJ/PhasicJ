#![cfg(test)]

use ::phasicj_agent_instr::Instrumenter;
use ::phasicj_agent_conf::PjAgentConf;

#[test]
fn smoke_test_instr() {
    let conf = PjAgentConf::new_with_defaults();
    let instr = Instrumenter::new();
    // TODO(dwtj): Everything!
}
