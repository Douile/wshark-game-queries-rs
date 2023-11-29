use wsdf::tap::{Field, FieldsLocal};
use wsdf::{Dissect, Proto};

wsdf::version!("0.0.1", 4, 2);

pub mod challenge;
pub mod info;
pub mod players;
pub mod primatives;
pub mod rules;

use challenge::ValveResponseChallenge;
use info::{ValveRequestInfo, ValveResponseInfo};
use players::{ValveRequestPlayers, ValveResponsePlayers};
use rules::{ValveRequestRules, ValveResponseRules};

wsdf::protocol!(Valve);

#[derive(Proto, Dissect)]
#[wsdf(proto_desc = "Valve query protocol")]
#[wsdf(proto_name = "Valve query")]
#[wsdf(proto_filter = "valve")]
#[wsdf(decode_from = ["udp.payload", ("udp.port", 27015)])]
struct Valve {
    #[wsdf(save, decode_with = "decode_header")]
    header: u32,
    #[wsdf(get_variant = "get_format_variant")]
    payload: ValveResponseFormat,
}

#[derive(Dissect)]
enum ValveResponseFormat {
    Simple(ValvePayload),
    Split(ValveResponseSplit),
    UnknownFormat,
}

fn get_format_variant(FieldsLocal(fields): FieldsLocal) -> &'static str {
    let header = fields.get_u32("header").unwrap();
    match *header {
        0xffffffff => "Simple",
        0xfeffffff => "Split",
        _ => "UnknownFormat",
    }
}

#[derive(Dissect)]
struct ValveResponseSplit {
    id: u32,
    total: u8,
    number: u8,
    size: u16,
    //#[wsdf(subdissector = "valve.payload")]
    //payload: Vec<u8>,
}

#[derive(Dissect)]
struct ValvePayload {
    #[wsdf(save, decode_with = "decode_kind")]
    kind: u8,
    #[wsdf(get_variant = "get_response_variant")]
    payload: ValveResponse,
}

#[derive(Dissect)]
enum ValveResponse {
    InfoRequest(ValveRequestInfo),
    InfoResponse(ValveResponseInfo),
    PlayersRequest(ValveRequestPlayers),
    PlayersResponse(ValveResponsePlayers),
    RulesRequest(ValveRequestRules),
    RulesResponse(ValveResponseRules),
    ChallengeResponse(ValveResponseChallenge),
    UnknownResponse,
}

fn decode_kind(Field(kind): Field<u8>) -> &'static str {
    match kind {
        0x54 => "Info request (A2S_INFO)",
        0x49 => "Info response (A2S_INFO)",
        0x6d => "GoldSource info response (A2S_INFO)",
        0x55 => "Players request (A2S_PLAYERS)",
        0x44 => "Players response (A2S_PLAYERS)",
        0x56 => "Rules request (A2S_RULES)",
        0x45 => "Rules response (A2S_RULES)",
        0x69 => "Ping request (A2A_PING)",
        0x6a => "Ping response (A2A_PING)",
        0x57 => "Challenge request (A2S_SERVERQUERY_GETCHALLENGE)",
        0x41 => "Challenge response (A2S_SERVERQUERY_GETCHALLENGE)",
        _ => "unknown",
    }
}

fn get_response_variant(FieldsLocal(fields): FieldsLocal) -> &'static str {
    let kind = fields.get_u8("kind").unwrap();
    match *kind {
        0x54 => "InfoRequest",
        0x49 => "InfoResponse",
        0x55 => "PlayersRequest",
        0x44 => "PlayersResponse",
        0x56 => "RulesRequest",
        0x45 => "RulesResponse",
        0x41 => "ChallengeResponse",
        _ => "UnknownResponse",
    }
}

fn decode_header(Field(header): Field<u32>) -> &'static str {
    match header {
        0xffffffff => "Valid",
        0xfeffffff => "Valid (Split)",
        _ => "Invalid",
    }
}
