use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::fmt;

struct Fruit {
    name: String,
}
impl Drop for Fruit {
    fn drop(&mut self) {
        println!("> Dropping :: {:?}",self.name);
    }
}

impl fmt::Debug for Fruit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Fruit").field("name",&self.name).finish()
    }
}

fn main() {
    let apple = Arc::new(Fruit { name:"Apple".to_string()});
    for _ in 1..10{
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}",apple);
        });
    }
    
    println!("{}",Arc::strong_count(&apple));
    thread::sleep(Duration::from_secs(2));
}
