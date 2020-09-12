fn main() {
    tonic_build::compile_protos("proto/grpcmap.proto").unwrap();
}
