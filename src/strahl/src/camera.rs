use std::vec;
use crate::{vec::Vec3f, basis::OrthonormalBasis, tracer::Ray};

pub struct PerspectiveCamera {
	frame: OrthonormalBasis,
	eye: Vec3f,
	width: f32,
	height: f32,
	far: f32
}

impl PerspectiveCamera {
	pub fn new(eye: Vec3f, look_at: &Vec3f, up: &Vec3f,
		width: f32, height: f32, far: f32) -> PerspectiveCamera {
		let forward = look_at - &eye;
		let frame = OrthonormalBasis::from_vw(up, &forward);
		
		return PerspectiveCamera{
			frame, eye, width, height, far
		};
	}

	pub fn generate_rays(&self, screen_width: u32, screen_height: u32) -> vec::Vec<Ray> {
		let mut rays : vec::Vec<Ray> = vec!();

		for yscreen in 0 .. screen_height {
			for xscreen in 0 .. screen_width {
				let xcamera = ((xscreen as f32) / ((screen_width - 1) as f32) - 0.5) * self.width;
				let ycamera = ((yscreen as f32) / ((screen_height - 1) as f32) - 0.5) * self.height;
				let frame = &self.frame;
				let world_space_direction = frame.eval(xcamera, ycamera, self.far);
				rays.push(Ray::new(self.eye.clone(), &world_space_direction));
			}
		}

		return rays;
	}
}