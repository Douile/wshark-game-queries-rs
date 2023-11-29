use wsdf::tap::{Field, Offset, Packet};
use wsdf::{Dispatch, Protocol, ProtocolField};

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

#[derive(Protocol)]
#[wsdf(proto_desc = "Valve query protocol")]
#[wsdf(proto_name = "Valve query")]
#[wsdf(proto_filter = "valve")]
#[wsdf(decode_from = ["udp.payload", ("udp.port", 27015)])]
struct Valve {
    #[wsdf(decode_with = "decode_header")]
    header: u32,
    #[wsdf(dispatch_field = "header")]
    payload: ValveResponseFormat,
}

#[derive(ProtocolField, Dispatch)]
enum ValveResponseFormat {
    Simple(ValvePayload),
    Split(ValveResponseSplit),
    Unknown,
}

impl ValveResponseFormat {
    fn dispatch_header(header: &u32) -> ValveResponseFormatDispatch {
        use ValveResponseFormatDispatch::*;
        match *header {
            0xffffffff => Simple,
            0xfeffffff => Split,
            _ => Unknown,
        }
    }
}

#[derive(ProtocolField)]
struct ValveResponseSplit {
    #[wsdf(save)]
    id: u32,
    total: u8,
    number: u8,
    size: u16,
}

#[derive(ProtocolField)]
struct ValvePayload {
    #[wsdf(decode_with = "decode_kind")]
    kind: u8,
    #[wsdf(dispatch_field = "kind")]
    payload: ValveResponse,
}

#[derive(ProtocolField, Dispatch)]
enum ValveResponse {
    InfoRequest(ValveRequestInfo),
    InfoResponse(ValveResponseInfo),
    PlayersRequest(ValveRequestPlayers),
    PlayersResponse(ValveResponsePlayers),
    RulesRequest(ValveRequestRules),
    RulesResponse(ValveResponseRules),
    ChallengeResponse(ValveResponseChallenge),
    Unknown,
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

impl ValveResponse {
    fn dispatch_kind(kind: &u8) -> ValveResponseDispatch {
        use ValveResponseDispatch::*;
        match *kind {
            0x54 => InfoRequest,
            0x49 => InfoResponse,
            0x55 => PlayersRequest,
            0x44 => PlayersResponse,
            0x56 => RulesRequest,
            0x45 => RulesResponse,
            0x41 => ChallengeResponse,
            _ => Unknown,
        }
    }
}

fn decode_header(Field(header): Field<u32>) -> &'static str {
    match header {
        0xffffffff => "Valid",
        0xfeffffff => "Valid (Split)",
        _ => "Invalid",
    }
}
