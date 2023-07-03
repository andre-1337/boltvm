//! Implementation of stack frames for stack tracing in the occurrence of an error

#[derive(Debug, Clone)]
pub struct StackFrame {
	pub file_name: String,
	pub function_name: String,
	pub line: u32,
}
