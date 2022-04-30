extern crate test_protobuf_derive;

use hello_world::TestMessage;
use runtime::{AscSomething, ToAscObj};
use test_protobuf_derive::{FromAscObjMacro, ToAscObjMacro, TestMacro};


pub mod hello_world {
    include!(concat!("../proto-bin", "/helloworld.rs"));
}

// #[derive(ToAscObjMacro)]
// #[my_trait('a, B, C)]
#[derive(TestMacro)]
#[ToAscObj(String)]
pub struct MyTestMessage;

#[derive(TestMacro)]
#[ToAscObj(i32)]
pub struct MyTestMessageAnother;

// impl ToAscObj<dyn AscSomething> for TestMessage{

// }
// #[derive(ToAscObjMacro)]
// struct Z{

// }
fn main(){

    let t = MyTestMessage;
    t.to();

    let t = MyTestMessageAnother;
    t.to();

    let t = TestMessage{name:"aaaa".to_string()};
    t.to();
}