#[derive(Copy, Clone, Debug)]
pub enum Reg {
    EAX,
    ECX,
    EDX,
    EBX,
    ESI,
    EDI,
    ESP,
    EBP,
    EIP,
}

impl Reg {
    fn from_iced(r: iced_x86::Register) -> Option<Self> {
        Some(match r {
            iced_x86::Register::None => return None,
            iced_x86::Register::EAX => Reg::EAX,
            iced_x86::Register::ECX => Reg::ECX,
            iced_x86::Register::EDX => Reg::EDX,
            iced_x86::Register::EBX => Reg::EBX,
            iced_x86::Register::ESI => Reg::ESI,
            iced_x86::Register::EDI => Reg::EDI,
            iced_x86::Register::ESP => Reg::ESP,
            iced_x86::Register::EBP => Reg::EBP,
            iced_x86::Register::EIP => Reg::EIP,
            _ => unimplemented!("{:?}", r),
        })
    }

    pub fn to_iced(&self) -> iced_x86::Register {
        match self {
            Reg::EAX => iced_x86::Register::EAX,
            Reg::ECX => iced_x86::Register::ECX,
            Reg::EDX => iced_x86::Register::EDX,
            Reg::EBX => iced_x86::Register::EBX,
            Reg::ESI => iced_x86::Register::ESI,
            Reg::EDI => iced_x86::Register::EDI,
            Reg::ESP => iced_x86::Register::ESP,
            Reg::EBP => iced_x86::Register::EBP,
            Reg::EIP => iced_x86::Register::EIP,
        }
    }
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::EAX => f.write_str("eax"),
            Reg::ECX => f.write_str("ecx"),
            Reg::EDX => f.write_str("edx"),
            Reg::EBX => f.write_str("ebx"),
            Reg::ESI => f.write_str("esi"),
            Reg::EDI => f.write_str("edi"),
            Reg::ESP => f.write_str("esp"),
            Reg::EBP => f.write_str("ebp"),
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
    GetMem(Arg, MemRef),
    Deref(Arg),
    Add,
    And,
    Sub,
    Mov,
    Call,
    Cmp,
}

#[derive(Debug)]
pub struct MemRef {
    pub seg: Option<Reg>,
    pub base: Option<Reg>,
    pub index: Option<Reg>,
    pub scale: u8,
    pub displacement: u32,
}

impl std::fmt::Display for MemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(seg) = self.seg {
            f.write_fmt(format_args!("{}:", seg))?;
        }
        f.write_str("[")?;
        let mut wrote = false;
        if let Some(base) = self.base {
            base.fmt(f)?;
            wrote = true;
        }
        if let Some(index) = self.index {
            if wrote {
                f.write_str("+")?;
            }
            f.write_fmt(format_args!("{}*{}", index, self.displacement))?;
            wrote = true;
        }
        if self.displacement > 0 {
            if wrote {
                f.write_str("+")?;
            }
            f.write_fmt(format_args!("{:#x}", self.displacement))?;
        }
        f.write_str("]")
    }
}

impl std::fmt::Display for UOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UOp::Comment(str) => f.write_fmt(format_args!("; {}", str)),
            UOp::Const(arg, c) => f.write_fmt(format_args!("{} -> {:#x}", arg, c)),
            UOp::GetReg(arg, reg) => f.write_fmt(format_args!("{} -> {}", arg, reg)),
            UOp::GetMem(arg, mem) => f.write_fmt(format_args!("{} -> {}", arg, mem)),
            UOp::Deref(arg) => f.write_fmt(format_args!("{} -> *{}", arg, arg)),
            UOp::Add => f.write_str("add"),
            UOp::And => f.write_str("and"),
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
            iced_x86::Mnemonic::And => mnemonic::and,
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
            iced_x86::OpKind::Register => self.op(UOp::GetReg(
                arg,
                Reg::from_iced(instr.op_register(idx)).unwrap(),
            )),
            iced_x86::OpKind::Memory => {
                let mem = MemRef {
                    seg: Reg::from_iced(instr.segment_prefix()),
                    base: Reg::from_iced(instr.memory_base()),
                    index: Reg::from_iced(instr.memory_index()),
                    scale: instr.memory_index_scale() as u8,
                    displacement: instr.memory_displacement32(),
                };
                self.op(UOp::GetMem(arg, mem))
            }
            iced_x86::OpKind::Immediate32 => self.op(UOp::Const(arg, instr.immediate32())),
            iced_x86::OpKind::Immediate8to32 => {
                self.op(UOp::Const(arg, instr.immediate8to32() as u32))
            }
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

    pub fn and(asm: &mut Assembler, instr: &iced_x86::Instruction) {
        use Arg::*;
        assert!(instr.op_count() == 2);
        // instr.memory_size() => size of and
        asm.operand(instr, X, 0);
        asm.operand(instr, Y, 1);
        asm.op(UOp::And)
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
