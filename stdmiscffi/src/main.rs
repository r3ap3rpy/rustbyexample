use std::fmt;

#[cfg(target_family = "unix")]
#[link_name = "m"]
unsafe extern {
    unsafe fn csqrt(z: Complex) -> Complex;
    unsafe fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    let z = Complex {re: -1., im: 0.};
    let z_sqrt = unsafe {csqrt(z)};
    let z_cosf = unsafe {ccosf(z)};
    println!("sqrt: {:?}",z_sqrt);
    println!("cosf: {:?}",z_cosf);
}

#[repr(C)]
#[derive(Clone,Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
