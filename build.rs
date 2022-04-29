
//https://github.com/imbolc/rust-derive-macro-guide
//https://stackoverflow.com/questions/56188700/how-do-i-make-my-custom-derive-macro-accept-trait-generic-parameters

fn main(){
    
    tonic_build::configure()
        .type_attribute("helloworld.TestMessage", "#[derive(test_protobuf_derive::ToAscObjMacro,test_protobuf_derive::FromAscObjMacro)]")
        //.file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .out_dir("proto-bin") 
        .compile(&["proto/helloworld.proto"], &["proto"])
        .expect("Failed to compile protobuf definitions");

}