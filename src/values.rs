#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    VInteger(i64),
    VFloat(f64),
    VBoolean(bool),
    VString(String)
}

impl std::string::ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::VInteger(a) => a.to_string(),
            Value::VFloat(a) => a.to_string(),
            Value::VBoolean(a) => a.to_string(),
            Value::VString(a) => a.clone(),
        }
    }
}

impl Value {
    pub fn add(&self, v: &Value) -> Value {
        return match self {
            Value::VInteger(a) => match v {
                Value::VInteger(b) => Value::VInteger(a + b),
                Value::VFloat(b) => Value::VFloat(*a as f64 + b),
                Value::VBoolean(b) => Value::VInteger(a + *b as i64),
                Value::VString(b) => Value::VString(a.to_string() + b),
            },
            Value::VFloat(a) => match v {
                Value::VInteger(b) => Value::VFloat(a + *b as f64),
                Value::VFloat(b) => Value::VFloat(a + b),
                Value::VBoolean(b) => Value::VFloat(a + *b as i64 as f64), // Mandatory cast to i64 first.
                Value::VString(b) => Value::VString(a.to_string() + b),
            },
            Value::VBoolean(a) => match v {
                Value::VInteger(b) => Value::VInteger(*a as i64 + b),
                Value::VFloat(b) => Value::VFloat(*a as i64 as f64 + b), // Mandatory cast to i64 first.
                Value::VBoolean(b) => Value::VInteger(*a as i64 + *b as i64),
                Value::VString(b) => Value::VString(a.to_string() + b),
            },
            Value::VString(a) => match v {
                Value::VInteger(b) => Value::VString([a.to_string(), b.to_string()].concat()),
                Value::VFloat(b) => Value::VString([a.to_string(), b.to_string()].concat()),
                Value::VBoolean(b) => Value::VString([a.to_string(), b.to_string()].concat()),
                Value::VString(b) => Value::VString(a.to_string() + b),
            },
        }
    }
}
