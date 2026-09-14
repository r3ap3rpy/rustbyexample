use integrationtests::adder;

fn main() {
    let result = adder(3,2);
    assert_eq!(result, 5);
}
