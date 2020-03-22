use crate::geometry;

use std::iter::FromIterator;

use rand::thread_rng;
use rand_distr::{Normal, Uniform, Distribution};

pub struct Map {
    pub width: usize,
    pub height: usize
}

#[derive(Debug, Copy, Clone)]
pub struct Pose {
    pub location: geometry::Point
    // TODO: rotation
}

#[derive(Debug, Copy, Clone)]
pub struct OdometryMeasurement {
    // velocities in units per tick
    pub vx: f64,
    pub vy: f64,
    pub sigma: f64
}

pub struct LocalizationParticleFilter {
    pub particles: Vec<Pose>,
    map: Map
}

impl LocalizationParticleFilter {
    pub fn new(num_particles: usize, map: Map) -> LocalizationParticleFilter {
        let x = Uniform::new_inclusive::<f64, f64>(0.0, map.width as f64);
        let y = Uniform::new_inclusive::<f64, f64>(0.0, map.height as f64);
        let mut rng = thread_rng();
        LocalizationParticleFilter {
            particles: Vec::from_iter(
                (0..num_particles).map(|_| Pose { location: geometry::Point { x: x.sample(&mut rng), y: y.sample(&mut rng) } })
            ),
            map: map
        }
    }

    pub fn predict(&mut self, odom: OdometryMeasurement) {
        let mut rng = thread_rng();
        let vx = Normal::new(odom.vx, odom.sigma).unwrap();
        let vy = Normal::new(odom.vx, odom.sigma).unwrap();
        for particle in self.particles.iter_mut() {
            particle.location.x += vx.sample(&mut rng);
            particle.location.y += vy.sample(&mut rng);
        }
    }
}