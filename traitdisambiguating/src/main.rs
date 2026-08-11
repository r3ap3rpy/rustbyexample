trait UsernameWidget {
    fn get(&self) -> String;
}
trait AgeWidget {
    fn get(&self) -> u8;
}
struct Form {
    username: String,
    age: u8,
}
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}
fn main() {
    let form = Form {
        username: String::from("rustacean"),
        age: 36,
    };
    // print fails as rust cannot exactly choose between the two getters.
    //println!()
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!(String::from("rustacean"),username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(36,age);
}
