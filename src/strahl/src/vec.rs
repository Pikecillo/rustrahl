use std::ops;

#[derive(Clone, Debug)]
pub struct Vec3f {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vec3f {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
		return Vec3f {x, y, z};
	}

	pub fn zero() -> Vec3f {
		return Vec3f::new(0.0, 0.0, 0.0);
	}

	pub fn dot(&self, other: &Vec3f) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	pub fn cross(&self, other: &Vec3f) -> Vec3f {
		return Vec3f::new(self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x);
	}

	pub fn length(&self) -> f32 {
		return self.dot(self).sqrt();
	}

	pub fn normalized(&self) -> Vec3f {
		let len = self.length();
		return self * (1.0 / len);
	}
}

impl ops::Add<&Vec3f> for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: &Vec3f) -> Vec3f {
		return Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
	}
}

impl ops::Add for &Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: &Vec3f) -> Vec3f {
		return Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
	}
}

impl ops::Sub<&Vec3f> for Vec3f {
	type Output = Vec3f;

	fn sub(self, rhs: &Vec3f) -> Vec3f {
		return Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
	}
}

impl ops::Sub for &Vec3f {
	type Output = Vec3f;

	fn sub(self, rhs: &Vec3f) -> Vec3f {
		return Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
	}
}

impl ops::Mul<f32> for Vec3f {
	type Output = Vec3f;

	fn mul(self, scalar: f32) -> Vec3f {
		return Vec3f::new(self.x * scalar, self.y * scalar, self.z * scalar);
	}
}

impl ops::Mul<f32> for &Vec3f {
	type Output = Vec3f;

	fn mul(self, scalar: f32) -> Vec3f {
		return Vec3f::new(self.x * scalar, self.y * scalar, self.z * scalar);
	}
}
