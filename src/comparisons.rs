use crate::Number;

impl<const N: usize> PartialOrd for Number<N> {
    fn gt(&self, other: &Self) -> bool {
        if self.body.len() < other.body.len() {
            return false;
        }
        if self.body.len() > other.body.len() {
            return true;
        }
        let mut i = 0;
        while i < self.body.len() {
            if self.body[i] == other.body[i] {
                i += 1;
                continue;
            }
            return self.body[i] > other.body[i];
        }
        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        self == other || self > other
    }

    fn lt(&self, other: &Self) -> bool {
        other > self && self != other
    }

    fn le(&self, other: &Self) -> bool {
        self == other || self < other
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        if self > other {
            return Some(Ordering::Greater);
        }
        if self == other {
            return Some(Ordering::Equal);
        }
        if self < other {
            return Some(Ordering::Less);
        }
        None
    }
}

impl<const N: usize> PartialEq for Number<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.body[i] != other.body[i] {
                return false;
            }
        }
        return true;
    }
    
    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl<const N: usize> Eq for Number<N> { }

impl<const N: usize> Ord for Number<N> {
    fn max(self, other: Self) -> Self {
        if self >= other {
            return self;
        } else {
            return other;
        }
    }

    fn min(self, other: Self) -> Self {
        if self <= other {
            return self;
        } else {
            return other;
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self
            .partial_cmp(other)
            .expect("Something has gone TERRIBLY wrong");
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            return min;
        }
        if self > max {
            return max;
        }
        self
    }
}
