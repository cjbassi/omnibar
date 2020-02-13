mod battery;
mod clock;
mod cpu;
mod custom;
mod disk;
mod memory;
mod pulseaudio;
mod sway;

pub use self::battery::*;
pub use clock::*;
pub use cpu::*;
pub use custom::*;
pub use disk::*;
pub use memory::*;
pub use pulseaudio::*;
pub use sway::*;
