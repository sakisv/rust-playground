#[derive(Debug)]
enum DType {
    DPositive(i32),
    DNegative(i32),
    DZero(i32),
}

struct SecondDegree {
    a: i32,
    b: i32,
    c: i32,
    d: DType,
}

impl DType {
    fn get_roots(&self, a: i32, b: i32, _c: i32) -> (f64, f64) {
        match self {
            DType::DPositive(d) => {
                let res1 = (f64::from(-b) + f64::from(*d).sqrt()) / 2.0 * f64::from(a);
                let res2 = (f64::from(-b) - f64::from(*d).sqrt()) / 2.0 * f64::from(a);
                (res1, res2)
            },
            DType::DZero(_) => {
                let res = f64::from(-b) / 2.0 * f64::from(a);
                (res, res)
            },
            DType::DNegative(_) => {
                (f64::NAN, f64::NAN)
            }
        }
    }
}

impl SecondDegree {
    fn new(a: i32, b: i32, c: i32) -> Self {
        let d = b * b - 4 * a * c;
        match d {
            x if d.is_positive() =>  SecondDegree{a, b, c, d: DType::DPositive(x)},
            y if d.is_negative() =>  SecondDegree{a, b, c, d: DType::DNegative(y)},
            _ =>  SecondDegree{a, b, c, d: DType::DZero(d)},
        }
    }

    fn get_roots(&self) -> (f64, f64) {
        self.d.get_roots(self.a, self.b, self.c)
    }
}

fn main() {
    let (a, b, c) = (2, 8, 1);
    println!("A={a}, B={b}, C={c}");

    let sd = SecondDegree::new(a, b, c);

    println!("D = {:?}", sd.d);
    println!("Roots = {:?}", sd.get_roots());
}
