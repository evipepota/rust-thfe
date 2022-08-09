use rand::prelude::{thread_rng, Distribution};
type Torus = u32;

pub fn f2torus(src: f64) -> Torus {
    //f64 -> u32
    let torus = (src % 1.0) as f64 * 2u64.pow(32) as f64;
    return (torus as i64) as u32;
}

pub fn d_ta(std_dev: f64) -> Torus {
    //choose from modular normal distribution
    let mut rng = thread_rng();
    let dist = rand_distr::Normal::<f64>::new(0.0, std_dev).unwrap();
    f2torus(dist.sample(&mut rng))
}
