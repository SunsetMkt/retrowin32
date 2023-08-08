//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.

use crate::Machine;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

struct StaticStack {
    ptr: *mut u8,
    len: usize,
    ofs: usize,
}

impl Default for StaticStack {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            ofs: 0,
        }
    }
}

impl StaticStack {
    fn new(ptr: *mut u8, len: usize) -> Self {
        StaticStack { ptr, len, ofs: 0 }
    }

    unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        let ptr = self.ptr.add(self.ofs);
        self.ofs += size;
        if self.ofs > self.len {
            panic!("overflow");
        }
        ptr
    }
    unsafe fn pop(&mut self, size: usize) {
        self.ofs -= size;
    }

    fn cur_ptr(&self) -> *mut u8 {
        unsafe { self.ptr.add(self.ofs) }
    }

    fn realign(&mut self) {
        let align = 8;
        self.ofs = self.ofs + (8 - 1) & !(8 - 1);
        if self.ofs > self.len {
            panic!("overflow");
        }
    }

    unsafe fn write(&mut self, buf: &[u8]) -> *mut u8 {
        let ptr = self.cur_ptr();
        std::ptr::copy_nonoverlapping(buf.as_ptr(), ptr, buf.len());
        self.ofs += buf.len();
        if self.ofs > self.len {
            panic!("overflow");
        }
        ptr
    }
}

pub struct Shims {
    buf: StaticStack,
    call64_addr: u32,
}

fn the_handler() {
    println!("made it back to 64-land!");
}

impl Shims {
    pub fn new(addr: *mut u8, size: u32) -> Self {
        unsafe {
            let mut buf = StaticStack::new(addr, size as usize);

            // trampoline_x86_64.s:call64:
            let call64 = buf.write(&[0x67, 0xff, 0x54, 0x24, 0x08, 0xcb]);
            buf.realign();

            // 16:32 selector:address of call64, which is written just below:
            let call64_addr = buf.write(&(call64 as u32).to_le_bytes()) as u32;
            buf.write(&(0x2bu32).to_le_bytes());
            buf.realign();

            Shims { buf, call64_addr }
        }
    }

    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        unsafe {
            let tramp_addr = self.buf.cur_ptr() as u32;
            let target: u64 = the_handler as u64; //handler.map(|f| std::mem::transmute(f));
            println!("tramp {:x} handler target {:0x}", tramp_addr, target);

            // Code from trampoline_x86.s:

            // pushl high 32 bits of dest
            self.buf.write(b"\x68");
            self.buf.write(&((target >> 32) as u32).to_le_bytes());
            // pushl low 32 bits of dest
            self.buf.write(b"\x68");
            self.buf.write(&(target as u32).to_le_bytes());

            // lcalll *call64_addr
            self.buf.write(b"\xff\x1d");
            self.buf.write(&self.call64_addr.to_le_bytes());

            // addl $0x08, %esp
            self.buf.write(b"\x83\xc4\x08");

            // retl $20, %esp
            self.buf.write(b"\xc2\x20\x00");
            self.buf.realign();

            println!("registered {} at {:x}", name, tramp_addr,);

            0x2010
        }
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
