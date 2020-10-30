#![cfg(test)]

use ::phasicj_agent_svm_embed::{
    svm,
    svm_sha1_hash,
};

#[test]
fn test_svm_is_non_empty() {
    assert!(svm().len() > 0);
}

#[test]
fn test_svm_sha1_hash_length() {
    assert!(svm_sha1_hash().len() == 40);
}
