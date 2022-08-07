use crate::math;
use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub pos: math::Vec3,
    pub v: f32, // potential energy [kcal/mol]
    pub q: f32, // electrical charge [C]
}

impl Particle {
    pub fn new(charge: f32) -> Self {
        Self {
            pos: math::Vec3::empty(),
            v: 0.0,
            q: charge,
        }
    }

    pub fn create_electron(location: math::Vec3) -> Self {
        Self {
            pos: location,
            v: 0.0,
            q: -1.6e-19,
        }
    }

    pub fn create_proton(location: math::Vec3) -> Self {
        Self {
            pos: location,
            v: 0.0,
            q: 1.6e-19,
        }
    }

    // pub fn setPosition()
}
