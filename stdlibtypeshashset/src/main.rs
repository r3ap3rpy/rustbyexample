use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1,2,3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2,3,4].into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));
    // b.insert(&4) fails because value is already in the set.
    b.insert(5);
    println!("A: {:?}",a);
    println!("B: {:?}",b);
    println!("Union: {:?}",a.union(&b).collect::<Vec<&i32>>());
    println!("Difference: {:?}",a.difference(&b).collect::<Vec<&i32>>());
    println!("Intersection: {:?}",a.intersection(&b).collect::<Vec<&i32>>());
    println!("Symmetric difference: {:?}",a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
