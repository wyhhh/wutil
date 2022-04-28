use std::time::Duration;
use std::time::Instant;

pub fn time_test<R>(f: impl FnOnce() -> R) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();
    (start.elapsed(), ret)
}

pub fn vec_push<T>(len: usize, mut constructor: impl FnMut() -> T) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(constructor());
    }
    vec
}

pub mod rate {
    use core::fmt;
    use num::Zero;
    use std::fmt::Display;

    pub struct Rate<T, T2>(pub T, pub T2, pub u8);
    pub struct RemainRate<T, T2>(pub T, pub T2, pub u8);

    pub trait RateTrait<T> {
        fn rate(self, other: T) -> f64;
    }

    impl RateTrait<f32> for f32 {
        fn rate(self, other: f32) -> f64 {
            (self as f64).rate(other as f64)
        }
    }

    impl RateTrait<f64> for f32 {
        fn rate(self, other: f64) -> f64 {
            (self as f64).rate(other)
        }
    }

    impl RateTrait<f32> for f64 {
        fn rate(self, other: f32) -> f64 {
            (self).rate(other as f64)
        }
    }

    impl RateTrait<f64> for f64 {
        fn rate(self, other: f64) -> f64 {
            self / other * 100.0
        }
    }

    impl<T, T2> fmt::Debug for Rate<T, T2>
    where
        T: Copy + RateTrait<T2>,
        T2: Copy + RateTrait<T> + Zero,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.1.is_zero() {
                write!(f, "N/A")
            } else {
                write!(
                    f,
                    "{:.width$?}%",
                    self.0.rate(self.1),
                    width = self.2 as usize
                )
            }
        }
    }

    impl<T, T2> Display for Rate<T, T2>
    where
        T: Copy + RateTrait<T2>,
        T2: Copy + RateTrait<T> + Zero,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<T, T2> fmt::Debug for RemainRate<T, T2>
    where
        T: Copy + RateTrait<T2>,
        T2: Copy + RateTrait<T> + Zero,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.1.is_zero() {
                write!(f, "N/A")
            } else {
                write!(
                    f,
                    "{:.width$?}%",
                    100.0 - self.0.rate(self.1),
                    width = self.2 as usize
                )
            }
        }
    }

    impl<T, T2> Display for RemainRate<T, T2>
    where
        T: Copy + RateTrait<T2>,
        T2: Copy + RateTrait<T> + Zero,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[test]
    fn test() {
        println!("{:?}", Rate(0.5_f32, 0.0_f32, 2));
        println!("{:?}", Rate(0.5_f32, 1.0_f32, 2));
        println!("{:?}", Rate(0.5_f32, 1.0_f64, 2));
        println!("{:?}", Rate(0.34232_f64, 1.0_f64, 44));
        println!("{:?}", Rate(0.34232_f64, 1.0_f32, 44));

        println!("{:?}", RemainRate(0.5_f32, 1.0_f32, 2));
        println!("{:?}", RemainRate(0.5_f32, 1.0_f64, 2));
        println!("{:?}", RemainRate(0.34232_f64, 1.0_f64, 44));
        println!("{:?}", RemainRate(0.34232_f64, 1.0_f32, 44));
        println!("{:?}", RemainRate(0.34232_f64, 2.0_f32, 44));
        println!("{:?}", RemainRate(0.34232_f64, 5.0_f32, 44));
        println!("{:?}", RemainRate(2.34232_f64, 5.0_f32, 44));
    }
}
