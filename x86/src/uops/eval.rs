use super::{
    asm::{MemRef, MemorySize, Reg},
    UOp,
};
use crate::{registers::Flags, Memory, X86};

/// Compute the address found in instructions that reference memory, e.g.
///   mov [eax+03h],...
fn x86_addr(x86: &X86, mem: &MemRef) -> u32 {
    // TODO: see comments on regs.fs_addr.
    let seg = if let Some(seg) = mem.seg {
        match seg {
            Reg::FS => x86.regs.fs_addr,
            _ => 0u32,
        }
    } else {
        0u32
    };

    let base = if let Some(base) = mem.base {
        x86.regs.get32(base.to_iced())
    } else {
        0
    };

    let index = if let Some(index) = mem.index {
        x86.regs
            .get32(index.to_iced())
            .wrapping_mul(mem.scale as u32)
    } else {
        0
    };

    // In general these operations aren't written to wrap, but in some cases
    // the components are negative which is implemented in two's complement by
    // a wrapping add.
    seg.wrapping_add(base)
        .wrapping_add(index)
        .wrapping_add(mem.displacement)
}

#[allow(dead_code)]
fn dump_stack(x86: &X86) {
    for i in 0..8 {
        let addr = x86.regs.esp + (i * 4);
        log::info!("{:x} {:x}", addr, x86.mem.read_u32(addr));
    }
}

pub unsafe fn eval(x86: &mut X86, ops: &[UOp]) {
    use crate::uops::asm::Arg::*;
    use UOp::*;
    let mut xc = 0u32;
    let mut x: *mut u32 = &mut xc;
    let mut yc = 0u32;
    let mut y: *mut u32 = &mut yc;
    for op in ops {
        log::info!("eval: {}", op);
        match *op {
            Comment(ref text) => {
                log::info!("; {}", text);
            }
            Const(X, c) => {
                x = &mut xc;
                *x = c
            }
            Const(Y, c) => {
                y = &mut yc;
                *y = c
            }
            GetReg(X, reg) => x = x86.regs.ptr32(reg.to_iced()),
            GetReg(Y, reg) => y = x86.regs.ptr32(reg.to_iced()),
            GetMem(X, ref mem) => {
                x = x86.mem.as_mut_ptr().offset(x86_addr(x86, mem) as isize) as *mut u32
            }
            GetMem(Y, ref mem) => {
                y = x86.mem.as_mut_ptr().offset(x86_addr(x86, mem) as isize) as *mut u32
            }
            Deref(X) => x = x86.mem.as_mut_ptr().offset(*x as isize) as *mut u32,
            Deref(Y) => y = x86.mem.as_mut_ptr().offset(*y as isize) as *mut u32,
            Add(MemorySize::U32) => *x += *y,
            And(MemorySize::U32) => *x &= *y,
            Sub(MemorySize::U32) => *x -= *y,
            Mov(MemorySize::U8) => *(x as *mut u8) = *(y as *mut u8),
            Mov(MemorySize::U32) => *x = *y,
            Xor(MemorySize::U32) => *x ^= *y,

            Jmp => x86.regs.eip = *x,
            Jb => {
                if x86.flags.contains(Flags::CF) {
                    x86.regs.eip = *x;
                }
            }
            Je => {
                if x86.flags.contains(Flags::ZF) {
                    x86.regs.eip = *x;
                }
            }
            Jne => {
                if !x86.flags.contains(Flags::ZF) {
                    x86.regs.eip = *x;
                }
            }

            Cmp(_size) => {
                let _ = x.wrapping_sub(*y as usize);
                // XXX flags
            }
            _ => todo!("op {:?}", op),
        }
    }
}
