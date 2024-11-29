use std::fmt::Display;

use crate::gc::GcTrace;

// Implementation based off of Ceronman's Loxido: https://github.com/ceronman/loxido/blob/master/src/objects.rs

impl GcTrace for String {}

