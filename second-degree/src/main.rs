#[derive(Debug)]
enum DType {
    DPositive(i32),
    DNegative(i32),
    DZero(i32),
}

struct DPositive {
    d: i32,
}
struct DNegative {
    _d: i32,
}
struct DZero {
    _d: i32,
}

struct SecondDegree {
    a: i32,
    b: i32,
    c: i32,
    d: DType,
}

trait Roots {
    fn get_roots(&self, a: i32, b: i32, _c: i32) -> (f64, f64);
}

impl SecondDegree {
    fn new(&self, a: i32, b: i32, c: i32) -> Self {
        let d = b * b - 4 * a * c;
        match d {
            d if d.is_positive() =>  SecondDegree{a, b, c, d: DType::DPositive(d)},
            d if d.is_negative() =>  SecondDegree{a, b, c, d: DType::DNegative(d)},
            _ =>  SecondDegree{a, b, c, d: DType::DZero(d)},
        }

    }
}

impl Roots for DPositive {
    fn get_roots(&self, a: i32, b: i32, _c: i32) -> (f64, f64) {
        let res1 = (f64::from(-b) + f64::from(self.d).sqrt()) / 2.0 * f64::from(a);
        let res2 = (f64::from(-b) - f64::from(self.d).sqrt()) / 2.0 * f64::from(a);
        (res1, res2)
    }
}

impl Roots for DZero {
    fn get_roots(&self, a: i32, b: i32, _c: i32) -> (f64, f64) {
        let res = f64::from(-b) / 2.0 * f64::from(a);
        (res, res)
    }
}

impl Roots for DNegative {
    fn get_roots(&self, _a: i32, _b: i32, _c: i32) -> (f64, f64) {
        (f64::NAN, f64::NAN)
    }
}

fn main() {
    let (a, b, c) = (2, 8, 1);
    println!("A={a}, B={b}, C={c}");

    let sd = SecondDegree::new(a, b, c);

    println!("D = {:?}", sd.d.get_roots());
}
