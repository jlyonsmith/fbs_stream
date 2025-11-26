use fbs_stream::msg_schema::root_as_header;
use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:8765";
    let listener = TcpListener::bind(server_addr).await?;

    println!("Server running on {}", server_addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // Create the framed transport with the default LengthDelimitedCodec
            let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

            // Process incoming messages (BytesMut frames)
            while let Some(frame) = framed.next().await {
                match frame {
                    Ok(bytes_mut) => {
                        println!("Received frame with length: {}", bytes_mut.len());

                        let header = root_as_header(&bytes_mut).unwrap();

                        println!("Message Type: {:?}", header.msg_type());

                        match header.msg_type() {
                            fbs_stream::msg_schema::Message::Info => {
                                let info = header.msg_as_info().unwrap();
                                println!("Protocol Version: {}", info.protocol_ver());
                            }
                            fbs_stream::msg_schema::Message::Error => {
                                let error = header.msg_as_error().unwrap();
                                println!("Error Code: {}", error.error_no());
                                println!("Error Message: {}", error.error_string().unwrap());
                            }
                            _ => println!("Unknown message type"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading frame: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
