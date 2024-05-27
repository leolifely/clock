extern crate sdl2;
extern crate chrono;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use sdl2::video::FullscreenType;

use std::path::Path;
use std::time::Duration;

use chrono::Local;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("Clock", 640, 480)
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

    'running: loop {
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
        canvas.clear();
        let now = Local::now();
        let (width, height) = canvas.output_size().unwrap();
        draw_time(now.format("%H:%M:%S").to_string(), &mut canvas, &ttf_context, width as i32, height as i32);
        canvas.present();

        // Delay to control the frame rate
        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}

fn draw_time(time: String, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext, window_width: i32, window_height: i32) {
    // Ensure the font path is correct
    let font_path: &Path = Path::new("fonts/DMMono-Regular.ttf");
    
    // Load the font
    let font_size = (window_width / 5) as u16;
    let font = match ttf_context.load_font(font_path, font_size) {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Could not load font: {}", e);
            return;
        }
    };

    // Render the text to a surface
    let surface = match font.render(&time)
        .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)) {
        Ok(surface) => surface,
        Err(e) => {
            eprintln!("Could not render text to surface: {}", e);
            return;
        }
    };

    // Create a texture from the surface
    let texture_creator = canvas.texture_creator();
    let texture = match texture_creator.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Could not create texture from surface: {}", e);
            return;
        }
    };

    // Determine the text's width and height
    let TextureQuery { width, height, .. } = texture.query();

    // Define the destination rectangle and center it
    let target = sdl2::rect::Rect::new(
        (window_width - width as i32) / 2,
        (window_height - height as i32) / 2,
        width,
        height
    );

    // Copy the texture to the canvas
    if let Err(e) = canvas.copy(&texture, None, Some(target)) {
        eprintln!("Could not copy texture to canvas: {}", e);
    }
}
