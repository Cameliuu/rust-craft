pub mod server;
pub mod packets;
use server::start;
fn main() -> std::io::Result<()>{
    start(String::from("127.0.0.1:25565"))?;
    Ok(())
}
