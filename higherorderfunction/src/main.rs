fn is_odd(i: i32) -> bool {
    i % 2 == 1
}
fn main() {
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n) {
            acc += n;
        }
    }
    println!("Imperative style: {}",acc);

    let sum: i32 = (0..).take_while(|&n| n * n < upper).filter(|&n| is_odd(n * n)).sum();
    println!("functional style: {}",sum);
}
