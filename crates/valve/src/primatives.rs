use wsdf::tap::{Field, Offset, Packet};
use wsdf::{Dispatch, Protocol, ProtocolField};

/// A null delimited string
#[derive(ProtocolField)]
pub struct DelimString(#[wsdf(consume_with = "consume_string")] Vec<u8>);

const STRING_DELIM: u8 = 0;

fn consume_string(Offset(offset): Offset, Packet(pkt): Packet) -> (usize, String) {
    if offset >= pkt.len() {
        return (
            0,
            format!("Unexpected EOF (expected more bytes at offset {})", offset),
        );
    }

    let mut i = offset;

    loop {
        if pkt[i] == STRING_DELIM {
            break;
        }

        i += 1;
    }

    (
        i - offset + 1,
        std::str::from_utf8(&pkt[offset..i])
            .unwrap_or("<invalid utf8>")
            .to_string(),
    )
}

/// A u8 that is true if 1
#[derive(ProtocolField)]
pub struct BoolU8(#[wsdf(decode_with = "decode_bool")] u8);

fn decode_bool(Field(b): Field<u8>) -> &'static str {
    match b {
        1 => "true",
        _ => "false",
    }
}
