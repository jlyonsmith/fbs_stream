use bytes::Bytes;
use fbs_stream::my_game::{Monster, MonsterArgs};
use flatbuffers::FlatBufferBuilder;
use futures::sink::SinkExt;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8765").await?;
    // Create the framed transport
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    // 1. Build your FlatBuffer message
    let mut builder = FlatBufferBuilder::new();
    let monster_name = Some(builder.create_string("Orc"));
    let monster = Monster::create(
        &mut builder,
        &MonsterArgs {
            name: monster_name,
            mana: 150,
            hp: 300,
            ..Default::default()
        },
    );
    builder.finish(monster, None);
    let message_bytes = Bytes::from(builder.finished_data().to_vec());

    framed.send(message_bytes).await?;

    println!("Sent FlatBuffer message");

    Ok(())
}
