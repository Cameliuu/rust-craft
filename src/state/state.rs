
#[derive(Debug,Clone, Copy)]
pub enum ProtocolState
{
    Handshake,
    Status,
    Login,
    Play,
    Configuration
}
