extern crate sdl2;
extern crate strahl;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use std::vec;

fn render(screen_width: u32, screen_height: u32) -> vec::Vec<u8> {
	let eye = strahl::vec::Vec3f::new(0.0, 10.0, 8.0);
	let look_at = strahl::vec::Vec3f::new(0.0, 0.0, -9.0);
	let up = strahl::vec::Vec3f::new(0.0, 1.0, 0.0);
	let aspect_ratio = screen_height as f32 / screen_width as f32;
	let width = 3.0 * aspect_ratio;
	let height = 3.0;
	let far = 3.0;
	let camera = strahl::camera::PerspectiveCamera::new(eye, look_at, up,
		height, width, far);
	let ao_samples = 1;

	let mut scene = strahl::tracer::Scene::new();

	for x in -9 .. 10 {
		for z in -18 .. 1 {
			&scene.add_sphere(strahl::tracer::Sphere::new(strahl::vec::Vec3f::new(x as f32 * 2.0, 0.0, z as f32 * 2.0), 1.0));
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
