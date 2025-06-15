use std::net::TcpStream;

use crate::handlers::login_handler::handle_login;
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
    println!("[ + ] DEBUG RAW DATA  {:?}",data);
    let packet = Packet::read_from_bytes(data, state)?;
    match packet {
        Packet::Handshake(packet) => {
            *state = match packet.next_state {
                1 => ProtocolState::Status,
                2 => {
                    println!("[ ! ] Login requested"); 
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
        },
        Packet::LoginStart(packet) =>
        {
            println!("[ + ] CREATING LOGIN SUCCESS RESPONSE");
            let response_bytes = handle_login(packet)?;
            Ok(response_bytes)
        },
        Packet::LoginAck(packet) => 
        {
            Ok(Vec::new())
        },
        Packet::LoginPluginResponse =>
        {
            *state = ProtocolState::Configuration;
            println!("[ + ] STATE UPDATED, now in {:?}", state);
                Ok(Vec::new())
        },
        Packet::ClientInformation(packet) => {
                
            *state = ProtocolState::Play;
            println!("[ + ] STATE UPDATED, now in {:?}", state);
                Ok(Vec::new())
        }
    }
}

}
