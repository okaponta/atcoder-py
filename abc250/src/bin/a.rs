use proconio::input;

fn main() {
    input! {
        h:usize,w:usize,
        r:usize,c:usize,
    }
    let mut ans = 4;
    if r == 1 {
        ans -= 1;
    }
    if r == h {
        ans -= 1;
    }
    if c == 1 {
        ans -= 1;
    }
    if c == w {
        ans -= 1;
    }
    println!("{}", ans);
}
