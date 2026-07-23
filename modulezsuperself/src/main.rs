fn function() {
    println!("I am the outer function!");
}
mod cool {
    pub fn function() {
        println!("I am cool::function!");
    }
}

mod my {
    fn function() {
        println!("I am my::function!");
    }
    mod cool {
        pub fn function() {
            println!("I am my::cool::function!");
        }
    }
    pub fn indirect_call() {
        println!("I am my::indirect_call!");
        // current module scope
        self::function();
        function();
        // parents scope
        self::cool::function();
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
fn main() {
    my::indirect_call();
}


