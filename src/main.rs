mod os_logo;


extern crate sdl2;
extern crate chrono;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
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
    let (mut window_width, mut window_height) = canvas.output_size().unwrap();

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
        
        

        let now = Local::now();
       // if window_width != canvas.output_size().unwrap().0 {
            println!("Showing logo");
            (window_width, window_height) = canvas.output_size().unwrap();
            let TextureQuery { width, height, .. } = logo_texture.query();
            let scale_factor = (window_width as f32 / 4.0) / width as f32;
            let new_width = (width as f32 * scale_factor) as u32;
            let new_height = (height as f32 * scale_factor) as u32;
        
            let target = sdl2::rect::Rect::new(
                0,//(window_width as i32 - new_width as i32) / 2,
                0,//(window_height as i32- new_height as i32) / 2 + (height / 4) as i32,
                new_width,
                new_height
            );
            if let Err(e) = canvas.copy(&logo_texture, None, Some(target)) {
                eprintln!("Could not copy logo texture to canvas: {}", e);
            }
        //}

        let time_rect = draw_time(now.format("%H:%M:%S").to_string(), &mut canvas, &ttf_context, window_width as i32, window_height as i32);
        
    
        
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(1000 / 60));
        canvas.fill_rect(time_rect).unwrap();
        println!("{},{}", time_rect.width(), time_rect.height());
    }
}

fn draw_time(time: String, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext, window_width: i32, window_height: i32) -> Rect{

    let font_path: &Path = Path::new("fonts/DMMono-Regular.ttf");
    
    // Load the font
    let font_size = (window_width / 5) as u16;
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    // Render the text to a surface
    let surface = font.render(&time)
        .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255)).unwrap();

    // Create a texture from the surface
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

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
    target
}
