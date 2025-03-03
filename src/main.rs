use std::fmt;
use std::ops;

struct Value<'a> {
    data: f64,
    deps: Vec<&'a Value<'a>>,
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
        }
    }
}

impl<'a> ops::Mul<&'a Value<'a>> for &'a Value<'a> {
    type Output = Value<'a>;

    fn mul(self, other: Self) -> Value<'a> {
        Value {
            data: self.data * other.data,
            deps: vec![self, other],
        }
    }
}

impl<'a> Value<'a> {
    fn fmt_helper(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        if indent == 0 {
            writeln!(f, "({})", self.data)?;
        } else {
            writeln!(
                f,
                "{}|--({})",
                str::repeat(" ", 2 * (indent - 2)),
                self.data
            )?;
        }

        for dep in &self.deps {
            dep.fmt_helper(f, indent + 2)?;
        }
        Ok(())
    }

    fn new(data: f64) -> Value<'a> {
        Value {
            data: data,
            deps: Vec::new(),
        }
    }
}

fn main() {
    let a = Value::new(2.0);
    let b = Value::new(-3.0);
    let c = Value::new(10.0);

    let e = &a * &b;
    let d = &e + &c;

    println!("{}", d);
}
