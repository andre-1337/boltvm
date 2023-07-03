use std::vec;

use crate::error::*;
use crate::frame::*;
use crate::instruction::Instruction;
use crate::register::Register;
use crate::value::{Value, ValueOrRegister};

use backtrace::Backtrace;

/// The virtual machine implementation
#[derive(Debug)]
pub struct BoltVM {
    /// 65535 registers in total
    pub registers: Vec<Value>,
    pub frames: Vec<StackFrame>,
}

impl BoltVM {
    pub fn new() -> Self {
        // initialize every register to 0
        let mut regs = Vec::with_capacity(u16::MAX as usize);
        for _ in 0..u16::MAX {
            regs.push(Value::Null);
        }

        Self {
            registers: regs,
            frames: vec![],
        }
    }

    fn format_stack_frame(&mut self) -> String {
        let mut result = String::new();

        for frame in self.frames.iter() {
            if frame.file_name.starts_with("boltvm") {
                result.push_str(&format!(
                    "	in '{}' at '{}', line {}\n",
                    frame.file_name, frame.function_name, frame.line
                ));
            }
        }

        result
    }

    pub fn handle_error(&mut self) {
        let backtrace = Backtrace::new();
        let frames = backtrace.frames();

        let mut converted_frames = vec![];
        for frame in frames {
            if let Some(symbol) = frame.symbols().get(0) {
                if let Some(function_name) = symbol.name() {
                    if let Some(file) = symbol.filename() {
                        if let Some(line) = symbol.lineno() {
                            converted_frames.push(StackFrame {
                                file_name: file.file_name().unwrap().to_str().unwrap().to_owned(),
                                function_name: function_name.to_string(),
                                line,
                            });
                        }
                    }
                }
            }
        }

        self.frames.extend(converted_frames);
        let stack_trace = self.format_stack_frame();
        if stack_trace.is_empty() {
            println!("	no stacktrace");
        } else {
            println!("{stack_trace}");
        }

        self.frames.clear();
    }

    pub fn execute(&mut self, program: Vec<Instruction>) -> Result<()> {
        self.frames.push(StackFrame {
            file_name: String::from("boltvm"),
            function_name: String::from("execute()"),
            line: 80,
        });

        for instruction in program {
            match instruction {
                Instruction::LoadInt(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadInt"),
                        line: 80,
                    });

                    self.registers[register.as_index()] = Value::Int(value)
                }

                Instruction::LoadFlt(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadFlt"),
                        line: 99,
                    });

