# Rust FlatBuffer Stream Example

This is example crate shows how to stream [FlatBuffers](https://flatbuffers.dev/) messages over a `TcpStream` in Rust.

- Has separate client and server binaries for Rust.
- Uses [`tokio_util`](https://docs.rs/tokio-util/latest/tokio_util/) and the [`LengthDelimetedCodec`](https://docs.rs/tokio-util/latest/tokio_util/codec/length_delimited/struct.LengthDelimitedCodec.html#method.new) to frame streamed FlatBuffer messages with a length prefix.
- Shows how to setup the `build.rs` to compile an arbitrary number of `.fbs` files in a `schemas/` directory.
- Demonstrates how to differentiate a collection of FlatBuffer messages with a message header.

This is the best way that I could figure out to work with FlatBuffer messages, the goal being to hide the guts of the framing, build and encode/decode as much as possible. *If you know a better way, I'd love to hear about it!*.

> Note that if you work with a higher level networking tool such as [NATS](https://nats.io/) you don't need to do all the message framing yourself.  However, my observation is that FlatBuffers sacrifices some simplicity in how you construct messages to enable blisteringly fast message decode.  You probably want to use the lowest networking layer possible to leverage this speed.
