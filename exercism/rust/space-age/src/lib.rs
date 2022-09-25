use space_age_derive::Planet;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}
pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31_556_952.0
    }
}

#[derive(Planet)]
pub struct Mercury;

#[derive(Planet)]
pub struct Venus;

#[derive(Planet)]
pub struct Earth;

#[derive(Planet)]
pub struct Mars;

#[derive(Planet)]
pub struct Jupiter;

#[derive(Planet)]
pub struct Saturn;

#[derive(Planet)]
pub struct Uranus;

#[derive(Planet)]
pub struct Neptune;