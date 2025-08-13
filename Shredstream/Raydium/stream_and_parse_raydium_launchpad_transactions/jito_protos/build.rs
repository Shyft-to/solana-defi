use tonic_build::configure;

fn main() {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    configure()
        .compile(
            &[
                "protos/mev-protos/auth.proto",
                "protos/mev-protos/shared.proto",
                "protos/mev-protos/shredstream.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
