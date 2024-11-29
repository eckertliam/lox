use std::fmt::Display;

use crate::gc::GcRef;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    String(GcRef<String>),
    Number(f64),
    Bool(bool),
    Nil,
}


impl Value {
    pub fn equal(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => n == m,
            (Self::Bool(b), Self::Bool(c)) => b == c,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }

    pub fn greater(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => n > m,
            _ => false,
        }
    }

    pub fn less(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => n < m,
            _ => false,
        }
    }
}

macro_rules! unary_error {
    ($op:expr, $value:expr) => {
        {
            eprintln!("Runtime error: Cannot perform {} on {}", $op, $value);
            return Err(());
        }
    };
}

macro_rules! binary_error {
    ($op:expr, $lhs:expr, $rhs:expr) => {
        {
            eprintln!("Runtime error: Cannot perform {} on {} and {}", $op, $lhs, $rhs);
            return Err(());
        }
    };
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Self, ()>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(n) => Ok(Self::Number(-n)),
            Self::Bool(_) => unary_error!("-", &self),
            Self::Nil => unary_error!("-", &self),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Result<Self, ()>;

    fn not(self) -> Self::Output {
        match self {
            Self::Bool(b) => Ok(Self::Bool(!b)),
            _ => unary_error!("!", &self),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Result<Self, ()>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => Ok(Self::Number(n + m)),
            _ => binary_error!("+", &self, &rhs),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Self, ()>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => Ok(Self::Number(n - m)),
            _ => binary_error!("-", &self, &rhs),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Self, ()>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => Ok(Self::Number(n * m)),
            _ => binary_error!("*", &self, &rhs),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Self, ()>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => Ok(Self::Number(n / m)),
            _ => binary_error!("/", &self, &rhs),
        }
    }
}

impl std::ops::Rem for Value {
    type Output = Result<Self, ()>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(n), Self::Number(m)) => Ok(Self::Number(n % m)),
            _ => binary_error!("%", &self, &rhs),
        }
    }
}