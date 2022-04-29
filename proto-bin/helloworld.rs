#[derive(test_protobuf_derive::ToAscObjMacro,test_protobuf_derive::FromAscObjMacro)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMessage {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
