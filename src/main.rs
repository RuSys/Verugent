#[macro_use]
extern crate verugent;
//use verugent::vcore::*;

mod led;
mod fifo;
mod uart;

fn main() {
    
    led::led();
    fifo::fifo();
    uart::uart();
}
