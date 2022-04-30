#[derive(test_protobuf_derive::ToAscObjMacro)]
#[ToAscObj(String)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMessage {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
