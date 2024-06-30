use std::ops::{Bound, RangeBounds};

pub struct Interval {
    pub start: f32,
    pub end: f32,
}

impl Interval {
    pub const EMPTY: Self = Self {
        start: f32::INFINITY,
        end: f32::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        start: f32::NEG_INFINITY,
        end: f32::INFINITY,
    };

    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> f32 {
        self.end - self.start
    }

    pub fn surrounds(&self, x: f32) -> bool {
        x > self.start && x < self.end
    }
}

impl RangeBounds<f32> for Interval {
    fn start_bound(&self) -> Bound<&f32> {
        if self.start == f32::NEG_INFINITY {
            Bound::Unbounded
        } else {
            Bound::Included(&self.start)
        }
    }

    fn end_bound(&self) -> Bound<&f32> {
        if self.end == f32::INFINITY {
            Bound::Unbounded
        } else {
            Bound::Included(&self.end)
        }
    }
}
