use proconio::input;

fn main() {
    input! {
        mut a:[usize;3],
    }
    a.sort();
    println!("{}", a[2] - a[0]);
}
