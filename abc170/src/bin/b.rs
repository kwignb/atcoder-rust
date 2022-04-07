use proconio::input;

fn main() {
    input! {
        x: i32, y: i32,
    }
    if y % 2 == 0 && y >= x * 2 && x * 4 >= y {
        println!("Yes");
    } else {
        println!("No");
    }
}