                    self.registers[register.as_index()] = Value::Float(value)
                }

                Instruction::LoadStr(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadStr"),
                        line: 109,
                    });

                    self.registers[register.as_index()] = Value::String(value)
                }

                Instruction::LoadBool(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadBool"),
                        line: 119,
                    });

                    self.registers[register.as_index()] = Value::Bool(value)
                }

                Instruction::AddInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AddInt"),
                        line: 129,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Int(a + b);
                }

                Instruction::AddFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AddFlt"),
                        line: 141,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Float(a + b);
                }

                Instruction::SubInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::SubInt"),
                        line: 153,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Int(a - b);
                }

                Instruction::SubFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::SubFlt"),
                        line: 165,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Float(a - b);
                }

                Instruction::MulInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::MulInt"),
                        line: 177,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Int(a * b);
                }

                Instruction::MulFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::MulFlt"),
                        line: 189,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Float(a * b);
                }

                Instruction::DivInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::DivInt"),
                        line: 201,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();

                    if b == 0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        self.registers[dest_reg.as_index()] = Value::Int(a / b);
                    }
                }

                Instruction::DivFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::DivFlt"),
                        line: 218,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();

                    if b == 0.0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        self.registers[dest_reg.as_index()] = Value::Float(a / b);
                    }
                }

                Instruction::ConcatStrings(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::ConcatStrings"),
                        line: 235,
                    });

                    let a = self.get_string(src_reg1).unwrap();
                    let b = self.get_string(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::String(format!("{}{}", a, b));
                }

                Instruction::AndBool(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AndBool"),
                        line: 247,
                    });

                    let a = self.get_bool(src_reg1).unwrap();
                    let b = self.get_bool(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a && b);
                }

                Instruction::OrBool(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::OrBool"),
                        line: 259,
                    });

                    let a = self.get_bool(src_reg1).unwrap();
                    let b = self.get_bool(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a || b);
                }

                Instruction::LtInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LtInt"),
                        line: 271,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a < b);
                }

                Instruction::GtInt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GtInt"),
                        line: 283,
                    });

                    let a = self.get_int(src_reg1).unwrap();
                    let b = self.get_int(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a > b);
                }

                Instruction::LtFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LtFlt"),
                        line: 295,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a < b);
                }

                Instruction::GtFlt(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GtFlt"),
                        line: 307,
                    });

                    let a = self.get_float(src_reg1).unwrap();
                    let b = self.get_float(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a > b);
                }

                Instruction::EqBool(dest_reg, src_reg1, src_reg2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::EqBool"),
                        line: 319,
                    });

                    let a = self.get_bool(src_reg1).unwrap();
                    let b = self.get_bool(src_reg2).unwrap();
                    self.registers[dest_reg.as_index()] = Value::Bool(a == b);
                }

                Instruction::Print(reg_or_val) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::Print"),
                        line: 331,
                    });

                    match reg_or_val {
                        ValueOrRegister::Value(string) => print!("{}", string),
                        ValueOrRegister::Register(register) => {
                            let value = &self.registers[register.as_index()];
                            print!("{}", value);
                        }
                    }
                }

                Instruction::CreateArray(register) => {
					self.frames.push(StackFrame {
						file_name: String::from("boltvm"),
						function_name: String::from("Instruction::CreateArray"),
						line: 347
					});

                    self.registers[register.as_index()] = Value::Array(Vec::new());
                }

                Instruction::ArrayAdd(register, value) => {
					self.frames.push(StackFrame {
						file_name: String::from("boltvm"),
						function_name: String::from("Instruction::ArrayAdd"),
						line: 357
					});

                    if let Value::Array(array) = &mut self.registers[register.as_index()] {
                        array.push(value);
                    } else {
                        return Err(Error::ExpectedType("array", register));
                    }
                }

                Instruction::GetArrayElemPtr(dest_reg, array_reg, index) => {
					self.frames.push(StackFrame {
						file_name: String::from("boltvm"),
						function_name: String::from("Instruction::GetArrayElemPtr"),
						line: 371
					});

                    if let Value::Array(array) = &self.registers[array_reg.as_index()] {
                        if let Some(value) = array.get(index) {
                            self.registers[dest_reg.as_index()] = value.clone();
                        } else {
                            return Err(Error::ArrayIndexOutOfBounds(index));
                        }
                    } else {
                        return Err(Error::ExpectedType("array", array_reg));
                    }
                }

                Instruction::Halt => break,
            }
        }

        Ok(())
    }

    #[inline]
    fn get_int(&self, register: Register) -> Result<i32> {
        match self.registers[register.as_index()] {
            Value::Int(value) => Ok(value),
            _ => Err(Error::ExpectedType("int", register)),
        }
    }

    #[inline]
    fn get_float(&self, register: Register) -> Result<f32> {
        match self.registers[register.as_index()] {
            Value::Float(value) => Ok(value),
            _ => Err(Error::ExpectedType("float", register)),
        }
    }

    #[inline]
    fn get_string(&self, register: Register) -> Result<String> {
        match self.registers[register.as_index()] {
            Value::String(ref value) => Ok(value.clone()),
            _ => Err(Error::ExpectedType("string", register)),
        }
    }

    #[inline]
    fn get_bool(&self, register: Register) -> Result<bool> {
        match self.registers[register.as_index()] {
            Value::Bool(value) => Ok(value),
            _ => Err(Error::ExpectedType("bool", register)),
        }
    }

    pub fn dump_register_contents(&self) {
        println!("register contents:");
        let mut used_registers = 0;

        for (i, val) in self.registers.iter().enumerate() {
            if !val.is_null() {
                println!("{}r{i}{}: {}", Color::Blue, Color::Reset, val);
                used_registers += 1;
            }
        }

        let total_registers = self.registers.len();
        let unused_registers = total_registers - used_registers;

        println!(
            "\nused registers: {}{used_registers}{}",
            Color::Green,
            Color::Reset
        );
        println!(
            "unused registers: {}{unused_registers}{}",
            Color::Yellow,
            Color::Reset
        );
    }
}
