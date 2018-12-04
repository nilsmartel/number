extern crate num_complex;
extern crate num_rational;

mod number;

use num_complex::Complex;
use num_rational::Rational64;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
