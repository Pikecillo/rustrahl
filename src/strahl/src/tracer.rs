extern crate rand;

use rand::Rng;
use std::f32;
use std::vec;

use crate::{basis::OrthonormalBasis, vec::Vec3f};

#[derive(Debug)]
pub struct Ray {
	origin: Vec3f,
	direction: Vec3f
}

impl Ray {
	pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
		let normalized_direction = direction.normalized();
		return Ray {origin, direction: normalized_direction};
	}

	pub fn at(&self, t: f32) -> Vec3f {
		return self.origin + self.direction * t;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
	pub t: f32,
	pub normal: Vec3f,
	pub point: Vec3f
}

impl Hit {
	pub fn new(t: f32, normal: Vec3f, point: Vec3f) -> Hit {
		return Hit {t, normal, point};
	}

	pub fn miss() -> Hit {
		return Hit {t: f32::INFINITY, normal: Vec3f::zero(), point: Vec3f::zero()};
	}

	pub fn update(&mut self, other: &Hit) {
		if !other.is_miss() && other.t < self.t {
			self.t = other.t;
			self.normal = other.normal;
			self.point = other.point;
		}
	}

	pub fn is_miss(self) -> bool {
		return self.t == f32::INFINITY;
	}
}

pub trait Traceable {
	fn hit(&self, ray: &Ray) -> Hit;
}

#[derive(Debug)]
pub struct Sphere {
	center: Vec3f,
	radius: f32
}

impl Sphere {
	pub fn new(center: Vec3f, radius: f32) -> Sphere {
		return Sphere {center, radius};
	}
}

impl Traceable for Sphere {
	fn hit(&self, ray: &Ray) -> Hit {
		let center = self.center;
		let origin = ray.origin;
		let direction = ray.direction;
		let oc = origin - center;

		let a = direction.dot(&direction);
		let b = 2.0 * oc.dot(&direction);
		let c = oc.dot(&oc) - self.radius * self.radius;
		
		let discriminant = b * b - 4.0 * a * c;

		if discriminant < 0.0 {
			return Hit::miss();
		}

		let t = (-b - discriminant.sqrt()) / (2.0 * a);

		if t <= 1e-06 {
			return Hit::miss();
		}

		let hit_point = ray.at(t);
		let normal = hit_point - center;

		return Hit::new(t, normal.normalized(), hit_point);
	}
}

struct Plane {
	point: Vec3f,
	normal: Vec3f
}

impl Plane {
	fn new(point: Vec3f, normal: Vec3f) -> Plane {
		return Plane{point, normal};
	}
}

impl Traceable for Plane {
	fn hit(&self, ray: &Ray) -> Hit {
		let dn = ray.direction.dot(&self.normal);

		if dn.abs() < 1e-06 {
			return Hit::miss();
		}

		let t = (self.point - ray.origin).dot(&self.normal);

		return Hit::new(t, self.normal, ray.at(t));
	}
}

pub struct Scene {
	spheres: vec::Vec<Sphere>
}

impl Scene {
	pub fn new() -> Scene {
		return Scene{spheres: vec!()};
	}

	pub fn add_sphere(&mut self, sphere: Sphere) {
		self.spheres.push(sphere);
	}

	fn ray_cast(&self, ray: &Ray) -> Hit {
		let mut hit = Hit::miss();
		for sphere in &self.spheres {
			let current_hit = sphere.hit(&ray);
			&hit.update(&current_hit);
		}

		return hit;
	}

	pub fn trace(&self, rays: &vec::Vec<Ray>) -> vec::Vec<Hit> {
		let mut hits = vec!();
		
		for ray in rays {
			hits.push(self.ray_cast(&ray));
		}

		return hits;
	}

	pub fn occlusion(&self, rays: &vec::Vec<Ray>) -> f32 {
		let hits = &self.trace(rays);
		let mut attenuation = 0.0;

		for hit in hits {
			if !hit.is_miss() {
				attenuation += 1.0;
			}
		}

		return (1.1 * attenuation / hits.len() as f32).min(1.0);
	}

	pub fn ambient_occlusion(&self, hits: &vec::Vec<Hit>, samples: u32) -> vec::Vec<f32> {
		let mut ao_coefficients : vec::Vec<f32> = vec!();

		for hit in hits {
			if !hit.is_miss() {
				let rays = sample_hemisphere(hit.point, hit.normal, samples);
				ao_coefficients.push(self.occlusion(&rays));
			} else {
				ao_coefficients.push(0.0);
			}
		}

		return ao_coefficients;
	}
}

fn sample_hemisphere(center: Vec3f, normal: Vec3f, samples: u32) -> vec::Vec<Ray> {
	let mut rays : vec::Vec<Ray> = vec!();
	let mut rng = rand::thread_rng();
	let frame = OrthonormalBasis::from_u(normal);

	for _i in 0 .. samples {
    	let height: f32 = rng.gen_range(0.0, 1.0);
    	let angle: f32 = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
		let direction = frame.u * height + frame.v * angle.cos() + frame.w * angle.sin();
		rays.push(Ray::new(center, direction));
	}

	return rays;
}
