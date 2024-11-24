mod nop;    pub use nop::Nop;
mod add;    pub use add::Add;
mod sub;    pub use sub::Sub;
mod addi;   pub use addi::Addi;
mod subi;   pub use subi::Subi;
mod beq;    pub use beq::Beq;
mod bne;    pub use bne::Bne;
mod blt;    pub use blt::Blt;
mod ble;    pub use ble::Ble;
mod jal;    pub use jal::Jal;
mod lw;     pub use lw::Lw;
mod lh;     pub use lh::Lh;
mod lb;     pub use lb::Lb;
mod lhu;    pub use lhu::Lhu;
mod lbu;    pub use lbu::Lbu;
mod sw;     pub use sw::Sw;
mod sh;     pub use sh::Sh;
mod sb;     pub use sb::Sb;
mod r#in;   pub use r#in::In;
mod out;    pub use out::Out;
mod and;    pub use and::And;
mod or;     pub use or::Or;
mod xor;    pub use xor::Xor;
mod srl;    pub use srl::Srl;
mod sra;    pub use sra::Sra;
mod sll;    pub use sll::Sll;
mod andi;   pub use andi::Andi;
mod ori;    pub use ori::Ori;
mod xori;   pub use xori::Xori;
mod srli;   pub use srli::Srli;
mod srai;   pub use srai::Srai;
mod slli;   pub use slli::Slli;

use sb_emu_state::State;

pub trait Inst {
    fn exec(&self, state: State) -> anyhow::Result<State>;
}
