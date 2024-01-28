// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / (60.0 * 60.0 * 24.0 * 365.25),
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planets {
    (for $($n:ident,$p:expr),+) => {
        $(
        pub struct $n;
        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.earth_years / $p
            }
        }
        )*
    }
}

planets!(
    for
        Mercury, 0.2408467,
        Venus, 0.61519726,
        Earth, 1.0,
        Mars, 1.8808158,
        Jupiter, 11.862615,
        Saturn, 29.447498,
        Uranus, 84.016846,
        Neptune, 164.79132
);
