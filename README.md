# FlatBuffer Stream Example

This is example crate shows how to stream [FlatBuffers](https://flatbuffers.dev/) messages over a `TcpStream` in Rust.

- Has separate client and server binaries
- Uses [`tokio_util`](https://docs.rs/tokio-util/latest/tokio_util/) and the [`LengthDelimetedCodec`](https://docs.rs/tokio-util/latest/tokio_util/codec/length_delimited/struct.LengthDelimitedCodec.html#method.new) to frame streamed FlatBuffer messages with a length prefix.
- Shows how to setup the `build.rs` to compile an arbitrary number of `.fbs` files in a `schemas/` directory.
- Demonstrates how to differentiate a collection of FlatBuffer messages with a message header.
