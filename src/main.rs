mod resample;
mod geometry;
mod stats;
mod localization;

use geometry::Point;
use localization::{
    LocalizationParticleFilter, 
    Map,
    OdometryMeasurement, 
    LaserScanMeasurement, 
    Pose
};

use std::error::Error;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use plotters::drawing::bitmap_pixel::BGRXPixel;
use plotters::element::*;
use plotters::prelude::*;

const MAP_WIDTH: usize = 200;
const MAP_HEIGHT: usize = 200;

const WIDTH: usize = MAP_WIDTH * 2;
const HEIGHT: usize = MAP_HEIGHT * 2;

const LASERSCAN_STDDEV: f64 = 10.;

fn map_point_to_window(point: Point) -> (i32, i32) {
    ((point.x / MAP_WIDTH as f64 * WIDTH as f64) as i32, (point.y / MAP_HEIGHT as f64 * HEIGHT as f64) as i32)
}

enum MCLPhase {
    Predict, Update
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0u8; WIDTH * HEIGHT * 4];
    let mut window = Window::new(
        "Monte Carlo Localization Demo",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

    let map = Map { width: MAP_WIDTH, height: MAP_HEIGHT };
    let mut true_pose = Pose { location: Point { x: (MAP_WIDTH / 2) as f64, y: (MAP_HEIGHT / 2) as f64} };
    let mut mcl = LocalizationParticleFilter::new(100, &map);
    let mut next_phase: MCLPhase = MCLPhase::Predict;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if let Some(keys) = window.get_keys_pressed(KeyRepeat::No) {
            for key in keys {
                match key {
                    Key::Space => {
                        next_phase = match next_phase {
                            MCLPhase::Predict => {
                                mcl.predict(OdometryMeasurement { vx: 10.0, vy: 0.0, sigma: 3.0 });
                                MCLPhase::Update
                            }
                            MCLPhase::Update => {
                                mcl.update(&LaserScanMeasurement::simulated_measure_from(&true_pose, &map, LASERSCAN_STDDEV));
                                MCLPhase::Predict
                            }
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        let root =
        BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut buf[..], (WIDTH as u32, HEIGHT as u32))?
            .into_drawing_area();
        root.fill(&BLACK)?;

        for particle in &mcl.particles {
            let (x, y) = map_point_to_window(particle.location);
            root.draw(&Circle::new((x, y), 1, ShapeStyle::from(&RED).filled()))?;
        }
        drop(root);
        window.update_with_buffer(unsafe { std::mem::transmute(&buf[..]) }, WIDTH, HEIGHT)?;
    }

    Ok(())
}