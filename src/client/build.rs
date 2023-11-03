fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["../../proto/zkp_auth.proto"], &["../../proto"])
        .unwrap();
}
