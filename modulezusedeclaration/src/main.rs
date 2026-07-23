use deeply::nested::function as other_function;

fn function() {
    println!("I am the outer function!");
}

pub mod deeply {
    pub mod nested {
        pub fn function() {
            println!("deeply::nested::function!");
        }
    }
}
fn main() {
    other_function();
    {
        use deeply::nested::function;
        function();
    }
    function();
}
