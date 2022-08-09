use std::f64::consts::PI;

pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Copy for Complex {}

impl Clone for Complex {
    //Ownership
    fn clone(&self) -> Complex {
        *self
    }
}

/*pub struct Omega {
    pub o: [Complex; 512],
}

impl Omega {
    pub fn omega_init(n: usize) -> Self {
        let mut o = [Complex { re: 0.0, im: 0.0 }; 512];
        for i in 0..512 {
            o[i].re = (PI * 2.0 * (i as f64) / (n as f64)).cos();
            o[i].im = (PI * 2.0 * (i as f64) / (n as f64)).sin();
        }
        Omega { o: (o) }
    }
}*/

/*impl Omega {
    fn omega_inti
}*/

impl Complex {
    /*
    pub fn new(abs: f64, args: f64) -> Self {
        Self {
            re: (abs * args.cos()),
            im: (abs * args.sin()),
        }
    }
    */
    pub fn omega(n: usize, k: i64) -> Self {
        Self {
            re: (PI * 2.0 * (k as f64) / (n as f64)).cos(),
            im: (PI * 2.0 * (k as f64) / (n as f64)).sin(),
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let cop = *self;
        self.re = cop.re * rhs.re - cop.im * rhs.im;
        self.im = cop.re * rhs.im + cop.im * rhs.re;
    }
}
