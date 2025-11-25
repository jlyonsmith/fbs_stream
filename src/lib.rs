#[allow(warnings)]
mod gen_flatbuffers {
    include!(concat!(env!("OUT_DIR"), "/flatbuffers/mod.rs"));
}

pub use gen_flatbuffers::*;
