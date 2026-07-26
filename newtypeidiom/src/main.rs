struct Miles(f64);
struct Kilometres(f64);

impl Miles {
    pub fn to_kilometres(&self) -> Kilometres {
        Kilometres(self.0 * 1.609344)
    }
}

impl Kilometres {
    pub fn to_miles(&self) -> Miles {
        Miles(self.0 / 1.609344)
    }
}

fn is_marathon(distance: &Miles) -> bool {
    distance.0 >= 26.2
}

fn main() {
    let distance = Miles(30.0);
    let distance_km = distance.to_kilometres();
    println!("Is marathon: {}",is_marathon(&distance));
    println!("Is marathon: {}",is_marathon(&distance_km.to_miles()));
}
