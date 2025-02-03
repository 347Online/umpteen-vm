#[derive(Debug)]
pub enum Value {
    Empty,
    Number(f64),
    Bool(bool),
}

impl Value {
    pub fn kind(&self) -> &'static str {
        use Value as V;

        match self {
            V::Empty => "<empty>",
            V::Number(_) => "number",
            V::Bool(_) => "bool",
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "{}", self.kind()),
            Value::Number(x) => write!(f, "{x}"),
            Value::Bool(x) => write!(f, "{x}"),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
