fn diverging() -> ! {
    panic!("I shall never return!");
}
fn main() {

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 1..up_to {
            let addition: u32 = match i%2==1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd up to 9 (excluding): {}",sum_odd_numbers(9));
    diverging();
}
