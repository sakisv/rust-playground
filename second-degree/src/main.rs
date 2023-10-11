fn main() {
    let (a, b, c) = (2, 8, 1);
    println!("A={a}, B={b}, C={c}");

    let d: i32 = b*b - 4*a*c;
    println!("D = {d}");

    if d > 0 {
        let res1 = (f64::from(-b) + f64::from(d).sqrt()) / 2.0 * f64::from(a);
        let res2 = (f64::from(-b) - f64::from(d).sqrt()) / 2.0 * f64::from(a);
        println!("x1 = {res1}\nx2 = {res2}");
    } else if d < 0 {
        println!("Cannot work with negative D");
    } else {
        let res = f64::from(-b) / 2.0 * f64::from(a);
        println!("x = {res}");
    }
}
