struct A;
struct SingleGen<T>(T);
struct Single(A);
struct S(A);
struct Sgen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec(_s: Sgen<A>) {}
fn gen_spec_i32(_s: Sgen<i32>) {}
fn generic<T>(_s: Sgen<T>) {}

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
}
