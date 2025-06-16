pub mod server;
pub mod packets;
pub mod handlers;
pub mod state;
pub mod utils;
use server::start;
fn main() -> std::io::Result<()>{
    start(String::from("127.0.0.1:25565"))?;
    Ok(())
}
