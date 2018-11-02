// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        (d.s as f64) / Self::orbital_period()
    }

    fn orbital_period() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period() -> f64 {
        0.2408467 * Earth::orbital_period()
    }
}

impl Planet for Venus {
    fn orbital_period() -> f64 {
        0.61519726 * Earth::orbital_period()
    }
}

impl Planet for Earth {
    fn orbital_period() -> f64 {
        31557600.0
    }
}

impl Planet for Mars {
    fn orbital_period() -> f64 {
        1.8808158 * Earth::orbital_period()
    }
}

impl Planet for Jupiter {
    fn orbital_period() -> f64 {
        11.862615 * Earth::orbital_period()
    }
}

impl Planet for Saturn {
    fn orbital_period() -> f64 {
        29.447498 * Earth::orbital_period()
    }
}

impl Planet for Uranus {
    fn orbital_period() -> f64 {
        84.016846 * Earth::orbital_period()
    }
}

impl Planet for Neptune {
    fn orbital_period() -> f64 {
        164.79132 * Earth::orbital_period()
    }
}
