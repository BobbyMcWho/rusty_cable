#[macro_use]
extern crate helix;

ruby! {
    class RustyCable {
        def hello() {
            println!("Hello from rusty_cable!");
        }
    }
}
