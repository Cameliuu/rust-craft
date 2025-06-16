
use std::f64::consts::E;

use vintor::{DecodeError,decode,EncodeError};

use crate::{packets::{handhsake::{HandshakeError, HandshakePacket}, login::{LoginAckPacket, LoginStartPacket}, ping::PingPacket, status_request::StatusRequest ,client_information::ClientInformationPackage}, state::state::ProtocolState};
impl From<DecodeError> for PacketError {
    fn from(err: DecodeError) -> Self {
        PacketError::Decode(err)
    }
}
impl From<EncodeError> for PacketError {
    fn from(err: EncodeError) -> Self {
        PacketError::Encode(err)
    }
}

#[derive(Debug)]
pub enum PacketError
{
    Decode(DecodeError),
    IdNotSupported(u8),
    InvalidUtf8,
    Handshake(HandshakeError),
    UnexpectedEOF,
    Encode(EncodeError),
    RegistryLoadFailed
}
#[derive(Debug)]
pub enum Packet
{
    Handshake(HandshakePacket),
    Ping(PingPacket),
    StatusRequest(StatusRequest),
    LoginStart(LoginStartPacket),
    LoginAck(LoginAckPacket),
    LoginPluginResponse,
    ClientInformation(ClientInformationPackage),
    KnownPacks, //TO-DO PARSE THIS,
    FinishConfigurationAck,
    ClientTickEnd
}
impl Packet {
    pub fn read_from_bytes(bytes: &[u8], state: &ProtocolState) -> Result<Packet, PacketError> {
    if bytes.is_empty() {
        return Err(PacketError::UnexpectedEOF);
    }

    let mut cursor = 0;

    let (packet_length, len_bytes_read) = decode(&bytes)?;
    cursor += len_bytes_read;

    if bytes.len() < cursor + packet_length as usize {
        return Err(PacketError::UnexpectedEOF);
    }

    let packet_data = &bytes[cursor..cursor + packet_length as usize];

    let (packet_id, id_bytes_read) = decode(packet_data)?;
    if id_bytes_read > packet_data.len() {
        return Err(PacketError::UnexpectedEOF);
    }

    let payload = &packet_data[id_bytes_read..];

    match state {
        ProtocolState::Handshake => match packet_id {
            0 => {
                let handshake_packet = HandshakePacket::from_bytes(payload)?;
                Ok(Packet::Handshake(handshake_packet))
            }
            _ => Err(PacketError::IdNotSupported(packet_id as u8)),
        },

        ProtocolState::Status => match packet_id {
            0 => {
                let request_packet = StatusRequest::from_bytes(payload)?;
                Ok(Packet::StatusRequest(request_packet))
            }
            1 => {
                let ping_packet = PingPacket::from_bytes(packet_length, packet_id as u8, payload)?;
                Ok(Packet::Ping(ping_packet))
            }
            _ => Err(PacketError::IdNotSupported(packet_id as u8)),
        },
        ProtocolState::Login =>
        {
            match packet_id
            {
                0 => {

                    let login_packet = LoginStartPacket::from_bytes(payload)?;
                    Ok(Packet::LoginStart(login_packet))
                },
                2 => 
                {
                    println!("[ + ] RECEIVED LOGIN PLUGIN RESPONSE. IGNORING FOR NOW");
                    Ok(Packet::LoginPluginResponse)

                }
                3 => 
                {
                    println!("[ + ] RECEIVED LOGIN ACK");
                    let login_ack_packet = LoginAckPacket::from_bytes(payload)?;
                    Ok(Packet::LoginAck(login_ack_packet))
                },
                other => Err(PacketError::IdNotSupported(other as u8))
            }
        },
        ProtocolState::Configuration => {
            match packet_id {
                0 => {
                    let client_info_pack = ClientInformationPackage::from_bytes(payload)?;

                    println!("[ + ] RECEIVED CLIENT INFORMATION : {:?} ",client_info_pack);
                    Ok(Packet::ClientInformation(client_info_pack))
                },
                7 => {
                    println!("[ + ] RECEIVED KNOWN PACKS IGNORING FOR NOW");
                    Ok(Packet::KnownPacks)
                },
                3 => {
                    print!("[ + ] RECEIVED FINISH CONFIGURATION ACK");
                    Ok(Packet::FinishConfigurationAck)
                }

                other => Err(PacketError::IdNotSupported(other as u8))
                
            }
        },
        ProtocolState::Play => {
            match packet_id {
               11 => {
                   println!("[ + ] RECEIVED CLIENT TICK END PACKET");
                   Ok(Packet::ClientTickEnd)
               }
               other => Err(PacketError::IdNotSupported(other as u8))
            }
        }

        // Ignore Login/Play since you're not using them yet
        _ => Err(PacketError::IdNotSupported(packet_id as u8)),
    }
}

}



