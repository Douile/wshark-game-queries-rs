use wsdf::tap::{Offset, Packet};
use wsdf::ProtocolField;

#[derive(ProtocolField)]
pub struct ValveResponseChallenge {
    challenge: u32,
}

#[derive(ProtocolField)]
pub struct OptionalChallenge(#[wsdf(consume_with = "consume_optional")] Vec<u8>);

fn consume_optional(Offset(offset): Offset, Packet(pkt): Packet) -> (usize, String) {
    if offset == pkt.len() {
        return (0, "None".to_string());
    }

    const SIZE: usize = std::mem::size_of::<u32>();

    if offset + SIZE > pkt.len() {
        return (0, "None".to_string());
    }

    let bytes = &pkt[offset..offset + SIZE];
    let v = Some(u32::from_be_bytes(bytes.try_into().unwrap()));

    (SIZE, format!("{:?}", v))
}
