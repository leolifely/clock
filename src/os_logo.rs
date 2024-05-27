use std::path::Path;

use os_info;
use os_info::Type;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;

pub fn get_distro_logo_path() -> String{
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Arch => "assets/arch.png".to_string(),
        _ => "assets/unknown.png".to_string(),
    }
}

pub fn draw_logo(logo_path: &str, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, window_width: i32, window_height: i32) {
    let texture_creator = canvas.texture_creator();
    let logo_texture = match texture_creator.load_texture(Path::new(logo_path)) {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Could not load logo texture: {}", e);
            return;
        }
    };

    let TextureQuery { width, height, .. } = logo_texture.query();
    let scale_factor = (window_width as f32 / 4.0) / width as f32;
    let new_width = (width as f32 * scale_factor) as u32;
    let new_height = (height as f32 * scale_factor) as u32;

    let target = sdl2::rect::Rect::new(
        (window_width - new_width as i32) / 2,
        (window_height - new_height as i32) / 2 + (window_height / 4),
        new_width,
        new_height
    );

    if let Err(e) = canvas.copy(&logo_texture, None, Some(target)) {
        eprintln!("Could not copy logo texture to canvas: {}", e);
    }
}