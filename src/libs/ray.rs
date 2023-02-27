use crate::libs::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: Option<f64>) -> Self {
        match time {
            Some(value) => Ray {
                origin,
                direction,
                time: value,
            },
            None => Ray {
                origin,
                direction,
                time: 0.0,
            },
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
