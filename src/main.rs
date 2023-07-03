pub mod error;
pub mod frame;
pub mod instruction;
pub mod register;
pub mod value;
pub mod vm;

use instruction::Instruction::*;
use register::Register;
use vm::BoltVM;
use value::*;

fn main() {
    let mut vm = BoltVM::new();

    let program = vec![
		// array manipulation
        Push(Value::String(String::from("Hello, world!"))),
		Pop(Register(0)),
		Print(ValueOrRegister::Register(Register(0))),
		Push(Value::String(String::from("this should be still in stack"))),
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
