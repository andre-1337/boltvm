use crate::register::Register;
use crate::value::*;

/// Define the instructions for the virtual machine
#[derive(Debug, Clone)]
pub enum Instruction {
    // Regular instructions
    /// Push an integer into a register
    LoadInt(Register, i32),
    /// Push a float into a register
    LoadFlt(Register, f32),
    /// Push a string into a register
    LoadStr(Register, String),
    /// Push a boolean into a register
    LoadBool(Register, bool),
    /// Add two integers and store the result in a register       
    AddInt(Register, Register, Register),
    /// Add two floats and store the result in a register
    AddFlt(Register, Register, Register),
    /// Subtract two integers and store the result in a register
    SubInt(Register, Register, Register),
    /// Subtract two floats and store the result in a register
    SubFlt(Register, Register, Register),
    /// Multiply two integers and store the result in a register
    MulInt(Register, Register, Register),
    /// Multiply two floats and store the result in a register
    MulFlt(Register, Register, Register),
    /// Divide two integers and store the result in a register
    DivInt(Register, Register, Register),
    /// Divide two floats and store the result in a register
    DivFlt(Register, Register, Register),
    /// Concatenate two strings and store the result in a register
    ConcatStrings(Register, Register, Register),
    /// Perform a logical AND operation and store the result in a register
    AndBool(Register, Register, Register),
    /// Perform a logical OR operation and store the result in a register
    OrBool(Register, Register, Register),
    /// Check if the first integer is less than the second and store the result in a register
    LtInt(Register, Register, Register),
    /// Check if the first integer is greater than the second and store the result in a register
    GtInt(Register, Register, Register),
    /// Check if the first float is less than the second and store the result in a register
    LtFlt(Register, Register, Register),
    /// Check if the first float is greater than the second and store the result in a register
    GtFlt(Register, Register, Register),
    /// Check if two booleans are equal and store the result in a register
    EqBool(Register, Register, Register),
    /// Print the value of a register to standard output
    Print(ValueOrRegister),
    /// Creates an array with the given register
    CreateArray(Register),
    /// Adds an element to the list
    ArrayAdd(Register, Value),
    /// Gets an element at a certain index and stores it in the given register
    GetArrayElemPtr(Register, Register, usize),
    /// Stop execution
    Halt,
}
