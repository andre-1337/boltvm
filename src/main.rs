pub mod error;
pub mod frame;
pub mod instruction;
pub mod register;
pub mod types;
pub mod value;
pub mod vm;

use instruction::Instruction::*;
use register::Register;
use value::*;
use vm::BoltVM;

fn main() {
    let mut vm = BoltVM::new();

    let program = vec![
		LoadInt(Register(0), 1000000000),
		CopyReg(Register(1), Register(0)),
		Print(ValueOrRegister::Register(Register(0))),
		Print(ValueOrRegister::Register(Register(1))),
		Halt,
    ];

    match vm.execute(program) {
        Ok(_) => {
            print!("\n\n");
            vm.debug_dump();
        }

        Err(error) => {
            println!("{error}\nstacktrace:");
            vm.handle_error();
        }
    };
}
