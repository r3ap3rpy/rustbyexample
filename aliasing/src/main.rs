type Nanoseconds = u64;
type Inch = u64;
type U64 = u64;

use std::collections::HashMap;

fn main() {
    let nanoseconds: Nanoseconds = 5 as u64;
    let inches: Inch = 2 as U64;
    println!("nanoseconds: {}, inches: {}, unit: {}",nanoseconds,inches,nanoseconds + inches);
    // reduces boilerplate, io::Result<T> type is an alias for Result<T,io::Error>

    let mut my_hash_map: HashMap<char,char> = HashMap::new();
    my_hash_map.insert('c', 'k');
    println!("My hashmap has the following values: {:?}",my_hash_map);

}
