use crate::packets::handhsake::HandshakePacket;
use crate::handlers::login_handler::handle_login;
use crate::packets::ping::PingPacket;
use crate::packets::status::{create_response_packet,create_pong_packet};
use crate::packets::packet::{Packet,PacketError};
use crate::packets::status_request::StatusRequest;
pub fn handle_ping(ping_packet: PingPacket) -> Result<Vec<u8>,PacketError>
{
    println!("[ + ] RECEIVED PING PACKET");
    Ok(create_pong_packet(ping_packet.total_length,ping_packet.packet_id,ping_packet.bytes)?)
}
pub fn handle_status(status_packet: StatusRequest) -> Result<Vec<u8>,PacketError>
{

            println!("[ + ] DETECTED STATUS REQUEST");
            Ok(Vec::new())
}
pub fn handle_handshake(handshake_packet: HandshakePacket) -> Result<Vec<u8>,PacketError>
{
    println!("Received handshake: {:?}",handshake_packet);
    match handshake_packet.next_state {
        1 => {
            println!("[ + ] DETECTED PING REQUEST");
            Ok(create_response_packet()?)
        },
        2 => {
            println!("[ + ] DETECTED LOGIN HANDHSAKE");
            Ok(Vec::new())
        }
        _ => Err(PacketError::IdNotSupported(handshake_packet.next_state as u8))
        
    }
}
