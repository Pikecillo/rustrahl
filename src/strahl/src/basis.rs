use crate::vec::Vec3f;

pub struct OrthonormalBasis {
	pub u: Vec3f,
	pub v: Vec3f,
	pub w: Vec3f,
}

impl OrthonormalBasis {
	pub fn eval(&self, local_x: f32, local_y: f32, local_z: f32) -> Vec3f {
		let x = &self.u * local_x;
		let y = &self.v * local_y;
		let z = &self.w * local_z;
		return &(&x + &y) + &z;
	}

	pub fn from_vw(v: &Vec3f, w: &Vec3f) -> OrthonormalBasis {
		let w = w.normalized();
		let mut u = w.cross(&v.normalized());

		if u.length() < 1e-06 {
			u = w.cross(&Vec3f::new(0.0, 1.0, 0.0));
		}
		if u.length() < 1e-06 {
			u = w.cross(&Vec3f::new(1.0, 0.0, 0.0));
		}

		let v = u.cross(&w);

		return OrthonormalBasis{
			u, v, w
		};
	}

	pub fn from_u(u: &Vec3f) -> OrthonormalBasis {
		let u = u.normalized();
		let mut v = u.cross(&Vec3f::new(0.0, 0.0, 1.0));

		if v.length() < 1e-06 {
			v = u.cross(&Vec3f::new(0.0, 1.0, 0.0));
		}

		v = v.normalized();
		let w = v.cross(&u);

		return OrthonormalBasis{
			u, v, w
		};
	}
}