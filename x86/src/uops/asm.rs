#[derive(Copy, Clone, Debug)]
pub enum Reg {
    ECX,
    ESP,
    EIP,
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::ECX => f.write_str("ecx"),
            Reg::ESP => f.write_str("esp"),
            Reg::EIP => f.write_str("eip"),
        }
    }
}

#[derive(Debug)]
pub enum Arg {
    X,
    Y,
}
impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::X => f.write_str("x"),
            Arg::Y => f.write_str("y"),
        }
    }
}

#[derive(Debug)]
pub enum UOp {
    Comment(Box<str>),
    Const(Arg, u32),
    GetReg(Arg, Reg),
    GetMem(Arg),
    Deref(Arg),
    Add,
    Sub,
    Mov,
    Call,
    Cmp,
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
            UOp::Cmp => f.write_str("cmp"),
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
        self.op(UOp::Comment(format!("{}", instr).into_boxed_str()));
        let f = match instr.mnemonic() {
            iced_x86::Mnemonic::Call => mnemonic::call,
            iced_x86::Mnemonic::Mov => mnemonic::mov,
            iced_x86::Mnemonic::Push => mnemonic::push,
            iced_x86::Mnemonic::Cmp => mnemonic::todo,
            iced_x86::Mnemonic::Je => mnemonic::todo,
            iced_x86::Mnemonic::Sub => mnemonic::todo,
            iced_x86::Mnemonic::And => mnemonic::todo,
            iced_x86::Mnemonic::Lea => mnemonic::todo,
            m => unimplemented!("mnemonic {m:?}"),
        };
        f(self, instr);
    }

    fn op(&mut self, uop: UOp) {
        self.uops.push(uop);
    }

    fn operand(&mut self, instr: &iced_x86::Instruction, arg: Arg, idx: u32) {
        match instr.op_kind(idx) {
            iced_x86::OpKind::Register => self.op(UOp::GetReg(arg, Reg::ESP)),
            iced_x86::OpKind::Memory => self.op(UOp::GetMem(arg)),
            iced_x86::OpKind::Immediate32 => self.op(UOp::Const(arg, instr.immediate32())),
            k => unimplemented!("{:?}", k),
        }
    }

    pub fn assemble(&mut self) -> Vec<UOp> {
        std::mem::replace(&mut self.uops, Vec::new())
    }

    pub fn dump(&self) {
        for op in &self.uops {
            log::info!("{}", op);
        }
    }
}

mod mnemonic {
    use super::*;

    pub fn todo(asm: &mut Assembler, _instr: &iced_x86::Instruction) {
        asm.op(UOp::Comment("todo".into()));
    }

    pub fn call(asm: &mut Assembler, instr: &iced_x86::Instruction) {
        use {Arg::*, Reg::*, UOp::*};
        assert!(instr.op_count() == 1);
        asm.op(GetReg(X, ESP));
        asm.op(Const(Y, 4));
        asm.op(Sub);
        asm.op(GetReg(Y, EIP));
        asm.op(Deref(X));
        asm.op(Mov);
        match instr.op0_kind() {
            iced_x86::OpKind::NearBranch32 => asm.op(Const(X, instr.near_branch32())),
            iced_x86::OpKind::Memory => asm.operand(instr, X, 0),
            k => unimplemented!("{:?}", k),
        };
        asm.op(Call);
    }

    pub fn mov(asm: &mut Assembler, instr: &iced_x86::Instruction) {
        use Arg::*;
        assert!(instr.op_count() == 2);
        // instr.memory_size() => size of mov
        asm.operand(instr, X, 0);
        asm.operand(instr, Y, 1);
        asm.op(UOp::Mov)
    }

    pub fn push(asm: &mut Assembler, instr: &iced_x86::Instruction) {
        use {Arg::*, Reg::*, UOp::*};
        assert!(instr.op_count() == 1);
        asm.op(GetReg(X, ESP));
        asm.op(Const(Y, 4));
        asm.op(Sub);
        asm.op(Deref(X));
        asm.operand(instr, Y, 0);
        asm.op(Mov);
    }
}
