#[derive(Debug)]
pub enum Reg {
    ECX,
    ESP,
}

#[derive(Debug)]
pub enum Arg {
    X,
    Y,
    Reg(Reg),
}

#[derive(Debug)]
pub enum UOp {
    Const(Arg, u32),
    Reg(Arg, Reg),
    Mem(Arg),
    Deref(Arg),
    Add,
    Sub,
    Mov,
    Call,
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
        log::info!("{} {:?}", instr, instr.op0_kind());

        let f = match instr.mnemonic() {
            iced_x86::Mnemonic::Call => call,
            iced_x86::Mnemonic::Mov => mov,
            iced_x86::Mnemonic::Push => push,
            m => unimplemented!("mnemonic {m:?}"),
        };
        f(self, instr);
        log::info!("{:#?}", self.assemble())
    }
    fn add(&mut self, uop: UOp) {
        self.uops.push(uop);
    }

    pub fn assemble(&mut self) -> Vec<UOp> {
        std::mem::replace(&mut self.uops, Vec::new())
    }
}

fn op(asm: &mut Assembler, instr: &iced_x86::Instruction, idx: u32) {
    match instr.op_kind(idx) {
        iced_x86::OpKind::Register => asm.add(UOp::Reg(Arg::X, Reg::ESP)),
        iced_x86::OpKind::Memory => asm.add(UOp::Mem(Arg::X)),
        iced_x86::OpKind::Immediate32 => asm.add(UOp::Const(Arg::X, instr.immediate32())),
        k => unimplemented!("{:?}", k),
    }
}

fn call(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    assert!(instr.op_count() == 1);
    asm.add(UOp::Reg(Arg::X, Reg::ESP));
    // XXX write eip
    asm.add(UOp::Const(Arg::Y, 4));
    asm.add(UOp::Sub);
    match instr.op0_kind() {
        iced_x86::OpKind::NearBranch32 => asm.add(UOp::Const(Arg::X, instr.near_branch32())),
        _ => unimplemented!(),
    };
    asm.add(UOp::Call);
}

fn mov(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    assert!(instr.op_count() == 2);
    // instr.memory_size() => size of mov
    op(asm, instr, 0);
    op(asm, instr, 1);
    asm.add(UOp::Mov)
}

fn push(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    assert!(instr.op_count() == 1);
    asm.add(UOp::Reg(Arg::X, Reg::ESP));
    asm.add(UOp::Const(Arg::Y, 4));
    asm.add(UOp::Sub);
    asm.add(UOp::Deref(Arg::X));
    op(asm, instr, 0);
    asm.add(UOp::Mov);
}
