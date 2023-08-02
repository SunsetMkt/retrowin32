use crate::{host, pe, shims::Shims, winapi};
use memory::{Mem, VecMem};
use std::collections::HashMap;
#[cfg(feature = "cpuemu")]
use x86::X86;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct Machine {
    #[cfg(feature = "cpuemu")]
    pub x86: x86::X86,
    pub memory: VecMem,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Machine {
            #[cfg(feature = "cpuemu")]
            x86: x86::X86::new(),
            memory: VecMem::default(),
            host,
            state: winapi::State::new(),
            shims: Shims::default(),
            labels: HashMap::new(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn load_exe(&mut self, buf: &[u8], cmdline: String, relocate: bool) -> anyhow::Result<()> {
        pe::load_exe(self, buf, cmdline, relocate)
    }

    /// If eip points at a shim address, call the handler and update eip.
    #[cfg(feature = "cpuemu")]
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(false);
        }
        let handler = self
            .shims
            .get(self.x86.cpu.regs.eip)
            .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
        handler(self);
        // Handler will have set eip to the return address from the stack.
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    #[cfg(feature = "cpuemu")]
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    #[cfg(feature = "cpuemu")]
    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.x86
            .single_step(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }
}
