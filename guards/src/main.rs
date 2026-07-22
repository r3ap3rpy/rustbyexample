use std::todo;
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

struct Currency<'a> {
    name: &'a str,
    amount: u32,
}

fn main() {
    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("Temperature is rising, above 30 celsius!"),
        Temperature::Celsius(t) => println!("Below or equal 30 degrees!"),
        Temperature::Fahrenheit(t) if t > 86 => println!("Temperature is above 86 fahrenheit"),
        Temperature::Fahrenheit(t) => println!("Temperature is less or equal to 86 fahrenheit"),
    }

    let number: u8 = 4;

    match number {
        i if i>= 0 => println!("The number is positive!"),
        i => println!("The number is negative!"),
    }

    let euro = Currency { name: "Euro", amount: 300 };
    match euro {
        Currency {name: "Euro", amount: x} if x >= 300 => println!("So much EURO!"),
        Currency {name: "USD", amount: x} if x > 300 => println!("So much USD!"),
        _ => todo!("Implement rest!"),
    }
}
