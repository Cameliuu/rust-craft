
use std::net::{TcpListener, TcpStream};
use std::io::{Write,Read};
use std::thread;
use crate::handlers::handler::{Handler};
use crate::state::state::ProtocolState;

fn send_response(mut stream: &TcpStream, bytes: Vec<u8>) 
{
    stream.write_all(&bytes).expect("[ ! ] FAILED TO WRITE RESPONSE BYTES TO STREAM");
    stream.flush().expect("[ ! ] FAILED TO FLUSH STREAM");

}
fn handle_client(mut stream: TcpStream, state: &mut ProtocolState) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // Connection closed by client
            break;
        }

        let data = &buffer[..bytes_read];
        let response_bytes=Handler::handle_packet(data,state).expect("[ ! ] ERROR WHILE CREATING RESPONSE BYTES"); 
        send_response(&stream, response_bytes);
    }
    Ok(())
}

pub fn start(adrr: String) -> std::io::Result<()>
{
            
    let listener = TcpListener::bind(adrr)?; 

    for incoming_stream in listener.incoming() {
        println!("[ + ] OPENING NEW CONNECTION");
        match incoming_stream 
        {
            Ok(stream) =>{
                match stream.try_clone()
                {
                    Ok(cloned_stream) => {
                        
    let mut state: ProtocolState = ProtocolState::Handshake;
                        thread::spawn(move || {

                            handle_client(cloned_stream,&mut state).expect("[ ! ]  FATAL ERROR");
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
