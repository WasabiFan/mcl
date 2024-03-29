use crate::geometry;
use crate::stats::normal_pdf;
use crate::resample::resample;

use std::{cmp::Ordering, iter::FromIterator};

use rand::thread_rng;
use rand_distr::{Normal, Uniform, Distribution};

#[derive(Debug, Copy, Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64
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

#[derive(Debug, Copy, Clone)]
pub struct LaserScanMeasurement {
    pub down: f64,
    pub left: f64,
    pub sigma: f64
}

impl LaserScanMeasurement {
    pub fn simulated_measure_from(pose: &Pose, map: &Map, sigma: f64) -> LaserScanMeasurement {
        // TODO: is it smart to have the noise stddev be directly tied to particle likelihood?
        let pos_noise = Normal::new(0., sigma).unwrap();
        let mut rng = thread_rng();
        LaserScanMeasurement {
            down: pose.location.y + pos_noise.sample(&mut rng),
            left: pose.location.x + pos_noise.sample(&mut rng),
            sigma
        }
    }

    pub fn likelihood(&self, pose: &Pose, map: &Map) -> f64 {
        let left_expected = pose.location.x;
        let down_expected = pose.location.y;
        normal_pdf(self.down, self.sigma, down_expected) * normal_pdf(self.left, self.sigma, left_expected)
    }
}

pub struct LocalizationParticleFilter {
    pub particles: Vec<Pose>,
    map: Map,
    pose_estimate: Option<Pose>
}

impl LocalizationParticleFilter {
    pub fn new(num_particles: usize, map: &Map) -> LocalizationParticleFilter {
        let x = Uniform::new_inclusive::<f64, f64>(0.0, map.width as f64);
        let y = Uniform::new_inclusive::<f64, f64>(0.0, map.height as f64);
        let mut rng = thread_rng();
        LocalizationParticleFilter {
            particles: Vec::from_iter(
                (0..num_particles).map(|_| Pose { location: geometry::Point { x: x.sample(&mut rng), y: y.sample(&mut rng) } })
            ),
            map: map.clone(),
            pose_estimate: None
        }
    }

    pub fn predict(&mut self, odom: OdometryMeasurement) {
        let mut rng = thread_rng();
        let vx = Normal::new(odom.vx, odom.sigma).unwrap();
        let vy = Normal::new(odom.vy, odom.sigma).unwrap();
        for particle in self.particles.iter_mut() {
            particle.location.x += vx.sample(&mut rng);
            particle.location.y += vy.sample(&mut rng);
        }
    }

    pub fn update(&mut self, scan: &LaserScanMeasurement) {
        let likelihoods: Vec<f64> = self.particles.iter().map(|particle| scan.likelihood(particle, &self.map)).collect();

        // Normalize weights so they sum to 1
        let norm_factor: f64 = likelihoods.iter().sum();
        let weights: Vec<f64> = likelihoods.iter().map(|l| l / norm_factor).collect();

        // TODO: What's the smarter way to compute this pose estimate?
        // Currently, this is only based on the most recent measurement.
        self.pose_estimate = weights
            .iter()
            .zip(&self.particles)
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .and_then(|(_, pose_estimate)| Some(pose_estimate.clone()));

        self.particles = resample(&self.particles[..], &weights[..]);
    }

    pub fn get_pose_estimate(&self) -> Option<Pose> {
        self.pose_estimate
    }
}