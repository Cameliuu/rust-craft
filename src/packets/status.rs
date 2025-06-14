use serde_json::json;
use vintor::{encode, encode_long, EncodeError};

pub fn create_pong_packet(total_length: i32, packet_id: u8, payload: Vec<u8>) -> Result<Vec<u8>, EncodeError> {
    let mut packet = Vec::new();

    let length_bytes = encode(total_length)?;
    packet.extend(length_bytes);

    let packet_id_bytes = encode(packet_id as i32)?;
    packet.extend(packet_id_bytes);

    packet.extend(payload);

    Ok(packet)
}

pub fn create_response_packet() -> Result<Vec<u8>, EncodeError> {
    let json_response = json!({
        "version": {
            "name": "1.21.5",
            "protocol": 770 
        },
        "players": {
            "max": 420,
            "online": 69
        },
        "description": {
            "text": "Iuli milsuge"
        }
    });
    
    let response_str = json_response.to_string();
    let response_bytes = response_str.as_bytes();
    let response_bytes_length = encode(response_bytes.len() as i32)?;

    let mut inner = Vec::new();
    inner.extend(encode(0)?);                // Packet ID = 0 for status response
    inner.extend(response_bytes_length);
    inner.extend(response_bytes);

    let mut final_packet = Vec::new();
    let inner_length = encode(inner.len() as i32)?;
    final_packet.extend(inner_length);
    final_packet.extend(inner);

    Ok(final_packet)
}

