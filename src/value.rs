use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    pub fn not(&self) -> Result<Value, String> {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(format!("Cannot negate {}", self)),
        }
    }

    pub fn negate(&self) -> Result<Value, String> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(format!("Cannot negate {}", self)),
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n + m)),
            (Value::String(s), Value::String(t)) => Ok(Value::String(s.clone() + &t)),
            _ => Err(format!("Cannot add {} and {}", self, other)),
        }
    }

    pub fn subtract(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n - m)),
            _ => Err(format!("Cannot subtract {} and {}", self, other)),
        }
    }

    pub fn multiply(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n * m)),
            _ => Err(format!("Cannot multiply {} and {}", self, other)),
        }
    }

    pub fn divide(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n / m)),
            _ => Err(format!("Cannot divide {} and {}", self, other)),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Ok(Value::Number(n % m)),
            _ => Err(format!("Cannot modulo {} and {}", self, other)),
        }
    }

    pub fn equal(&self, other: &Value) -> Value {
        Value::Bool(self == other)
    }

    pub fn not_equal(&self, other: &Value) -> Value {
        Value::Bool(self != other)
    }

    pub fn greater(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Value::Bool(n > m),
            _ => Value::Bool(false),
        }
    }

    pub fn less(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(n), Value::Number(m)) => Value::Bool(n < m),
            _ => Value::Bool(false),
        }
    }
}
