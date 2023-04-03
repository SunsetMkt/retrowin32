use super::{asm::Reg, UOp};
use crate::X86;

fn reg_to_iced(reg: Reg) -> iced_x86::Register {
    match reg {
        Reg::ECX => iced_x86::Register::ECX,
        Reg::ESP => iced_x86::Register::ESP,
        Reg::EIP => iced_x86::Register::EIP,
    }
}

#[allow(dead_code)]
pub unsafe fn eval(x86: &mut X86, ops: &[UOp]) {
    use crate::uops::asm::Arg::*;
    use UOp::*;
    let mut xc = 0u32;
    let mut x: &mut u32 = &mut xc;
    let mut yc = 0u32;
    let mut y: &mut u32 = &mut yc;
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
            &GetReg(X, reg) => x = &mut *x86.regs.ptr32(reg_to_iced(reg)),
            &GetReg(Y, reg) => y = &mut *x86.regs.ptr32(reg_to_iced(reg)),
            &GetMem(_) => todo!(),
            &Deref(_) => todo!(),
            &Add => *x += *y,
            &Sub => *x -= *y,
            &Mov => *x = *y,
            &Call => todo!(),
            &Cmp => {
                let _ = *x - *y;
            }
        }
    }
}
