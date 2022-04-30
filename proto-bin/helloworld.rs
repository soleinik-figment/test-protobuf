#[derive(test_protobuf_derive::TestMacro)]
#[ToAscObj(String)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMessage {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
