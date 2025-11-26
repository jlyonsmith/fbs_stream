use bytes::Bytes;
use fbs_stream::msg_schema::{Header, HeaderArgs, Info, Message};
use flatbuffers::FlatBufferBuilder;
use futures::sink::SinkExt;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8765").await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    let mut builder = FlatBufferBuilder::new();
    let info = Info::create(
        &mut builder,
        &fbs_stream::msg_schema::InfoArgs {
            protocol_ver: 42,
            ..Default::default()
        },
    );
    let monster = Header::create(
        &mut builder,
        &HeaderArgs {
            msg_type: Message::Info,
            msg: Some(info.as_union_value()),
        },
    );
    builder.finish(monster, None);
    let message_bytes = Bytes::from(builder.finished_data().to_vec());

    framed.send(message_bytes).await?;

    println!("Sent FlatBuffer message");

    Ok(())
}
