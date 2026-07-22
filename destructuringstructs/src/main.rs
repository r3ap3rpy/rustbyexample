
#[derive(Debug)]
struct Foo {
    x: (u32,u32),
    y: u32,
}

#[derive(Debug)]
struct Bar {
    foo: Foo,
}

fn main() {
    let foo = Foo {x: (10,20),y:30};

    match foo {
        Foo { x: (10,k),y:l} => println!("Gotcha: k: {} l: {}",k,l ),
        Foo { y, ..} => println!("Gotcha: {:?}",y),
        Foo { y:2, x:i} => println!("Gotcha: {:?}",i),

    }
    let faa = Foo { x: (1,2), y: 3};
    println!("{:?}",faa);
    let Foo {x: x0, y:y0 } = faa;
    println!("{x0:?} -> {y0}");

    let baba = Bar { foo: faa};
    println!("{:?}",baba);
    println!("{:?}",baba.foo);
}
