use std::f32;
use std::ops;
use std::vec;

extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

#[derive(Clone, Copy, Debug)]
struct Vec3f {
	x: f32,
	y: f32,
	z: f32
}

impl Vec3f {
	fn new(x: f32, y: f32, z: f32) -> Vec3f {
		return Vec3f {x, y, z};
	}

	fn zero() -> Vec3f {
		return Vec3f::new(0.0, 0.0, 0.0);
	}

	fn dot(&self, other: &Vec3f) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	fn cross(&self, other: &Vec3f) -> Vec3f {
		return Vec3f::new(self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x);
	}

	fn length(&self) -> f32 {
		return self.dot(self).sqrt();
	}

	fn normalized(self) -> Vec3f {
		let len = self.length();
		return self * (1.0 / len);
	}
}

impl ops::Add for Vec3f {
	type Output = Vec3f;

	fn add(self, rhs: Vec3f) -> Vec3f {
		return Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
	}
}

impl ops::Sub for Vec3f {
	type Output = Vec3f;

	fn sub(self, rhs: Vec3f) -> Vec3f {
		return Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
	}
}

impl ops::Mul<f32> for Vec3f {
	type Output = Vec3f;

	fn mul(self, scalar: f32) -> Vec3f {
		return Vec3f::new(self.x * scalar, self.y * scalar, self.z * scalar);
	}
}

#[derive(Debug)]
struct Ray {
	origin: Vec3f,
	direction: Vec3f
}

impl Ray {
	fn new(origin: Vec3f, direction: Vec3f) -> Ray {
		let normalized_direction = direction.normalized();
		return Ray {origin, direction: normalized_direction};
	}

	fn at(&self, t: f32) -> Vec3f {
		return self.origin + self.direction * t;
	}
}

#[derive(Clone, Copy, Debug)]
struct Hit {
	t: f32,
	normal: Vec3f,
	point: Vec3f
}

impl Hit {
	fn new(t: f32, normal: Vec3f, point: Vec3f) -> Hit {
		return Hit {t, normal, point};
	}

	fn miss() -> Hit {
		return Hit {t: f32::INFINITY, normal: Vec3f::zero(), point: Vec3f::zero()};
	}

	fn update(&mut self, other: &Hit) {
		if !other.is_miss() && other.t < self.t {
			self.t = other.t;
			self.normal = other.normal;
			self.point = other.point;
		}
	}

	fn is_miss(self) -> bool {
		return self.t == f32::INFINITY;
	}
}

trait Traceable {
	fn hit(&self, ray: &Ray) -> Hit;
}

#[derive(Debug)]
struct Sphere {
	center: Vec3f,
	radius: f32
}

