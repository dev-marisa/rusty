// iterative fibonacci
fn i_fib(mut n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let mut fib = 0;
    let mut y = 1;
    let mut x = 0;
    while n > 1 {
        fib = y + x;
        x = y;
        y = fib;
        n -= 1;
    }
    return fib;
}

// recursive fibonacci
fn r_fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    return r_fib(n-1) + r_fib(n-2);
}

fn main() {
    for n in 0..9 {
        println!("{} {}", n, r_fib(n));
    }
    for n in 0..9 {
        println!("{} {}", n, i_fib(n));
    }
}