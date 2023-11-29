use wsdf::tap::Field;
use wsdf::Dissect;

use crate::challenge::OptionalChallenge;
use crate::primatives::{BoolU8, DelimString};

#[derive(Dissect)]
pub struct ValveRequestInfo {
    payload: [u8; 20],
    challenge: OptionalChallenge,
}

#[derive(Dissect)]
pub struct ValveResponseInfo {
    protocol_version: u8,
    name: DelimString,
    map: DelimString,
    folder: DelimString,
    game_mode: DelimString,
    appid: u16,
    players: u8,
    max_players: u8,
    bots: u8,
    #[wsdf(decode_with = "decode_server_type")]
    server_type: u8,
    #[wsdf(decode_with = "decode_environment_type")]
    environment_type: u8,
    has_password: BoolU8,
    vac_secured: BoolU8,
    game_version: DelimString,
}

fn decode_server_type(Field(value): Field<u8>) -> &'static str {
    match value {
        b'd' => "Dedicated",
        b'l' => "Non-dedicated",
        b'p' => "SourceTV relay (proxy)",
        _ => "Unknown",
    }
}

fn decode_environment_type(Field(value): Field<u8>) -> &'static str {
    match value {
        b'l' => "Linux",
        b'w' => "Windows",
        b'm' | b'o' => "Mac",
        _ => "Unknown",
    }
}
