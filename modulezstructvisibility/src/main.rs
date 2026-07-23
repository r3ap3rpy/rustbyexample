mod my_mod {
    pub struct OpenBox<T> {
        pub contents: T,
    }
    pub struct ClosedBox<T> {
        contents: T,
    }
    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}
fn main() {
    let open_box = my_mod::OpenBox {contents: "My public open box!"};
    println!("This OpenBox contains: {}",open_box.contents);

    let closed_box = my_mod::ClosedBox::new("classified information");
    // you cannot retrieve private fields
}
