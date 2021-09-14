use chrono_tz::Tz;

use crate::{InputValueError, InputValueResult, Result, Scalar, ScalarType, Value};

#[Scalar(internal, name = "TimeZone")]
impl ScalarType for Tz {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(s.parse()?),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(Tz::name(self))
    }
}
