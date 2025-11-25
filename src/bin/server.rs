use fbs_stream::my_game::root_as_monster;
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

                        let monster = root_as_monster(&bytes_mut).unwrap();

                        println!(
                            "Monster received: name = {:?}, hp = {}, mana = {}",
                            monster.name(),
                            monster.hp(),
                            monster.mana()
                        );
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
