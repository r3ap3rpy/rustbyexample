struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1} 
}

fn main() {
    let mut sequence = 0..3;
    println!("Four calls to the sequence!");
    println!("Next: {:?}",sequence.next());
    println!("Next: {:?}",sequence.next());
    println!("Next: {:?}",sequence.next());
    println!("Next: {:?}",sequence.next());

    for i in 0..3 {
        println!("For loop: {}",i);
    }

    println!("Next 4 terms of fibonacci are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("{}",i);
    }
    let array = [1u32,2,3,4,5];
    println!("Iterate over the array: {:?}",&array);
    for i in array.iter() {
        println!("{}",i);
    }
}
