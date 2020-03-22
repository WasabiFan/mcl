use crate::geometry;

use rand::thread_rng;
use rand_distr::{Normal, Distribution};

pub struct Pose {
    location: geometry::Point
    // TODO: rotation
}

pub struct OdometryMeasurement {
    // velocities in units per tick
    vx: f64,
    vy: f64,
    sigma: f64
}

pub struct LocalizationParticleFilter {
    particles: Vec<Pose>
}

impl LocalizationParticleFilter {
    fn predict(&mut self, odom: OdometryMeasurement) {
        let mut rng = thread_rng();
        let vx = Normal::new(odom.vx, odom.sigma).unwrap();
        let vy = Normal::new(odom.vx, odom.sigma).unwrap();
        for particle in self.particles.iter_mut() {
            particle.location.x += vx.sample(&mut rng);
            particle.location.y += vy.sample(&mut rng);
        }
    }
}