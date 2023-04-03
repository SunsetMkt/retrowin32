use super::{asm::MemRef, UOp};
use crate::X86;

/// Compute the address found in instructions that reference memory, e.g.
///   mov [eax+03h],...
fn x86_addr(x86: &X86, mem: &MemRef) -> u32 {
    // TODO: see comments on regs.fs_addr.
    let seg = if let Some(seg) = mem.seg {
        todo!("seg {:?}", seg);
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
pub unsafe fn eval(x86: &mut X86, ops: &[UOp]) {
    use crate::uops::asm::Arg::*;
    use UOp::*;
    let mut xc = 0u32;
    let mut x: *mut u32 = &mut xc;
    let mut yc = 0u32;
    let mut y: *mut u32 = &mut yc;
    for op in ops {
        match op {
            &Comment(_) => {}
            &Const(X, c) => {
                x = &mut xc;
                *x = c
            }
            &Const(Y, c) => {
                y = &mut yc;
                *y = c
            }
            &GetReg(X, reg) => x = x86.regs.ptr32(reg.to_iced()),
            &GetReg(Y, reg) => y = x86.regs.ptr32(reg.to_iced()),
            &GetMem(X, ref mem) => {
                x = x86.mem.as_mut_ptr().offset(x86_addr(x86, mem) as isize) as *mut u32
            }
            &GetMem(Y, ref mem) => {
                y = x86.mem.as_mut_ptr().offset(x86_addr(x86, mem) as isize) as *mut u32
            }
            &Deref(X) => x = *x as *mut u32,
            &Deref(Y) => y = *y as *mut u32,
            &Add => *x += *y,
            &And => todo!(),
            &Sub => *x -= *y,
            &Mov => *x = *y,
            &Call => todo!(),
            &Cmp => {
                let _ = *x - *y;
            }
        }
    }
}
