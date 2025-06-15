use std::net::TcpStream;

use crate::packets::packet::{Packet,PacketError};
use crate::handlers::handshake_handler::{handle_handshake,handle_ping};
#[derive(Debug)]
pub struct Handler
{

}


impl Handler 
{
    pub fn handle_packet(data: &[u8]) -> Result<Vec<u8>,PacketError>
    {
         match Packet::read_from_bytes(data) {
            Ok(packet) => {
                match packet {
                   Packet::Handshake(packet)=> {
                       let response_bytes = handle_handshake(packet).expect("Failed to handle handshake");
                       println!("[ + ] SENDING SERVER STATUS RESPONSE");
                        Ok(response_bytes)
                    
                   },
                   Packet::Ping(packet)=>
                   {
                        let response_bytes = handle_ping(packet).expect("Failed to handle ping");
                        
                        println!("[ + ] SENDING PONG RESPONSE {:?}", response_bytes);
                        Ok(response_bytes)
                   }
                }
            },

        Err(e) => Err(e)
        }
    }
}
