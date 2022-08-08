use crate::math;
use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;

const PROTON_RENDER_RADIUS: f32 = 0.2;
const ELECTRON_RENDER_RADIUS: f32 = 0.1;

#[derive(Debug, Copy, Clone)]
pub enum SubatomicParticleType {
    Proton = 1,
    Neutron = 2,
    Electron = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub particle_type: SubatomicParticleType,
    pub m: f32,          // particle mass [kg]
    pub r: f32,          // radius of the particle [m]
    pub pos: math::Vec3, // current Euclidean XYZ position [m]
    pub v: f32,          // potential energy [kcal/mol]
    pub q: f32,          // electrical charge [C]
}

impl Particle {
    pub fn get_radius(particle_type: SubatomicParticleType) -> f32 {
        match particle_type {
            SubatomicParticleType::Proton | SubatomicParticleType::Neutron => PROTON_RENDER_RADIUS,
            SubatomicParticleType::Electron => ELECTRON_RENDER_RADIUS,
        }
    }

    pub fn get_color(particle_type: SubatomicParticleType) -> (f32, f32, f32) {
        match particle_type {
            SubatomicParticleType::Proton | SubatomicParticleType::Neutron => (1.0, 1.0, 1.0),
            SubatomicParticleType::Electron => (0.3, 0.3, 1.0),
        }
    }

    pub fn new(particle_type: SubatomicParticleType, mass: f32, charge: f32) -> Self {
        Self {
            particle_type: particle_type,
            m: mass,
            r: Particle::get_radius(particle_type),
            pos: math::Vec3::empty(),
            v: 0.0,
            q: charge,
        }
    }

    pub fn create_electron(location: math::Vec3) -> Self {
        Self {
            particle_type: SubatomicParticleType::Electron,
            m: 9.1093837015e-31,
            r: Particle::get_radius(SubatomicParticleType::Electron),
            pos: location,
            v: 0.0,
            q: -1.6e-19,
        }
    }

    pub fn create_proton(location: math::Vec3) -> Self {
        Self {
            particle_type: SubatomicParticleType::Proton,
            m: 1.67262192369e-27,
            r: Particle::get_radius(SubatomicParticleType::Proton),
            pos: location,
            v: 0.0,
            q: 1.6e-19,
        }
    }

    // pub fn setPosition()
}
