mod my_mod {
    fn private_function() {
        println!("I'm my_mod::private_function!");
    }
    pub fn public_function() {
        println!("I'm my_mod::public_function!");
    }
    pub fn indirect_access() {
        println!("Called my_mod::indirect_access to my_mod::private_function!");
        private_function();
    }
    pub mod nested {
        fn private_function() {
            println!("I am my_mod::nested::private_function!");
        }
        pub fn public_function() {
            println!("I am my_mod::nested::public_function!");
        }
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("I am only visible in the given path!");
            public_function_in_nested(); 
        }
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }
     pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}
fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my_mod::public_function();

    my_mod::indirect_access();
    my_mod::nested::public_function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}
