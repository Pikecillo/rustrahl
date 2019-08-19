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
	pub fn new(eye: Vec3f, look_at: Vec3f, up: Vec3f,
		width: f32, height: f32, far: f32) -> PerspectiveCamera {
		let frame = OrthonormalBasis::from_vw(up, look_at - eye);
		
		return PerspectiveCamera{
			frame, eye, width, height, far
		};
	}

	pub fn generate_rays(self, screen_width: u32, screen_height: u32) -> vec::Vec<Ray> {
		let mut rays : vec::Vec<Ray> = vec!();

		let u = self.frame.u;
		let v = self.frame.v;
		let w = self.frame.w;

		for yscreen in 0 .. screen_height {
			for xscreen in 0 .. screen_width {
				let xcamera = ((xscreen as f32) / ((screen_width - 1) as f32) - 0.5) * self.width;
				let ycamera = ((yscreen as f32) / ((screen_height - 1) as f32) - 0.5) * self.height;
				let direction = u * xcamera + v * ycamera + w * self.far;
				rays.push(Ray::new(self.eye, direction));
			}
		}

		return rays;
	}
}