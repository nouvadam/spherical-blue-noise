//! # Spherical blue noise generator
//!
//! Library for generating points with blue noise distribution on a unit sphere.
//!
//! The underlying idea for this algorithm is:
//! * First generate points on a sphere with random (white) distribution.
//! * Then treat each point as physically, charged particle and apply to each one repulsive force from other particles.
//! * With time, particles converges to the equilibrium that resembles blue noise.
//!
//! Hence, the time complexity of this algorithm is O(N^2), where N is the number of points. (could be made faster by using octree, like in barnes-hut algorithm)
//!
//! # Example
//!
//! ````rust
//! use spherical_blue_noise::*;
//!
//!let blue_noise_vec: Vec<(f32, f32, f32)> = BlueNoiseSphere::new(16, &mut rand::thread_rng()).into_iter().collect();
//!println!("{:?}", blue_noise_vec);
//!
//! ````
//!
//! The basic idea is based on the paper:
//!
//! Wong, Kin-Ming and Wong, Tien-Tsin. "Spherical Blue Noise", Pacific Graphics Short Papers, 2018,
//! [link](https://diglib.eg.org/handle/10.2312/pg20181267)

#![warn(missing_docs)]

use glam::*;
use rand::Rng;
use rand_distr::{Distribution, UnitSphere};
use rayon::prelude::*;

/// # Points on sphere that forms blue noise.
///
/// The only way to get points is through iterator, created by calling into_iterator method:
///
/// ```
/// BlueNoiseSphere::new(16, &mut rand::thread_rng()).into_iter()
/// ```

#[derive(Clone)]
pub struct BlueNoiseSphere {
    particles: Vec<Vec3>,
}

impl BlueNoiseSphere {
    /// Creates new spherical blue noise from `num_of_points` with default parameters.

    pub fn new<R: Rng + ?Sized>(num_of_points: u32, rng: &mut R) -> Self {
        Self::new_raw(num_of_points, rng).advance_multiple(
            16,
            0.999_383_57_f32.powi(num_of_points as i32) / 4.0 + 0.01,
            0.8,
        )
    }

    /// Creates new spherical blue noise with passed parameters.
    ///
    /// * `num_of_points` - The number of points that should lie on the sphere.
    ///
    /// * `num_of_iterations` - The number of iterations the algorithm makes before returning blue noise. More is slower and better.
    ///
    /// * `maximum_angular_displacement_threshold` - The "starting speed" of points to converge into blue noise.
    ///
    /// * `angular_displacement_threshold_decay` - The ratio that governs the decay of the starting speed. If the ratio is bigger, then the algorithm needs fewer iterations to converge, at the cost of more randomness in produced pattern. With bigger ratio, point may "vibrate" rather than converge into blue noise.

    pub fn new_with_params<R: Rng + ?Sized>(
        rng: &mut R,
        num_of_points: u32,
        num_of_iterations: u16,
        maximum_angular_displacement_threshold: f32,
        angular_displacement_threshold_decay: f32,
    ) -> Self {
        Self::new_raw(num_of_points, rng).advance_multiple(
            num_of_iterations,
            maximum_angular_displacement_threshold,
            angular_displacement_threshold_decay,
        )
    }

    /// Returns new `BlueNoiseSphere` with random points on the sphere, without passed any iteration of the algorithm, should then be called method `advance` or `advance_multiple`.

    pub fn new_raw<R: Rng + ?Sized>(num_of_points: u32, rng: &mut R) -> Self {
        BlueNoiseSphere {
            particles: (0..num_of_points)
                .map(|_point| {
                    let sample = UnitSphere.sample(rng);
                    Vec3::new(sample[0], sample[1], sample[2])
                })
                .collect(),
        }
    }

    /// Calls `num_of_iterations` times the `advance` method on `BlueNoiseSphere`. Each time the `maximum_angular_displacement_threshold` is reduced by `angular_displacement_threshold_decay`.
    pub fn advance_multiple(
        self,
        num_of_iterations: u16,
        maximum_angular_displacement_threshold: f32,
        angular_displacement_threshold_decay: f32,
    ) -> Self {
        match num_of_iterations {
            0 => self,
            _ => self
                .advance(maximum_angular_displacement_threshold)
                .advance_multiple(
                    num_of_iterations - 1,
                    maximum_angular_displacement_threshold * angular_displacement_threshold_decay,
                    angular_displacement_threshold_decay,
                ),
        }
    }

    /// Makes random points on sphere into blue noise. Takes `maximum_angular_displacement_threshold` as displacement (in radians) of each point toward direction, in which the point is pushed away by other particles. Returns new `BlueNoiseSphere` struct.
    pub fn advance(&self, maximum_angular_displacement_threshold: f32) -> Self {
        BlueNoiseSphere {
            particles: self
                .particles
                .par_iter()
                .map(|current_particle| {
                    // Update force at t time for the particle.
                    let updated_angular_acceleration = self
                        .particles
                        .iter()
                        .filter(|&particle| particle != current_particle)
                        .fold(Vec3::zero(), |curr_acc, other_particle| {
                            curr_acc
                                - (current_particle.cross(*other_particle).normalize()
                                    / ((current_particle.angle_between(*other_particle).powi(2))
                                        + 0.00000001))
                        });

                    // Update particle position and return it
                    Mat3::from_axis_angle(
                        updated_angular_acceleration.normalize(),
                        maximum_angular_displacement_threshold,
                    )
                    .mul_vec3(*current_particle)
                })
                .collect(),
        }
    }
}

/// Changes `BlueNoiseSphere` struct into iterator by converting particles into vector of 3 element tuples.
#[derive(Clone)]
pub struct BlueNoiseSphereIterator {
    points: Vec<(f32, f32, f32)>,
}

impl IntoIterator for BlueNoiseSphere {
    type Item = (f32, f32, f32);
    type IntoIter = BlueNoiseSphereIterator;

    fn into_iter(self) -> Self::IntoIter {
        BlueNoiseSphereIterator {
            points: self
                .particles
                .iter()
                .map(|particle| (particle.x, particle.y, particle.z))
                .collect(),
        }
    }
}

impl Iterator for BlueNoiseSphereIterator {
    type Item = (f32, f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        self.points.pop()
    }
}
