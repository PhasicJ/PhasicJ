#![cfg(test)]

use ::phasicj_agent_rt_jar_embed::{
    rt_jar,
    rt_jar_sha1_hash,
};

#[test]
fn test_rt_jar_is_non_empty() {
    assert!(rt_jar().len() > 0);
}

#[test]
fn test_rt_jar_sha1_hash_length() {
    assert!(rt_jar_sha1_hash().len() == 40);
}
