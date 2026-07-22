fn main() {
    let n = 5;

    if n < 0 {
        println!("The n is negative!");
    } else if n > 5 {
        println!("The n is greater than 5");
    } else {
        println!("n is between 0 an 5");
    }
    let big_n = {
        if n > 0 && n < 6 {
            println!("increase n ten-fold");
            n * 10
        } else {
            println!("Just double it!");
            n * 2
        }

    };
    println!("The big_n has value: {:?}",big_n);
}