impl Sphere {
	fn new(center: Vec3f, radius: f32) -> Sphere {
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

struct OrthonormalBasis {
	u: Vec3f,
	v: Vec3f,
	w: Vec3f,
}

impl OrthonormalBasis {
	fn from_vw(v: Vec3f, w: Vec3f) -> OrthonormalBasis {
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

	fn from_u(u: Vec3f) -> OrthonormalBasis {
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

struct PerspectiveCamera {
	frame: OrthonormalBasis,
	eye: Vec3f,
	width: f32,
	height: f32,
	far: f32
}

impl PerspectiveCamera {
	fn new(eye: Vec3f, look_at: Vec3f, up: Vec3f,
		width: f32, height: f32, far: f32) -> PerspectiveCamera {
		let frame = OrthonormalBasis::from_vw(up, look_at - eye);
		
		return PerspectiveCamera{
			frame, eye, width, height, far
		};
	}

	fn generate_rays(self, screen_width: u32, screen_height: u32) -> vec::Vec<Ray> {
		let mut rays : vec::Vec<Ray> = vec::Vec::new();

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

struct Scene {
	spheres: vec::Vec<Sphere>
}

impl Scene {
	fn new() -> Scene {
		return Scene{spheres: vec::Vec::new()};
	}

	fn add_sphere(&mut self, sphere: Sphere) {
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

	fn trace(&self, rays: &vec::Vec<Ray>) -> vec::Vec<Hit> {
		let mut hits = vec::Vec::new();
		
		for ray in rays {
			hits.push(self.ray_cast(&ray));
		}

		return hits;
	}

	fn occlusion(&self, rays: &vec::Vec<Ray>) -> f32 {
		let hits = &self.trace(rays);
		let mut attenuation = 0.0;

		for hit in hits {
			if !hit.is_miss() {
				attenuation += 1.0;
			}
		}

		return (1.1 * attenuation / hits.len() as f32).min(1.0);
	}

	fn ambient_occlusion(&self, hits: &vec::Vec<Hit>, samples: u32) -> vec::Vec<f32> {
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

fn render(screen_width: u32, screen_height: u32) -> vec::Vec<u8> {
	let eye = Vec3f::new(0.0, 10.0, 8.0);
	let look_at = Vec3f::new(0.0, 0.0, -9.0);
	let up = Vec3f::new(0.0, 1.0, 0.0);
	let aspect_ratio = screen_height as f32 / screen_width as f32;
	let width = 3.0 * aspect_ratio;
	let height = 3.0;
	let far = 3.0;
	let camera = PerspectiveCamera::new(eye, look_at, up,
		height, width, far);
	let ao_samples = 256;

	let mut scene = Scene::new();

	for x in -9 .. 10 {
		for z in -18 .. 1 {
			&scene.add_sphere(Sphere::new(Vec3f::new(x as f32 * 2.0, 0.0, z as f32 * 2.0), 1.0));
		}
	}

	let hits = &scene.trace(&camera.generate_rays(screen_width, screen_height));
	let ao_coefficients = &scene.ambient_occlusion(&hits, ao_samples);
	let mut framebuffer = vec![0; hits.len() * 3];

	for i in 0 .. hits.len() {
		let hit = hits[i];
		if hit.is_miss() {
			framebuffer[3 * i] = 0;
			framebuffer[3 * i + 1] = 0;
			framebuffer[3 * i + 2] = 0;
		}
		else {
			let attenuation = ao_coefficients[i];
			framebuffer[3 * i] = (255.0 * (1.0 - attenuation)) as u8;
			framebuffer[3 * i + 1] = (255.0 * (1.0 - attenuation)) as u8;
			framebuffer[3 * i + 2] = (255.0 * (1.0 - attenuation)) as u8;
			//framebuffer[3 * i] = (255.0 * hit.normal.x.abs()) as u8;
			//framebuffer[3 * i + 1] = (255.0 * hit.normal.y.abs()) as u8;
			//framebuffer[3 * i + 2] = (255.0 * hit.normal.z.abs()) as u8;
			framebuffer[3 * i] = (255.0 * (1.0 - attenuation) * hit.normal.x.abs()) as u8;
			framebuffer[3 * i + 1] = (255.0 * (1.0 - attenuation) * hit.normal.y.abs()) as u8;
			framebuffer[3 * i + 2] = (255.0 * (1.0 - attenuation) * hit.normal.z.abs()) as u8;
		}
	}

	return framebuffer;
}

fn main() -> Result<(), String> {
	let width: u32 = 900;
	let height: u32 = 700;
	let sdl = sdl2::init().unwrap();
	let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("AO", width, height)
        .resizable()
        .build()
        .unwrap();

	let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

	let texture_creator = canvas.texture_creator();

	let framebuffer = render(width, height);

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .map_err(|e| e.to_string())?;

	canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
	canvas.present();

	let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for y in 0 .. height as usize {
           		for x in 0 .. width as usize {
               		let buffer_offset = y * pitch + x * 3;
					let reverse_y = height as usize - y - 1;
					let framebuffer_offset = (reverse_y * width as usize + x) * 3;
              		buffer[buffer_offset] = framebuffer[framebuffer_offset];
               		buffer[buffer_offset + 1] = framebuffer[framebuffer_offset + 1];
               		buffer[buffer_offset + 2] = framebuffer[framebuffer_offset + 2];
           		}
			}
    	})?;

		canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height)))?;
		canvas.present();
    }

	return Ok(());
}
