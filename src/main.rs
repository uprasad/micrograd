use std::fmt;
use std::ops;

struct Value<'a> {
    data: f64,
    deps: Vec<&'a Value<'a>>,
    label: &'a str,
    op: &'a str,
    grad: f64,
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_helper(f, 0)
    }
}

impl<'a> ops::Add<&'a Value<'a>> for &'a Value<'a> {
    type Output = Value<'a>;

    fn add(self, other: Self) -> Value<'a> {
        Value {
            data: self.data + other.data,
            deps: vec![self, other],
            label: "",
            op: "+",
            grad: 0.0,
        }
    }
}

impl<'a> ops::Mul<&'a Value<'a>> for &'a Value<'a> {
    type Output = Value<'a>;

    fn mul(self, other: Self) -> Value<'a> {
        Value {
            data: self.data * other.data,
            deps: vec![self, other],
            label: "",
            op: "*",
            grad: 0.0,
        }
    }
}

impl<'a> Value<'a> {
    fn fmt_helper(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let data = if self.op == "" {
            format!("({}={}, grad={:.3})", self.label, self.data, self.grad)
        } else {
            format!(
                "({}={}, '{}', grad={:.3})",
                self.label, self.data, self.op, self.grad
            )
        };
        if indent == 0 {
            writeln!(f, "{}", data)?;
        } else {
            writeln!(f, "{}|----{}", str::repeat(" ", 3 * (indent - 2)), data,)?;
        }

        for dep in &self.deps {
            dep.fmt_helper(f, indent + 2)?;
        }
        Ok(())
    }

    fn tanh(&'a self) -> Value<'a> {
        let exp = (2.0 * self.data).exp();
        let tanh = (exp - 1.0) / (exp + 1.0);
        Value {
            data: tanh,
            deps: vec![self],
            label: "",
            op: "tanh",
            grad: 0.0,
        }
    }

    fn new(data: f64, label: &'a str) -> Value<'a> {
        Value {
            data: data,
            deps: Vec::new(),
            label: label,
            op: "",
            grad: 0.0,
        }
    }
}

fn main() {
    let a = Value::new(2.0, "a");
    let b = Value::new(-3.0, "b");
    let c = Value::new(10.0, "c");

    let mut e = &a * &b;
    e.label = "e";

    let mut d = &e + &c;
    d.label = "d";

    let f = Value::new(-2.0, "f");

    let mut l = &d * &f;
    l.label = "L";
    l.grad = 1.0;

    println!("{}", l);
}
