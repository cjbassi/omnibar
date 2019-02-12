mod bar;
mod bindings;
mod client;

const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    client::init();
}
