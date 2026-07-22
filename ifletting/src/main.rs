enum Foo {
    Bar,
    Baz,
    Qux(u32)
}


fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched: {:?}",i);
    }

    if let Some(i) = letter {
        println!("Matched: {:?}",i);
    }else{
        println!("Did not match number, go with letter!");
    }
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched: {:?}",i);
    } else if i_like_letters {
        println!("Didn't match a number, Let's go with a letter!");
    } else {
        println!("I dont like letters, let's go with emoticon!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(10);

    if let Foo::Bar = a {
        println!("Bar it is!");
    }

    if let Foo::Baz = b {
        println!("Baz it is!");
    }

    if let Foo::Qux(value) = c {
        println!("The value is: {:?}",value);
    }
    if let Foo::Qux(value @ 100) = c {
        println!("The value is exactly 100");
    } else {
        println!("The value is something else, this branch cannot retrieve, use match if you need it!");
    }

    match c {
        Foo::Qux(value @ 100) => println!("The value is something 100"),
        Foo::Qux(value) => println!("The value is {:?}",value),
        _ => (),
    }
}
