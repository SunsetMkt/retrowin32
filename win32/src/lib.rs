mod host;
#[cfg(not(feature = "cpuemu"))]
mod ldt;
mod machine;
mod pe;
mod reader;
mod shims;
#[cfg(feature = "cpuemu")]
use shims_emu;

pub mod trace;
mod winapi;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "cpuemu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
