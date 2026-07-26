struct A;
struct SingleGen<T>(T);
struct Single(A);
struct S(A);
struct Sgen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec(_s: Sgen<A>) {}
fn gen_spec_i32(_s: Sgen<i32>) {}
fn generic<T>(_s: Sgen<T>) {}

#[derive(Debug)]
struct Val {
    val: f64,
}

#[derive(Debug)]
struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T){}
}

fn main() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    reg_fn(S(A));
    gen_spec(Sgen(A));
    gen_spec_i32(Sgen(6));
    generic::<char>(Sgen('a'));
    generic(Sgen('c'));

    let value = Val { val: 3.14 };
    let gen_val = GenVal { gen_val: 3i32 };

    println!("value: {:?}, gen_val: {:?}",value, gen_val);

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
