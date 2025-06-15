use std::net::TcpStream;

use crate::packets::packet::{Packet,PacketError};
use crate::handlers::handshake_handler::{handle_handshake,handle_status,handle_ping};
use crate::state::state::ProtocolState;
#[derive(Debug)]
pub struct Handler
{

}


impl Handler 
{
    
pub fn handle_packet(data: &[u8], state: &mut ProtocolState) -> Result<Vec<u8>, PacketError> {
    let packet = Packet::read_from_bytes(data, state)?;

    match packet {
        Packet::Handshake(packet) => {
            *state = match packet.next_state {
                1 => ProtocolState::Status,
                2 => {
                    println!("[ ! ] Login requested but not implemented");
                    ProtocolState::Login
                },
                other => return Err(PacketError::IdNotSupported(other as u8)),
            };

            let response_bytes = handle_handshake(packet)?;
            println!("[ + ] STATE UPDATED, now in {:?}", state);
            Ok(response_bytes)
        }

        Packet::StatusRequest(packet) => {
            let response_bytes = handle_status(packet)?;
            println!("[ + ] STATUS RESPONSE");
            Ok(response_bytes)
        }

        Packet::Ping(packet) => {
            let response_bytes = handle_ping(packet)?;
            println!("[ + ] PONG RESPONSE {:?}", response_bytes);
            Ok(response_bytes)
        }
    }
}

}
