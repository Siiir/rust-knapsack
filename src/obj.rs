use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

#[derive(
    derive_more::Constructor, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Debug,
)]
pub struct Object {
    value: NotNan<f64>,
    volume: NotNan<f64>,
}

impl Object {
    pub fn easy_new(value: f64, volume: f64) -> Self {
        Self::new(
            NotNan::new(value).expect("invalid value for `value` argument"),
            NotNan::new(volume).expect("invalid value for `volume` argument"),
        )
    }
    pub fn volume(&self) -> NotNan<f64> {
        self.volume
    }
    pub fn value(&self) -> NotNan<f64> {
        self.value
    }
    pub fn volume_as_f64(&self) -> f64 {
        self.volume.into_inner()
    }
    pub fn value_as_f64(&self) -> f64 {
        self.value.into_inner()
    }
}

#[cfg(test)]
mod test {
    use crate::Object;

    #[test]
    fn object_creation() {
        let obj = Object::easy_new(100.0, 10.0);

        assert_eq!(obj.value_as_f64(), 100.0);
        assert_eq!(obj.volume_as_f64(), 10.0);
    }

    #[test]
    fn object_equality() {
        let obj1 = Object::easy_new(100.0, 10.0);
        let obj2 = Object::easy_new(100.0, 10.0);

        assert_eq!(obj1, obj2);
    }
}
