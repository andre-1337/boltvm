use crate::register::Register;

/// All the possible value types
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
	Array(Vec<Value>),
    Null,
}

impl Value {
	pub fn is_int(&self) -> bool {
		match *self {
			Value::Int(_) => true,
			_ => false,
		}
	}

	pub fn is_float(&self) -> bool {
		match *self {
			Value::Float(_) => true,
			_ => false,
		}
	}

	pub fn is_string(&self) -> bool {
		match *self {
			Value::String(_) => true,
			_ => false,
		}
	}

	pub fn is_bool(&self) -> bool {
		match *self {
			Value::Bool(_) => true,
			_ => false,
		}
	}

	pub fn is_list(&self) -> bool {
		match *self {
			Value::Array(_) => true,
			_ => false,
		}
	}

	pub fn is_null(&self) -> bool {
		match *self {
			Value::Null => true,
			_ => false
		}
	}

    pub fn as_bool(&self) -> bool {
        match *self {
            Value::Bool(value) => value,
            _ => panic!("expected boolean value"),
        }
    }

    pub fn as_jump_target(&self) -> usize {
        match *self {
            Value::Int(value) => value as usize,
            _ => panic!("expected integer value as jump target"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(val) => write!(f, "{}", val),
            Value::Float(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
			Value::Array(val) => write!(f, "{:?}", val),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueOrRegister {
	Value(String),
	Register(Register),
}
