fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
        println!("_mutable_integer is FROZEN: {}, cannot modify value!", _mutable_integer);
    }
    _mutable_integer = 20;
    println!("Now it's mutable since scope ran out: {}",_mutable_integer);
}
