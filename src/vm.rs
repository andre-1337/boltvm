use crate::instruction::Instruction;
use crate::register::Register;
use crate::value::{Value, ValueOrRegister};

/// The virtual machine implementation
#[derive(Debug)]
pub struct BoltVM {
    /// 65535 registers in total
    pub registers: Vec<Value>,
}

impl BoltVM {
    pub fn new() -> Self {
        // initialize every register to 0
        let mut regs = Vec::with_capacity(u16::MAX as usize);
        for _ in 0..u16::MAX {
            regs.push(Value::Null);
        }

        Self { registers: regs }
    }

    pub fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            match instruction {
                Instruction::LoadInt(register, value) => {
                    self.registers[register.as_index()] = Value::Int(value)
                }
                Instruction::LoadFlt(register, value) => {
                    self.registers[register.as_index()] = Value::Float(value)
                }
                Instruction::LoadStr(register, value) => {
                    self.registers[register.as_index()] = Value::String(value)
                }
                Instruction::LoadBool(register, value) => {
                    self.registers[register.as_index()] = Value::Bool(value)
                }
                Instruction::AddInt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_int(src_reg1);
                    let b = self.get_int(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Int(a + b);
                }
                Instruction::AddFlt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_float(src_reg1);
                    let b = self.get_float(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Float(a + b);
                }
                Instruction::ConcatStrings(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_string(src_reg1);
                    let b = self.get_string(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::String(format!("{}{}", a, b));
                }
                Instruction::AndBool(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_bool(src_reg1);
                    let b = self.get_bool(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a && b);
                }
                Instruction::OrBool(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_bool(src_reg1);
                    let b = self.get_bool(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a || b);
                }
                Instruction::LtInt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_int(src_reg1);
                    let b = self.get_int(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a < b);
                }
                Instruction::GtInt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_int(src_reg1);
                    let b = self.get_int(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a > b);
                }
                Instruction::LtFlt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_float(src_reg1);
                    let b = self.get_float(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a < b);
                }
                Instruction::GtFlt(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_float(src_reg1);
                    let b = self.get_float(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a > b);
                }
                Instruction::EqBool(dest_reg, src_reg1, src_reg2) => {
                    let a = self.get_bool(src_reg1);
                    let b = self.get_bool(src_reg2);
                    self.registers[dest_reg.as_index()] = Value::Bool(a == b);
                }
                Instruction::Print(reg_or_val) => {
                    match reg_or_val {
						ValueOrRegister::Value(string) => print!("{}", string),
						ValueOrRegister::Register(register) => {
							let value = &self.registers[register.as_index()];
							print!("{}", value);
						}
					}
                }
				Instruction::CreateArray(register) => {
					self.registers[register.as_index()] = Value::Array(Vec::new());
				},
				Instruction::ArrayAdd(register, value) => {
					if let Value::Array(array) = &mut self.registers[register.as_index()] {
						array.push(value);
					} else {
						panic!("Expected array type in register '{register}'");
					}
				},
				Instruction::GetArrayElemPtr(dest_reg, array_reg, index) => {
					if let Value::Array(array) = &self.registers[array_reg.as_index()] {
						if let Some(value) = array.get(index) {
							self.registers[dest_reg.as_index()] = value.clone();
						} else {
							panic!("Index out of bounds");
						}
					} else {
						panic!("Expected array type in register '{array_reg}'");
					}
				}
                Instruction::Halt => break,
            }
        }
    }

    fn get_int(&self, register: Register) -> i32 {
        match self.registers[register.as_index()] {
            Value::Int(value) => value,
            _ => panic!("Expected integer value in register"),
        }
    }

    fn get_float(&self, register: Register) -> f32 {
        match self.registers[register.as_index()] {
            Value::Float(value) => value,
            _ => panic!("Expected float value in register"),
        }
    }

    fn get_string(&self, register: Register) -> String {
        match self.registers[register.as_index()] {
            Value::String(ref value) => value.clone(),
            _ => panic!("Expected string value in register"),
        }
    }

    fn get_bool(&self, register: Register) -> bool {
        match self.registers[register.as_index()] {
            Value::Bool(value) => value,
            _ => panic!("Expected boolean value in register"),
        }
    }

    pub fn dump_register_contents(&self) {
        println!("register contents:");
        let mut used_registers = 0;

		for (i, val) in self.registers.iter().enumerate() {
			if !val.is_null() {
				println!("r{i}: {}", val);
				used_registers += 1;
			}
		}

		let total_registers = self.registers.len();
		let unused_registers = total_registers - used_registers;

		println!("\nused registers: {used_registers}");
		println!("unused registers: {unused_registers}");
    }
}
