use std::time::Instant;

pub struct StdClock {
}

pub struct StdInstant {
    instant: Instant
}

impl Clock<StdInstant> for StdClock {
    fn now(&self) -> StdInstant {
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
    fn now(&self) -> T;
}

pub trait ClockInstant {
    fn difference_millis(&self, other_instant: &Self) -> u128;
}

pub struct ManualClock {
    pub now_milliseconds: u128
}

impl Clock<ManualClockInstant> for ManualClock {
    fn now(&self) -> ManualClockInstant {
        ManualClockInstant { milliseconds: self.now_milliseconds }
    }
}

pub struct ManualClockInstant {
    milliseconds: u128
}

impl ClockInstant for ManualClockInstant {
    fn difference_millis(&self, other_instant: &Self) -> u128 {
        other_instant.milliseconds - self.milliseconds
    }
}
