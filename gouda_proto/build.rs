#![allow(clippy::expect_used)]

use std::env;

fn main() {
    let build_protos = env::var("GOUDA_BUILD_PROTOS")
        .unwrap_or_default()
        .to_lowercase()
        == "true";

    if !build_protos {
        return;
    }

    let proto_dir = "./protos";

    println!("cargo:rerun-if-changed={proto_dir}");

    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .skip_debug([
            "InitializationRequest",
            "RecoveryKeyVerificationRequest",
            "LoginUsernamePasswordRequest",
            "CrossSigningMethodSelectedEvent",
            "MessageContentText",
        ])
        .out_dir("./src/chat")
        .compile_protos(&["chat.proto"], &[proto_dir])
        .expect("Failed to compile proto files. Did you clone the submodules?");
}
