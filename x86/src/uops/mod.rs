#[derive(Debug)]
pub enum X86Reg {
    ECX,
    ESP,
}

impl std::fmt::Display for X86Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X86Reg::ECX => f.write_str("ecx"),
            X86Reg::ESP => f.write_str("esp"),
        }
    }
}

#[derive(Debug)]
pub enum Arg {
    X,
    Y,
    Reg(X86Reg),
}
impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::X => f.write_str("x"),
            Arg::Y => f.write_str("y"),
            Arg::Reg(reg) => reg.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum UOp {
    Comment(String),
    Const(Arg, u32),
    GetReg(Arg, X86Reg),
    GetMem(Arg),
    Deref(Arg),
    Add,
    Sub,
    Mov,
    Call,
}

impl std::fmt::Display for UOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UOp::Comment(str) => f.write_fmt(format_args!("; {}", str)),
            UOp::Const(arg, c) => f.write_fmt(format_args!("{} <- {:#x}", arg, c)),
            UOp::GetReg(arg, reg) => f.write_fmt(format_args!("{} <- {}", arg, reg)),
            UOp::GetMem(arg) => f.write_fmt(format_args!("{} <- mem", arg)),
            UOp::Deref(arg) => f.write_fmt(format_args!("{} <- *{}", arg, arg)),
            UOp::Add => f.write_str("add"),
            UOp::Sub => f.write_str("sub"),
            UOp::Mov => f.write_str("mov"),
            UOp::Call => f.write_str("call"),
        }
    }
}

pub struct Assembler {
    uops: Vec<UOp>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            uops: Default::default(),
        }
    }

    pub fn add_instr(&mut self, instr: &iced_x86::Instruction) {
        log::warn!("{} {:?}", instr, instr.op0_kind());
        self.add(UOp::Comment(format!("{}", instr)));
        let f = match instr.mnemonic() {
            iced_x86::Mnemonic::Call => call,
            iced_x86::Mnemonic::Mov => mov,
            iced_x86::Mnemonic::Push => push,
            m => unimplemented!("mnemonic {m:?}"),
        };
        f(self, instr);

        for op in &self.uops {
            log::info!("{}", op);
        }
    }

    fn add(&mut self, uop: UOp) {
        self.uops.push(uop);
    }

    fn op(&mut self, instr: &iced_x86::Instruction, arg: Arg, idx: u32) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => self.add(UOp::GetReg(arg, X86Reg::ESP)),
            iced_x86::OpKind::Memory => self.add(UOp::GetMem(arg)),
            iced_x86::OpKind::Immediate32 => self.add(UOp::Const(arg, instr.immediate32())),
            k => unimplemented!("{:?}", k),
        }
    }

    pub fn assemble(&mut self) -> Vec<UOp> {
        std::mem::replace(&mut self.uops, Vec::new())
    }
}

fn call(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    use {Arg::*, UOp::*, X86Reg::*};
    assert!(instr.op_count() == 1);
    asm.add(GetReg(X, ESP));
    // XXX write eip
    asm.add(Const(Y, 4));
    asm.add(Sub);
    match instr.op0_kind() {
        iced_x86::OpKind::NearBranch32 => asm.add(Const(X, instr.near_branch32())),
        _ => unimplemented!(),
    };
    asm.add(Call);
}

fn mov(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    use Arg::*;
    assert!(instr.op_count() == 2);
    // instr.memory_size() => size of mov
    asm.op(instr, X, 0);
    asm.op(instr, Y, 1);
    asm.add(UOp::Mov)
}

fn push(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    use {Arg::*, UOp::*, X86Reg::*};
    assert!(instr.op_count() == 1);
    asm.add(GetReg(X, ESP));
    asm.add(Const(Y, 4));
    asm.add(Sub);
    asm.add(Deref(X));
    asm.op(instr, Y, 0);
    asm.add(Mov);
}
