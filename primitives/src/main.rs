fn main() {
    let logical: bool = true;
    let a_float: f64 = 3.14;
    let an_integer: i32 = 10;

    let default_float = 3.1;
    let default_integer = 10;

    let mut inferred_type = 12;
    inferred_type = 1212121;

    let mut mutable = 20;
    mutable = 30;

    // type cannot change just value
    // mutable = true;
    let mutable = true; // shadowing is overwriting

    let my_array: [i32;5] = [1,2,3,4,5];
    // tuples can hold values of different types
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
