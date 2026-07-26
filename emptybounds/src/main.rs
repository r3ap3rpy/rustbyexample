struct Cardinal;
struct Turkey;
struct BlueJay;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }


fn main() {
    let c = Cardinal;
    let _t = Turkey;
    let b = BlueJay;

    println!("The cardinal is: {}",red(&c));
    println!("The blue jay is: {}",blue(&b));

}
