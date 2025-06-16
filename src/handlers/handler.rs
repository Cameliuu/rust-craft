use std::net::TcpStream;

use crate::handlers::login_handler::handle_login;
use crate::packets::packet::{Packet,PacketError};
use crate::handlers::handshake_handler::{handle_handshake,handle_status,handle_ping};
use crate::handlers::configuration_handler::{create_biomes_registry_data_packet, create_cat_variant_registry_data_packet, create_chicken_variant_registry_data_packet, create_cow_variant_registry_data_packet, create_damage_type_variant_registry_data_packet, create_dimensions_type_registry_data_packet, create_finish_configuration_packet, create_frog_variant_registry_data_packet, create_join_game_packet, create_known_packs_response, create_painting_variant_registry_data_packet, create_pig_variant_registry_data_packet, create_wolf_sound_variant_registry_data_packet, create_wolf_variant_registry_data_packet, create_registry_packs_from_file};
use crate::state::state::ProtocolState;
#[derive(Debug)]
pub struct Handler
{

}


impl Handler {
    pub fn handle_packet(data: &[u8], state: &mut ProtocolState) -> Result<Vec<Vec<u8>>, PacketError> {
        println!("[ + ] DEBUG RAW DATA  {:?}", data);
        let packet = Packet::read_from_bytes(data, state)?;
        match packet {
            Packet::Handshake(packet) => {
                *state = match packet.next_state {
                    1 => ProtocolState::Status,
                    2 => {
                        println!("[ ! ] Login requested");
                        ProtocolState::Login },
                    other => return Err(PacketError::IdNotSupported(other as u8)),
                };

                let response_bytes = handle_handshake(packet)?;
                println!("[ + ] STATE UPDATED, now in {:?}", state);
                Ok(vec![response_bytes])
            }

            Packet::StatusRequest(packet) => {
                let response_bytes = handle_status(packet)?;
                println!("[ + ] STATUS RESPONSE");
                Ok(vec![response_bytes])
            }

            Packet::Ping(packet) => {
                let response_bytes = handle_ping(packet)?;
                println!("[ + ] PONG RESPONSE {:?}", response_bytes);
                Ok(vec![response_bytes])
            },

            Packet::LoginStart(packet) => {
                println!("[ + ] CREATING LOGIN SUCCESS RESPONSE");
                let response_bytes = handle_login(packet)?;
                Ok(vec![response_bytes])
            },

            Packet::LoginAck(_) => {
                Ok(vec![]) // no response packet
            },

            Packet::LoginPluginResponse => {
                *state = ProtocolState::Configuration;
                println!("[ + ] STATE UPDATED, now in {:?}", state);
                Ok(vec![]) // no response packet here
            },

            Packet::ClientInformation(_) => {
                println!("[ + ] GENERATING KNOWN PACKETS RESPONSE");
                let response_bytes = create_known_packs_response()?;
                Ok(vec![response_bytes])
            },

            Packet::KnownPacks => {
                println!("[ + ] CREATING REGISTRY DATA PACKETS");

                
                let mut packets = Vec::new();
                /*
                packets.push(create_biomes_registry_data_packet()?);
                packets.push(create_dimensions_type_registry_data_packet()?);
                packets.push(create_painting_variant_registry_data_packet()?);
                packets.push(create_wolf_variant_registry_data_packet()?);
                packets.push(create_damage_type_variant_registry_data_packet()?);
                */
                let registry_packets = create_registry_packs_from_file()?;
                packets = registry_packets;
                packets.push(create_cat_variant_registry_data_packet()?);
                packets.push(create_wolf_sound_variant_registry_data_packet()?);
                packets.push(create_chicken_variant_registry_data_packet()?);
                packets.push(create_frog_variant_registry_data_packet()?);
                packets.push(create_pig_variant_registry_data_packet()?);
                packets.push(create_cow_variant_registry_data_packet()?);
        
                packets.push(create_finish_configuration_packet()?);

                Ok(packets)
            },
        Packet::FinishConfigurationAck => {

                *state = ProtocolState::Play;
                println!("[ + ] STATE UPDATED, now in {:?}", state);
                let response_bytes = create_join_game_packet()?;

                Ok(vec![response_bytes])
         },
         Packet::ClientTickEnd => {
             Ok(Vec::new())
         }
        }
    }
}

