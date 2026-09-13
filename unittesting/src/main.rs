fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn bad_add(x: i32, y: i32) -> i32 {
    x - y
}

fn sqrt(number: f64) -> Result<f64,String> {
    if number > 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("Negative floats don't have a square root!".to_string())
    }
}

fn divide_non_zero_result(a: u32, b:u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error!");
    } else if a < b {
        panic!("Divide result is zero!");
    }
    a / b
}

#[cfg(test)]
mod unittesting {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(5,10),15);
    }
    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(4,2),2);
    }
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0),x);
        Ok(())
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        divide_non_zero_result(1, 0);
    }
    #[test]
    #[should_panic = "Divide result is zero!"]
    fn test_panic_message() {
        divide_non_zero_result(1, 2);
    }
    #[test]
    #[ignore]
    fn test_divide() {
        divide_non_zero_result(4,2);
    }
}

fn main() {

}
