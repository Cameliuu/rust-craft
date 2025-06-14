
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::io::{Write,Read};
use std::thread;
use crate::packets::handhsake::HandshakePacket;
use crate::packets::packet::{self, Packet, PacketError};
use crate::packets::ping::PingPacket;
use crate::packets::status::{create_response_packet, create_pong_packet};
fn handle_ping(ping_packet: PingPacket) -> Result<Vec<u8>,PacketError>
{
    println!("[ + ] RECEIVED PING PACKET");
    Ok(create_pong_packet(ping_packet.total_length,ping_packet.packet_id,ping_packet.bytes)?)
}
fn handle_handshake(handshake_packet: HandshakePacket) -> Result<Vec<u8>,PacketError>
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
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // Connection closed by client
            break;
        }

        let data = &buffer[..bytes_read];

        match Packet::read_from_bytes(data) {
            Ok(packet) => {
                match packet {
                   Packet::Handshake(packet)=> {
                       let response_bytes = handle_handshake(packet).expect("Failed to handle handshake");
                       println!("[ + ] SENDING SERVER STATUS RESPONSE");

                        stream.write_all(&response_bytes).expect("Filed to write pong response bytes");
                       stream.flush().expect("Failed to flush");

                   },
                   Packet::Ping(packet)=>
                   {
                        let response_bytes = handle_ping(packet).expect("Failed to handle ping");
                        
                        println!("[ + ] SENDING PONG RESPONSE {:?}", response_bytes);
                        stream.write_all(&response_bytes).expect("Filed to write pong response bytes");
                       stream.flush().expect("Failed to flush");
                   }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse packet: {:?}", e);
            }
        }
    }
    Ok(())
}

pub fn start(adrr: String) -> std::io::Result<()>
{
    
    let listener = TcpListener::bind("127.0.0.1:25565")?; 

    for incoming_stream in listener.incoming() {
        println!("[ + ] OPENING NEW CONNECTION");
        match incoming_stream 
        {
            Ok(stream) =>{
                match stream.try_clone()
                {
                    Ok(cloned_stream) => {
                        thread::spawn(move || {
                            handle_client(cloned_stream);
                        });

                    },
                    Err(e) => eprintln!("[ ! ] FAILED TO CLONE STREAM {}",e)
                }
            },
            Err(e) => eprintln!("[ ! ] CONNECTION FAILED: {}",e)
        }

    }
    println!("Hello, world!");
    Ok(())
}
