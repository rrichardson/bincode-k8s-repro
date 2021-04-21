
use serde_json;
use bincode::{Options, DefaultOptions};
use k8s_openapi::api::core::v1::Endpoints;
mod blobs;

fn main() {
    // Deserializes from JSON just fine
    let ep : Endpoints = serde_json::from_str(blobs::EXAMPLE1).expect("unpack from json");

    // To bincode bits
    let blob = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .serialize(&ep)
            .expect("serialize with bincode");

    // Deserializing from bincode fails
    let bin_ep : Endpoints = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&blob)
            .expect("roundtrip back to struct");

    println!("Result {:?}", bin_ep);
}
