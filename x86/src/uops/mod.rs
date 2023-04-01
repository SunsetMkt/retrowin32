pub enum UOp {
    Set(u32, u32),
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
            m => unimplemented!("mnemonic {m:?}"),
        };
        f(self, instr);
    }
    fn add(&mut self, uop: UOp) {
        self.uops.push(uop);
    }

    pub fn assemble(&mut self) -> Vec<UOp> {
        Vec::new()
    }
}

fn call(asm: &mut Assembler, instr: &iced_x86::Instruction) {
    assert!(instr.op_count() == 1);
    match instr.op1_kind() {
        iced_x86::OpKind::NearBranch32 => asm.add(UOp::Set(1, instr.near_branch32())),
        _ => unimplemented!(),
    };
    asm.add(UOps::Sub(esp, 4));
    asm.add(UOps::Mov(esp));
}
