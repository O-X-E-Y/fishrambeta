use crate::math::{Equation, Variable};
use std::collections::BTreeMap;

impl Equation {
    pub fn calculate(self: &Self, values: &BTreeMap<Variable, f64>) -> f64 {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return *integer as f64,
                    Variable::Rational(rational) => {
                        return *(*rational).numer() as f64 / *(*rational).denom() as f64
                    }
                    _ => {}
                }
                return values[&variable];
            }
            Equation::Negative(negative) => return -negative.calculate(values),
            Equation::Addition(addition) => {
                return addition.iter().map(|x| x.calculate(&values)).sum()
            }
            Equation::Multiplication(multiplication) => {
                return multiplication
                    .iter()
                    .map(|x| x.calculate(&values))
                    .product()
            }
            Equation::Division(division) => {
                return division.0.calculate(&values) / division.1.calculate(&values)
            }
            Equation::Power(power) => {
                return power.0.calculate(&values).powf(power.1.calculate(&values))
            }
            Equation::Ln(ln) => return ln.calculate(values).ln(),
            Equation::Sin(sin) => return sin.calculate(values).sin(),
            Equation::Cos(cos) => return cos.calculate(values).cos(),
            Equation::Abs(abs) => return abs.calculate(values).abs(),
            Equation::Equals(_) => panic!("Cannot calculate equals"),
        }
    }
}
