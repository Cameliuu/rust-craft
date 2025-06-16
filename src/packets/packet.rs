
use std::f64::consts::E;

use vintor::{DecodeError,decode,EncodeError};
use crate::{packets::{client_information::ClientInformationPackage, handhsake::{HandshakeError, HandshakePacket}, login::{LoginAckPacket, LoginStartPacket}, packet_reader::{PacketReader, PacketReaderError}, ping::PingPacket, status_request::StatusRequest}, state::state::ProtocolState};
impl From<DecodeError> for PacketError {
    fn from(err: DecodeError) -> Self {
        PacketError::Decode(err)
    }
}
pub trait PacketReadable: Sized {
    fn read(reader: &mut PacketReader) -> Result<Self, PacketError>;
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
    RegistryLoadFailed,
    PacketReader(PacketReaderError),
    StateNotSupported(ProtocolState)
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
impl From<PacketReaderError> for PacketError 
{
    fn from(value: PacketReaderError)-> Self {
        PacketError::PacketReader(value)
    }
}
impl Packet {
    pub fn read_from_bytes(bytes: &[u8], state: &ProtocolState) -> Result<Packet, PacketError> {
    if bytes.is_empty() {
        return Err(PacketError::UnexpectedEOF);
    }

    let mut cursor = 0;
    let mut reader = PacketReader::new(bytes);
    let packet_length = reader.read_i32()?;

    if bytes.len() < cursor + packet_length as usize {
        return Err(PacketError::UnexpectedEOF);
    }

    let packet_id = reader.read_i32()?;



    match &state {
        ProtocolState::Handshake =>
        {
            Ok(Packet::Handshake(HandshakePacket::read(&mut reader)?))
        },
        _ => Err(PacketError::StateNotSupported(*state))
    }
}

}



