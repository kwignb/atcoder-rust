use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64, 
        b: f64, 
        h: f64, 
        m: f64, 
    }

    let hour = (h + m / 60.) * 30. / 180. * PI;
    let minute = m * 6. / 180. * PI;
    let theta = hour - minute;
    let ans_2;

    ans_2 = a * a + b * b - 2. * a * b * theta.cos();
    
    println!("{}", ans_2.sqrt());
}
