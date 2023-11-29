use wsdf::Dissect;

use crate::{challenge::ValveResponseChallenge, primatives::DelimString};

pub type ValveRequestRules = ValveResponseChallenge;

#[derive(Dissect)]
pub struct ValveResponseRules {
    rules_len: u8,
    #[wsdf(len_field = "rules_len")]
    rules: Vec<ValveRule>,
}

#[derive(Dissect)]
struct ValveRule {
    name: DelimString,
    value: DelimString,
}
