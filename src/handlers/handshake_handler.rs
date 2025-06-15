use crate::packets::handhsake::HandshakePacket;
use crate::packets::ping::PingPacket;
use crate::packets::status::{create_response_packet,create_pong_packet};
use crate::packets::packet::{Packet,PacketError};
pub fn handle_ping(ping_packet: PingPacket) -> Result<Vec<u8>,PacketError>
{
    println!("[ + ] RECEIVED PING PACKET");
    Ok(create_pong_packet(ping_packet.total_length,ping_packet.packet_id,ping_packet.bytes)?)
}
pub fn handle_handshake(handshake_packet: HandshakePacket) -> Result<Vec<u8>,PacketError>
{
    println!("Received handshake: {:?}",handshake_packet);
    match handshake_packet.next_state {
        1 => {
            println!("[ + ] DETECTED STATUS REQUEST");
            Ok(create_response_packet()?)
        },
        _ => Err(PacketError::IdNotSupported(handshake_packet.next_state as u8))
        
    }
}
