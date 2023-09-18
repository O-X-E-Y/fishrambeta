use crate::math::{Equation, Variable};
use std::collections::BTreeMap;

impl Equation {
    pub fn simplify(self) -> Self {
        let calculated_wrapped = self.calculate_exact();
        if calculated_wrapped.is_some() {
            let calculated = calculated_wrapped.unwrap();
            if calculated.is_integer() {
                return Equation::Variable(Variable::Integer(calculated.to_integer()));
            }
            return Equation::Variable(Variable::Rational((
                *calculated.numer(),
                *calculated.denom(),
            )));
        }
        match self {
            Equation::Variable(variable) => return Equation::Variable(variable),
            Equation::Negative(negative) => match *negative {
                Equation::Negative(negative) => return (*negative).simplify(),
                Equation::Variable(Variable::Integer(0)) => {
                    return Equation::Variable(Variable::Integer(0))
                }
                Equation::Variable(Variable::Integer(integer)) => {
                    return Equation::Variable(Variable::Integer(-integer))
                }
                Equation::Variable(Variable::Rational(rational)) => {
                    return Equation::Variable(Variable::Rational((-rational.0, rational.1)))
                }

                negative => return Equation::Negative(Box::new(negative.simplify())),
            },
            Equation::Addition(addition) => return simplify_addition(addition),
            Equation::Multiplication(multiplication) => simplify_multiplication(multiplication),
            Equation::Division(division) => return simplify_division(division),
            Equation::Power(power) => return simplify_power(power),
            Equation::Ln(ln) => return Equation::Ln(Box::new(ln.simplify())),
            Equation::Sin(sin) => return Equation::Sin(Box::new(sin.simplify())),
            Equation::Cos(cos) => return Equation::Cos(Box::new(cos.simplify())),
            Equation::Equals(equation) => {
                return Equation::Equals(Box::new((equation.0.simplify(), equation.1.simplify())))
            }
        }
    }
}

fn simplify_addition(addition: Vec<Equation>) -> Equation {
    let mut terms: BTreeMap<Equation, Vec<Equation>> = BTreeMap::new();

    for equation in addition.iter() {
        let (term, count) = match equation.clone().simplify() {
            Equation::Negative(negative) => (*negative, Equation::Variable(Variable::Integer(-1))),
            other => (other, Equation::Variable(Variable::Integer(1))),
        };

        match terms.get_mut(&term) {
            Some(total_count) => total_count.push(count),
            None => {
                terms.insert(term, vec![count]);
                ()
            }
        }
    }

    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter() {
        let next_term = Equation::Multiplication(vec![
            equation.clone(),
            Equation::Addition(count.clone()).simplify(),
        ])
        .simplify();
        simplified_addition.push(next_term);
    }

    return Equation::Addition(simplified_addition);
}

fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation {
    let mut terms: BTreeMap<Equation, i64> = BTreeMap::new();
    let mut negative = false;
    for equation in multiplication.iter() {
        let mut simplified = equation.clone().simplify();
        if simplified == Equation::Variable(Variable::Integer(0)) {
            return Equation::Variable(Variable::Integer(0));
        } else if simplified != Equation::Variable(Variable::Integer(1)) {
            simplified = match simplified {
                Equation::Negative(neg) => {
                    negative = !negative;
                    *neg
                }
                simplified => simplified,
            };

            if let Equation::Power(ref power) = simplified {
                if let Equation::Variable(variable) = &power.1 {
                    if let Variable::Integer(n) = variable {
                        //terms.insert(power.0.clone(), *terms.get(&simplified).unwrap_or(&0) + n);
                        //continue;
                        //TODO this breaks everythign for some reason
                    }
                }
            }
            terms.insert(
                simplified.clone(),
                *terms.get(&simplified).unwrap_or(&0) + 1,
            );
        }
    }

    let mut simplified_multiplication: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter() {
        if *count == 1 {
            simplified_multiplication.push(equation.clone())
        } else {
            simplified_multiplication.push(
                Equation::Power(Box::new((
                    equation.clone(),
                    Equation::Variable(Variable::Integer(*count)),
                )))
                .simplify(),
            );
        }
    }

    if simplified_multiplication.len() == 1 {
        return match negative {
            false => simplified_multiplication[0].clone(),
            true => Equation::Negative(Box::new(simplified_multiplication[0].clone())),
        };
    }

    let more_simplified_multiplication: Equation = Equation::Multiplication(
        simplified_multiplication
            .iter()
            .skip(1)
            .map(|x| x.clone())
            .collect::<Vec<_>>(),
    )
    .multiply_by(&simplified_multiplication[0]); //TODO this performance can be improved by
                                                 //omitting the clone but I don't know how yet
    if negative {
        return Equation::Negative(Box::new(more_simplified_multiplication));
    } else {
        return more_simplified_multiplication;
    }
}

fn simplify_power(power: Box<(Equation, Equation)>) -> Equation {
    let base = power.0.simplify();
    let exponent = power.1.simplify();

    if exponent == Equation::Variable(Variable::Integer(1)) {
        return base;
    }

    match base {
        Equation::Multiplication(terms) => {
            let mut simplified_power: Vec<Equation> = vec![];
            for term in terms.iter() {
                simplified_power.push(Equation::Power(Box::new((term.clone(), exponent.clone()))));
            }
            return Equation::Multiplication(simplified_power);
        }
        Equation::Power(ref power) => {
            if let Equation::Variable(ref exponent_as_variable) = exponent {
                if let Equation::Variable(ref exponent2_as_variable) = power.1 {
                    if (matches!(exponent_as_variable, Variable::Integer(_))
                        && matches!(exponent2_as_variable, Variable::Integer(_)))
                        || (matches!(exponent_as_variable, Variable::Rational(_))
                            && matches!(exponent2_as_variable, Variable::Rational(_)))
                    {
                        return Equation::Power(Box::new((
                            power.0.clone(),
                            Equation::Multiplication(vec![exponent, power.1.clone()]),
                        )))
                        .simplify();
                    }
                }
            }
        }
        _ => {}
    }

    return Equation::Power(Box::new((base, exponent)));
}

fn simplify_division(division: Box<(Equation, Equation)>) -> Equation {
    let mut numerator = division.0.simplify();
    let mut denominator = division.1.simplify();

    for factor in denominator.shared_factors(&numerator) {
        if (&numerator).has_factor(&factor) && (&denominator).has_factor(&factor) {
            numerator = numerator.remove_factor(&factor);
            denominator = denominator.remove_factor(&factor);
        }
    }

    numerator = numerator.simplify();
    denominator = denominator.simplify();

    return if numerator == Equation::Variable(Variable::Integer(0)) {
        Equation::Variable(Variable::Integer(0))
    } else if denominator == Equation::Variable(Variable::Integer(1)) {
        numerator
    } else {
        Equation::Division(Box::new((numerator, denominator)))
    };
}
