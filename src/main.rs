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
		CreateArray(Register(0)),
		ArrayAdd(Register(0), Value::Int(1)),
		ArrayAdd(Register(0), Value::Int(2)),
		ArrayAdd(Register(0), Value::Int(3)),
		ArrayAdd(Register(0), Value::Int(4)),
		Print(ValueOrRegister::Register(Register(0))),
        Halt,
    ];

    vm.execute(program);
    print!("\n\n");
    vm.dump_register_contents();
}
