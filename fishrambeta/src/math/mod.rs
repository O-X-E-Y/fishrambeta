use num_rational::Rational64;

mod calculate;
mod calculate_exact;
mod differentiate;
mod factors;
mod multiply_by;
mod simplify;
mod to_latex;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum Equation {
    Variable(Variable),
    Negative(Box<Equation>),
    Addition(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>),
    Ln(Box<Equation>),
    Equals(Box<(Equation, Equation)>),
    Sin(Box<Equation>),
    Cos(Box<Equation>),
}
///Represents a single number
#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum Variable {
    Integer(i64),
    Rational((i64, i64)),
    Constant(Constant),
    Letter(String),
    Vector(String),
}
///Mathematical constants
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
pub enum Constant {
    PI,
    E,
}

impl Equation {
    fn get_number_or_none(&self) -> Option<Rational64> {
        match self {
            Equation::Variable(Variable::Integer(n)) => Some((*n).into()),
            Equation::Variable(Variable::Rational(r)) => Some(Rational64::new(r.0, r.1)),
            _ => None,
        }
    }
}
