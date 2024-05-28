mod os_logo;
mod date_time;

extern crate sdl2;
extern crate chrono;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::FullscreenType;

use std::path::Path;
use std::time::Duration;

use chrono::Local;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Clock", 640, 480)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize the TTF context once
    let ttf_context = sdl2::ttf::init().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut is_fullscreen = false;

    let os_logo_path = os_logo::get_distro_logo_path();
    let texture_creator = canvas.texture_creator();
    let logo_texture = match texture_creator.load_texture(Path::new(&os_logo_path)) {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Could not load logo texture: {}", e);
            return;
        }
    };

    os_logo::draw_logo(&logo_texture, &mut canvas);

    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::F), ..} => {
                    if !is_fullscreen {
                        canvas.window_mut().set_fullscreen(FullscreenType::Desktop).unwrap();
                        is_fullscreen = true;
                    } else {
                        canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap();
                        is_fullscreen = false;
                    }
                }
                _ => {},
            }
        }
        
        

        let now = Local::now();
        
        os_logo::draw_logo(&logo_texture, &mut canvas);
        let (window_width, window_height) = canvas.output_size().unwrap();
        date_time::draw_time(now.format("%H:%M:%S").to_string(), &mut canvas, &ttf_context, window_width as i32, window_height as i32);
        
    
        
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}

