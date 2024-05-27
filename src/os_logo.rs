use os_info;
use os_info::Type;
use sdl2::render::{Texture, TextureQuery};

pub fn get_distro_logo_path() -> String {
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Arch => "assets/arch.png".to_string(),
        _ => "assets/unknown.png".to_string(),
    }
}

pub fn draw_logo(logo_texture: &Texture, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.clear();
    let window_width = canvas.output_size().unwrap().0;
    let TextureQuery { width, height, .. } = logo_texture.query();
    let scale_factor = (window_width as f32 / 2.2) / width as f32;
    let new_width = (width as f32 * scale_factor) as u32;
    let new_height = (height as f32 * scale_factor) as u32;

    let target = sdl2::rect::Rect::new(
        (window_width as i32 - new_width as i32) / 2,
        0,//(window_height as i32- new_height as i32) / 2 + (height / 4) as i32,
        new_width,
        new_height
    );
    if let Err(e) = canvas.copy(&logo_texture, None, Some(target)) {
        eprintln!("Could not copy logo texture to canvas: {}", e);
    }
}