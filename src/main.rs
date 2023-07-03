pub mod error;
pub mod instruction;
pub mod register;
pub mod value;
pub mod vm;

use instruction::Instruction::*;
use register::Register;
use value::*;
use vm::BoltVM;

fn main() {
    let mut vm = BoltVM::new();

    let program = vec![
		// array manipulation
        CreateArray(Register(0)),
        ArrayAdd(Register(0), Value::Int(1)),
        ArrayAdd(Register(0), Value::Int(2)),
        ArrayAdd(Register(0), Value::Int(3)),
        ArrayAdd(Register(0), Value::Int(4)),
        Print(ValueOrRegister::Register(Register(0))),

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
		},

		Err(error) => {
			eprintln!("{error}");
			return;
		},
	};
}
