use crate::simulation;
use kiss3d::nalgebra::Vector3;
use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;

// artificially scale the particles up, so they can be seen when far apart
const PARTICLE_RADIUS_FACTOR: f32 = 0.5e4;

// According to: https://en.wikipedia.org/wiki/Proton
const PROTON_RENDER_RADIUS: f32 = 2.83e-15 * PARTICLE_RADIUS_FACTOR;

// According to: https://en.wikipedia.org/wiki/Classical_electron_radius
const ELECTRON_RENDER_RADIUS: f32 = 0.81794e-15 * PARTICLE_RADIUS_FACTOR;

#[derive(Debug, Copy, Clone)]
pub enum SubatomicParticleType {
    Proton = 1,
    Neutron = 2,
    Electron = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub pos: Vector3<f32>, // current Euclidean XYZ position [m]
    pub v: Vector3<f32>,   // current Euclidean XYZ velocity [m]
    pub m: f32,            // particle mass [kg]
    pub r: f32,            // radius of the particle [m]
    pub e: f32,            // potential energy [kcal/mol]
    pub q: f32,            // electrical charge [C]
    pub particle_type: SubatomicParticleType,
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
            v: Vector3::default(),
            m: mass,
            r: Particle::get_radius(particle_type),
            pos: Vector3::default(),
            e: 0.0,
            q: charge,
        }
    }

    pub fn create_electron(location: Vector3<f32>) -> Self {
        Self {
            particle_type: SubatomicParticleType::Electron,
            v: Vector3::default(),
            m: 9.1093837015e-31,
            r: Particle::get_radius(SubatomicParticleType::Electron),
            pos: location,
            e: 0.0,
            q: -1.6e-19,
        }
    }

    pub fn create_proton(location: Vector3<f32>) -> Self {
        Self {
            particle_type: SubatomicParticleType::Proton,
            v: Vector3::default(),
            m: 1.67262192369e-27,
            r: Particle::get_radius(SubatomicParticleType::Proton),
            pos: location,
            e: 0.0,
            q: 1.6e-19,
        }
    }

    // pub fn setPosition()
}
