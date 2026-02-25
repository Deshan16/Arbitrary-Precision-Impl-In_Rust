use super::super::BigFloat;

impl BigFloat {
    pub fn sub(&self, other: &Self) -> Self {
        let mut o = other.clone();
        o.sign *= -1;
        self.add(&o)
    }
}