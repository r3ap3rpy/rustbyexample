fn main() {
    let collected_iterator: Vec<i32> = (1..10).collect();
    println!("Collected iterator: {:?}",collected_iterator);
    let mut xs = vec![1,2,3,4,5];
    println!("Initial vector: {:?}",xs);
    xs.push(10);
    println!("Length: {}",xs.len());
    println!("First: {}",xs[1]);
    println!("Last: {:?}",xs.pop());

    for x in xs.iter(){
        println!("Remaining elements: {}",x);
    }

    for (i,x) in xs.iter().enumerate() {
        println!("{} -> {}",i,x);
    }
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("New: {:?}",xs);
}
