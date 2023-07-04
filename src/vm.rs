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
    pub stack: Vec<Value>,
    pub frames: Vec<StackFrame>,
    pub instruction_pointer: usize,
}

impl BoltVM {
    pub fn new() -> Self {
        // initialize every register to 0
        let mut regs = Vec::with_capacity(65535);
        for _ in 0..regs.capacity() {
            regs.push(Value::Null);
        }

        Self {
            registers: regs,
            stack: vec![],
            frames: vec![],
            instruction_pointer: 0,
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

    fn increment_ip(&mut self) {
        self.instruction_pointer += 1;
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

                    self.registers[register.as_index()] = Value::Int(value);
                    self.increment_ip();
                }

                Instruction::LoadFlt(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadFlt"),
                        line: 99,
                    });

                    self.registers[register.as_index()] = Value::Float(value);
                    self.increment_ip();
                }

                Instruction::LoadStr(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadStr"),
                        line: 109,
                    });

                    self.registers[register.as_index()] = Value::String(value);
                    self.increment_ip();
                }

                Instruction::LoadBool(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LoadBool"),
                        line: 119,
                    });

                    self.registers[register.as_index()] = Value::Bool(value);
                    self.increment_ip();
                }

                Instruction::AddInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AddInt"),
                        line: 129,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Int(a + b);
                    self.increment_ip();
                }

                Instruction::AddFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AddFlt"),
                        line: 141,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Float(a + b);
                    self.increment_ip();
                }

                Instruction::SubInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::SubInt"),
                        line: 153,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Int(a - b);
                    self.increment_ip();
                }

                Instruction::SubFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::SubFlt"),
                        line: 165,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Float(a - b);
                    self.increment_ip();
                }

                Instruction::MulInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::MulInt"),
                        line: 177,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Int(a * b);
                    self.increment_ip();
                }

                Instruction::MulFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::MulFlt"),
                        line: 189,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Float(a * b);
                    self.increment_ip();
                }

                Instruction::DivInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::DivInt"),
                        line: 201,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    if b == 0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        self.registers[destination_register.as_index()] = Value::Int(a / b);
                    }

                    self.increment_ip();
                }

                Instruction::DivFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::DivFlt"),
                        line: 218,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    if b == 0.0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        self.registers[destination_register.as_index()] = Value::Float(a / b);
                    }

                    self.increment_ip();
                }

                Instruction::ConcatStrings(
                    destination_register,
                    source_register1,
                    source_register2,
                ) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::ConcatStrings"),
                        line: 235,
                    });

                    let a = self.get_string(source_register1).unwrap();
                    let b = self.get_string(source_register2).unwrap();
                    let result = Value::String(format!("{a}{b}"));

                    self.registers[destination_register.as_index()] = result;
                    self.increment_ip();
                }

                Instruction::AndBool(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::AndBool"),
                        line: 247,
                    });

                    let a = self.get_bool(source_register1).unwrap();
                    let b = self.get_bool(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a && b);
                    self.increment_ip();
                }

                Instruction::OrBool(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::OrBool"),
                        line: 259,
                    });

                    let a = self.get_bool(source_register1).unwrap();
                    let b = self.get_bool(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a || b);
                    self.increment_ip();
                }

                Instruction::LtInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LtInt"),
                        line: 271,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a < b);
                    self.increment_ip();
                }

                Instruction::GtInt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GtInt"),
                        line: 283,
                    });

                    let a = self.get_int(source_register1).unwrap();
                    let b = self.get_int(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a > b);
                    self.increment_ip();
                }

                Instruction::LtFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::LtFlt"),
                        line: 295,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a < b);
                    self.increment_ip();
                }

                Instruction::GtFlt(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GtFlt"),
                        line: 307,
                    });

                    let a = self.get_float(source_register1).unwrap();
                    let b = self.get_float(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a > b);
                    self.increment_ip();
                }

                Instruction::EqBool(destination_register, source_register1, source_register2) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::EqBool"),
                        line: 319,
                    });

                    let a = self.get_bool(source_register1).unwrap();
                    let b = self.get_bool(source_register2).unwrap();

                    self.registers[destination_register.as_index()] = Value::Bool(a == b);
                    self.increment_ip();
                }

                Instruction::Print(value_or_register) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::Print"),
                        line: 331,
                    });

                    match value_or_register {
                        ValueOrRegister::Value(string) => print!("{}", string),
                        ValueOrRegister::Register(register) => {
                            let value = &self.registers[register.as_index()];
                            print!("{}", value);
                        }
                    }

                    self.increment_ip();
                }

                Instruction::CreateArray(register) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::CreateArray"),
                        line: 347,
                    });

                    self.registers[register.as_index()] = Value::Array(Vec::new());
                    self.increment_ip();
                }

                Instruction::ArrayAdd(register, value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::ArrayAdd"),
                        line: 357,
                    });

                    if let Value::Array(array) = &mut self.registers[register.as_index()] {
                        array.push(value);
                    } else {
                        return Err(Error::ExpectedType(String::from("array"), register));
                    }

                    self.increment_ip();
                }

                Instruction::GetArrayElemPtr(destination_register, array_register, index) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GetArrayElemPtr"),
                        line: 371,
                    });

                    if let Value::Array(array) = &self.registers[array_register.as_index()] {
                        if let Some(value) = array.get(index) {
                            self.registers[destination_register.as_index()] = value.clone();
                        } else {
                            return Err(Error::ArrayIndexOutOfBounds(index));
                        }
                    } else {
                        return Err(Error::ExpectedType(String::from("array"), array_register));
                    }

                    self.increment_ip();
                }

                Instruction::GetArrayLength(array_register, destination_register) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::GetArrayLength"),
                        line: 396,
                    });

                    if let Value::Array(array) = &self.registers[array_register.as_index()] {
                        let length = array.len();
                        self.registers[destination_register.as_index()] = Value::Int(length as i32);
                    } else {
                        return Err(Error::ExpectedType(String::from("array"), array_register));
                    }

                    self.increment_ip();
                }

                Instruction::Push(value) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::Push"),
                        line: 411,
                    });

                    self.stack.push(value);
                    self.increment_ip();
                }

                Instruction::Pop(register) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::Pop"),
                        line: 421,
                    });

                    if let Some(value) = self.stack.pop() {
                        self.registers[register.as_index()] = value;
                    } else {
                        return Err(Error::StackUnderflow);
                    }

                    self.increment_ip();
                }

                Instruction::CopyReg(destination_register, source_register) => {
                    self.frames.push(StackFrame {
                        file_name: String::from("boltvm"),
                        function_name: String::from("Instruction::CopyReg"),
                        line: 484,
                    });

                    let source = self.registers[source_register.as_index()].clone();

                    self.registers[destination_register.as_index()] = source;
                    self.increment_ip();
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
            _ => Err(Error::ExpectedType(String::from("int"), register)),
        }
    }

    #[inline]
    fn get_float(&self, register: Register) -> Result<f32> {
        match self.registers[register.as_index()] {
            Value::Float(value) => Ok(value),
            _ => Err(Error::ExpectedType(String::from("float"), register)),
        }
    }

    #[inline]
    fn get_string(&self, register: Register) -> Result<String> {
        match self.registers[register.as_index()] {
            Value::String(ref value) => Ok(value.clone()),
            _ => Err(Error::ExpectedType(String::from("string"), register)),
        }
    }

    #[inline]
    fn get_bool(&self, register: Register) -> Result<bool> {
        match self.registers[register.as_index()] {
            Value::Bool(value) => Ok(value),
            _ => Err(Error::ExpectedType(String::from("bool"), register)),
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn get_array(&self, register: Register) -> Result<Vec<Value>> {
        match &self.registers[register.as_index()] {
            Value::Array(value) => Ok(value.clone()),
            _ => Err(Error::ExpectedType(String::from("array"), register)),
        }
    }

    pub fn debug_dump(&self) {
        println!("========== debug dump ==========");

        let mut used_registers = 0;
        let mut values_left_on_stack = 0;

        println!("registers:");
        for (i, value) in self.registers.iter().enumerate() {
            if !value.is_null() {
                println!("	{}r{i}{}: {}", Color::Blue, Color::Reset, value);
                used_registers += 1;
            }
        }

        let total_registers = self.registers.len();
        let unused_registers = total_registers - used_registers;

        println!(
            "\n	used registers: {}{used_registers}{}",
            Color::Green,
            Color::Reset
        );
        println!(
            "	unused registers: {}{unused_registers}{}\n",
            Color::Yellow,
            Color::Reset
        );

        println!("stack:");
        if !self.stack.is_empty() {
            for (i, value) in self.stack.iter().enumerate() {
                println!("	{}s{}{}: {}", Color::Blue, i, Color::Reset, value);
                values_left_on_stack += 1;
            }

            println!(
                "\n	values still on stack: {}{values_left_on_stack}{}",
                Color::Yellow,
                Color::Reset
            );
        } else {
            println!("	{}stack is empty{}\n", Color::Red, Color::Reset);
        }

		println!("number of instructions executed: {}{}{}", Color::Blue, self.instruction_pointer, Color::Reset);
        println!("======= end of debug dump ======");
    }
}
