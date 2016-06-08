

struct Complex {
    real: f64,
    imaginary: f64
}

impl Complex {

    fn add(&self, other : Complex) -> Complex {
        return Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }

    fn multiply(&self, other : Complex) -> Complex {
        return Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary + other.real
        }
    }

}

#[test]
fn add_real() {
    let c1 = Complex { real: 42., imaginary: 31. };
    let c2 = Complex { real: 11., imaginary: -4. };
    assert_eq!(c1.add(c2).real, 53.);
}

#[test]
fn add_imaginary() {
    let c1 = Complex { real: 15., imaginary: 4. };
    let c2 = Complex { real: -17., imaginary: -4. };
    assert_eq!(c1.add(c2).imaginary, 0.);
}

// #[test]
// pub fn it_works() {
//
// }
