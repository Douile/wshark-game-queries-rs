use wsdf::tap::Field;
use wsdf::Dissect;

use crate::challenge::ValveResponseChallenge;
use crate::primatives::DelimString;

pub type ValveRequestPlayers = ValveResponseChallenge;

#[derive(Dissect)]
pub struct ValveResponsePlayers {
    players_len: u8,
    #[wsdf(len_field = "players_len")]
    players: Vec<ValvePlayer>,
}

#[derive(Dissect)]
pub struct ValvePlayer {
    index: u8,
    name: DelimString,
    #[wsdf(enc = "ENC_LITTLE_ENDIAN")]
    score: i32,
    #[wsdf(bytes, decode_with = "decode_f32_le")]
    duration: [u8; 4],
}

fn decode_f32_le(Field(value): Field<&[u8]>) -> &'static str {
    let mut bytes = [0; 4];
    bytes.copy_from_slice(value);
    let value = f32::from_le_bytes(bytes);
    Box::leak(format!("{}", value).into_boxed_str())
}
