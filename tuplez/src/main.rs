use std::fmt::{Result, Display, Formatter};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"Matrix\n")?;
        write!(f,"\t[ {}, {}\n",self.0,self.1)?;
        write!(f,"\t  {}, {} ]\n",self.2,self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("First value: {}",long_tuple.0);
    println!("Second value: {}",long_tuple.1);
    let touple_ception = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Toupleception: {:?}",touple_ception);
    // if the touple is longer than 12 elements it cannot be printed
    let pair = (1,true);
    println!("The pair is: {:?}",pair);
    println!("Single elemnt tuple: {:?}",(1,));
    println!("Just a number: {:?}",(1));

    let to_destruct = (1,2,3,4);
    let (a,b,c,d) = to_destruct;

    println!("After destruction: {a},{b},{c},{d}",a=a,b=b,c=c,d=d);
    let matr = Matrix(1.1,2.2,3.3,4.4);
    println!("Matrix: {:?}",matr);
    println!("Matrix: {}",matr);
}
