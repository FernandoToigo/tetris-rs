use std::time::Instant;

pub struct StdClock {
}

pub struct StdInstant {
    instant: Instant
}

impl Clock<StdInstant> for StdClock {
    fn now() -> StdInstant {
        StdInstant {
            instant: Instant::now()
        }
    }
}

impl ClockInstant for StdInstant {
    fn difference_millis(&self, other_instant: &Self) -> u128 {
        other_instant.instant.duration_since(self.instant).as_millis()
    }
}

pub trait Clock<T: ClockInstant> {
    fn now() -> T;
}

pub trait ClockInstant {
    fn difference_millis(&self, other_instant: &Self) -> u128;
}
