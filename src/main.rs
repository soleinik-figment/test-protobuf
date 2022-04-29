extern crate test_protobuf_derive;

use hello_world::TestMessage;
use runtime::{AscSomething, ToAscObj};
use test_protobuf_derive::{FromAscObjMacro, ToAscObjMacro};


pub mod hello_world {
    include!(concat!("../proto-bin", "/helloworld.rs"));
}





impl ToAscObj<dyn AscSomething> for TestMessage{

}
// #[derive(ToAscObjMacro)]
// struct Z{

// }
fn main(){
    println!("Hello world!");
}