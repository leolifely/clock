use std::path::Path;

use os_info;
use os_info::Type;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureQuery};

pub fn get_distro_logo_path() -> String{
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Arch => "assets/arch.png".to_string(),
        _ => "assets/unknown.png".to_string(),
    }
}




