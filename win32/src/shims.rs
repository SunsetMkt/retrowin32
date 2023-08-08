//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.

use std::io::Write;

use crate::Machine;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

pub struct Shims {
    trampolines: &'static mut [u8],
    ofs: usize,
}

impl Default for Shims {
    fn default() -> Self {
        // unsafe {
        //     println!("tramp at {:x}", retrowin32_tramp as u64);
        //     println!("trampe at {:x}", retrowin32_tramp_sz);
        // }
        Self {
            trampolines: &mut [],
            ofs: 16,
        }
    }
}

fn the_handler() {
    println!("made it back to 64-land!");
}

impl Shims {
    pub fn set_space(&mut self, addr: *mut u8, size: u32) {
        unsafe {
            self.trampolines = std::slice::from_raw_parts_mut(addr, size as usize);
        }

        let mut out = &mut *self.trampolines;
        // 16:32 selector:address of call64, which is written just below:
        out.write(&(addr as u32 + 8).to_le_bytes()).unwrap();
        out.write(&(0x2bu32).to_le_bytes()).unwrap();

        // trampoline_x86_64.s:call64:
        out.write(&[0x67, 0xff, 0x54, 0x24, 0x08, 0xcb]).unwrap();
    }

    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let call64_addr = self.trampolines.as_ptr() as u32;
        let mut out = &mut self.trampolines[self.ofs..];
        let tramp_addr = out.as_ptr() as u32;
        let target: u64 = the_handler as u64; //handler.map(|f| std::mem::transmute(f)).unwrap();
        println!("handler target {:0x}", target);

        // Code from trampoline_x86.s:

        // pushl high 32 bits of dest
        out.write(b"\x68").unwrap();
        out.write(&((target >> 32) as u32).to_le_bytes()).unwrap();
        // pushl low 32 bits of dest
        out.write(b"\x68").unwrap();
        out.write(&(target as u32).to_le_bytes()).unwrap();

        // lcalll *call64_addr
        out.write(b"\xff\x1d").unwrap();
        out.write(&call64_addr.to_le_bytes()).unwrap();

        // addl $0x08, %esp
        out.write(b"\x83\xc4\x08").unwrap();

        // retl $20, %esp
        out.write(b"\xc2\x20\x00").unwrap();

        println!(
            "registered {} at {:x} {:x?}",
            name,
            tramp_addr,
            &self.trampolines[self.ofs..self.ofs + 32]
        );

        self.ofs += 0x20;
        println!("registered {} at {:x}", name, tramp_addr);
        0x2010
    }
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub fn async_call(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
    todo!()
}
