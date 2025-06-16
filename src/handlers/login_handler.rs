use crate::packets::{login::LoginStartPacket, packet::PacketError};
use vintor::{encode, EncodeError};

pub fn handle_login(login_start_packet: LoginStartPacket) -> Result<Vec<u8>,PacketError>
{
    println!("[ + ] PLAYER {} IS TRYING TO CONNECT WITH UUID {}",login_start_packet.player_name, login_start_packet.uuid);
    let mut inner = Vec::new();
    let mut response = Vec::new();

    let packet_id = encode(2)?;
    inner.extend(packet_id);
    let uuid_bytes = login_start_packet.uuid.as_bytes();
    inner.extend(uuid_bytes);

    let player_name_length = encode(login_start_packet.player_name.len() as i32)?;
    inner.extend(player_name_length);
    inner.extend(login_start_packet.player_name.as_bytes());
    
    let empty_encoded = encode(0)?;
    inner.extend(empty_encoded);

    let total_length = encode(inner.len() as i32)?;
    response.extend(total_length);
    response.extend(inner);
    
    Ok(response)
}
