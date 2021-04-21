
use serde_json;
use bincode::{Options, DefaultOptions};
use k8s_openapi::api::core::v1::Endpoints;
use k8s_openapi::api::networking::v1beta1::Ingress;
mod blobs;

fn main() {

    // Deserializes from JSON just fine
    let ing : Ingress = serde_json::from_str(blobs::INGRESS).expect("unpack ingress from json");

    // To bincode bits
    let ing_bits = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .serialize(&ing)
            .expect("serialize with bincode");

    // Deserializing from bincode fails
    let bin_ing : Ingress = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&ing_bits)
            .expect("roundtrip back to struct");

    println!("Ingress Result {:?}", bin_ing);

    // Deserializes from JSON just fine
    let ep : Endpoints = serde_json::from_str(blobs::EXAMPLE1).expect("unpack endpoint from json");

    // To bincode bits
    let endp_bits = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .serialize(&ep)
            .expect("serialize with bincode");

    // Deserializing from bincode fails
    let bin_ep : Endpoints = DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&endp_bits)
            .expect("roundtrip back to struct");

    println!("Endpoints Result {:?}", bin_ep);
}
