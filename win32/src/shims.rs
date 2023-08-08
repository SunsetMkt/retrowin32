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
        // Code from trampoline_x86_64.s:
        let mut out = &mut *self.trampolines;
        out.write(&[0x67, 0xff, 0x54, 0x24, 0x08, 0xcb]).unwrap();
        println!(
            "call64 at {:x}: {:x?}",
            self.trampolines.as_ptr() as u64,
            &self.trampolines[..6]
        );
    }

    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let call64_addr = self.trampolines.as_ptr() as u32;
        let mut out = &mut self.trampolines[self.ofs..];
        let tramp_addr = out.as_ptr() as u32;
        let target: u64 = the_handler as u64; //handler.map(|f| std::mem::transmute(f)).unwrap();
        println!("handler target {:0x}", target);

        // Code from trampoline_x86.s:

        // pushl segment selector for 64-bit mode
        //out.write(b"\x68\x07\x01\x00\x00").unwrap();
        out.write(b"\x6a\x2b").unwrap();
        // pushl call64 trampoline
        out.write(b"\x68").unwrap();
        out.write(&call64_addr.to_le_bytes()).unwrap();

        // pushl high 32 bits of dest
        out.write(b"\x68").unwrap();
        out.write(&((target >> 32) as u32).to_le_bytes()).unwrap();
        // pushl low 32 bits of dest
        out.write(b"\x68").unwrap();
        out.write(&(target as u32).to_le_bytes()).unwrap();

        // lcalll *8(%esp)
        out.write(b"\xff\x5c\x24\x08").unwrap();

        // addl $0x10, %esp
        out.write(b"\x83\xc4\x10").unwrap();

        // retl $20, %esp
        out.write(b"\xc3").unwrap();

        println!(
            "registered {} at {:x} {:x?}",
            name,
            tramp_addr,
            &self.trampolines[self.ofs..self.ofs + 32]
        );

        self.ofs += 32;
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
