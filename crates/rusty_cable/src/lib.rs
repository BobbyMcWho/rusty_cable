#[macro_use]
extern crate helix;
extern crate reqwest;
extern crate protobuf;

pub mod anycable;

ruby! {
    class RustyCable {
        def hello() {
            println!("Hi");
        }
    }
}
