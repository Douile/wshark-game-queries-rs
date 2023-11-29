use wsdf::ProtocolField;

use crate::{challenge::ValveResponseChallenge, primatives::DelimString};

pub type ValveRequestRules = ValveResponseChallenge;

#[derive(ProtocolField)]
pub struct ValveResponseRules {
    rules_len: u8,
    #[wsdf(len_field = "rules_len")]
    rules: Vec<ValveRule>,
}

#[derive(ProtocolField)]
struct ValveRule {
    name: DelimString,
    value: DelimString,
}
