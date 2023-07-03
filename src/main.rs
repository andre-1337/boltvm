pub mod error;
pub mod frame;
pub mod instruction;
pub mod register;
pub mod value;
pub mod vm;

use instruction::Instruction::*;
use register::Register;
use vm::BoltVM;

fn main() {
    let mut vm = BoltVM::new();

    let program = vec![
        // errors!
		LoadFlt(Register(1), 1.1),
		LoadFlt(Register(2), 0.0),
		DivFlt(Register(3), Register(1), Register(2)),
        Halt,
    ];

    match vm.execute(program) {
        Ok(_) => {
            print!("\n\n");
            vm.dump_register_contents();
        }

        Err(error) => {
			println!("{error}\nstacktrace:");
            vm.handle_error();
        }
    };
}
