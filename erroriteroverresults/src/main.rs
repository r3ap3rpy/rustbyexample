fn main() {
    let strings: Vec<&str> = vec!["tofu","22","33"];
    let numbers: Vec<i32> = strings.clone().into_iter().filter_map(|s|s.parse::<i32>().ok()).collect();
    println!("Numbers: {:?}",numbers);
    let mut errors = vec![];
    let failed_numbers: Vec<_> = strings.clone().into_iter().map(|s|s.parse::<u8>()).filter_map(|r| r.map_err(|e| errors.push(e)).ok()).collect();
    println!("Failed numbers: {:?}",failed_numbers);
    println!("Errors: {:?}",errors);

    let (valid, invalid): (Vec<_>,Vec<_>) = strings.clone().into_iter().map(|s|s.parse::<i32>()).partition(Result::is_ok);
    println!("valid: {:?}",valid);
    println!("invalid: {:?}",invalid);
    let valid_value: Vec<i32> = valid.into_iter().map(Result::unwrap).collect();
    let invalid_value: Vec<_> = invalid.into_iter().map(Result::unwrap_err).collect();
    println!("valid: {:?}",valid_value);
    println!("invalid_value: {:?}",invalid_value);
}
