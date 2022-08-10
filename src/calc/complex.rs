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

use once_cell::sync::OnceCell;

use crate::param;

static OM: OnceCell<Omega> = OnceCell::new();
pub(crate) fn init() {
    let mut o = [[Complex { re: 0.0, im: 0.0 }; param::trgsw::N * 2]; param::trgsw::NBIT + 1];
    for bit in 1..param::trgsw::NBIT + 2 {
        for i in 0..param::trgsw::N * 2 {
            o[bit - 1][i].re = (PI * 2.0 * (i as f64) / (1 << bit) as f64).cos();
            o[bit - 1][i].im = (PI * 2.0 * (i as f64) / (1 << bit) as f64).sin();
        }
    }
    OM.set(Omega { o: (o) });
}
pub struct Omega {
    pub o: [[Complex; param::trgsw::N * 2]; param::trgsw::NBIT + 1],
}

impl Complex {
    pub fn omega(n: usize, k: i64) -> Self {
        let mut idx = 0;
        for i in 0..param::trgsw::NBIT + 1 {
            let ch = 1 << (i + 1);
            if ch == n {
                idx = i;
            }
        }
        OM.get().unwrap().o[idx][k as usize % n]
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
