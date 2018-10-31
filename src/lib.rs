extern crate num_complex;
extern crate num_rational;

use num_complex::Complex;
use num_rational::Rational64;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Number {
    Undefined,
    Integer(i64),
    Rational(Rational64),
    Real(f64),
    Complex(Complex<f64>),
}

impl Number {
    pub fn new_integer(value: i64) -> Number {
        Number::Integer(value)
    }

    pub fn new_rational(nominator: i64, denominator: i64) -> Number {
        if nominator % denominator == 0 {
            return Number::new_integer(nominator / denominator);
        }
        Number::Rational(Rational64::new(nominator, denominator))
    }

    pub fn new_real(value: f64) -> Number {
        if value.floor() == value {
            return Number::new_integer(value as i64);
        }
        Number::Real(value)
    }

    pub fn new_complex(real: f64, imaginary: f64) -> Number {
        if imaginary == 0.0 {
            return Number::new_real(real);
        }
        Number::Complex(Complex::new(real, imaginary))
    }
}

pub trait AddNumber {
    fn add_number(self, number: Number) -> Number;
}

impl AddNumber for i64 {
    fn add_number(self, number: Number) -> Number {
        match number {
            Number::Undefined => Number::Undefined,
            Number::Integer(i) => Number::new_integer(self + i),
            Number::Rational(r) => {
                Number::new_rational(self * r.denom().clone() + r.numer(), r.denom().clone())
            }
            Number::Real(r) => Number::new_real(r * self as f64),
            Number::Complex(c) => Number::new_complex((self as f64) * c.re, c.im),
        }
    }
}

impl AddNumber for f64 {
    fn add_number(self, number: Number) -> Number {
        match number {
            Number::Undefined => Number::Undefined,
            Number::Integer(i) => Number::new_real(self + i as f64),
            Number::Rational(r) => Number::new_real(self * rational_to_float(r)),
            Number::Real(r) => Number::new_real(r * self as f64),
            Number::Complex(c) => Number::new_complex((self as f64) * c.re, c.im),
        }
    }
}

fn rational_to_float(r: Rational64) -> f64 {
    r.numer().clone() as f64 / r.denom().clone() as f64
}
