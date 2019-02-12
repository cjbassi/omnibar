mod bar;
mod bindings;
mod client;
mod macros;

const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    client::init();
}
