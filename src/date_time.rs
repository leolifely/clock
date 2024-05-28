use std::path::Path;

use sdl2::render::TextureQuery;

pub fn draw_time(time: String, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext, window_width: i32, window_height: i32) {

    let font_path: &Path = Path::new("assets/DMMono-Regular.ttf");
    
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
}

pub fn draw_date(date: String, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext, window_width: i32, window_height: i32) {
    let font_path: &Path = Path::new("assets/DMMono-Regular.ttf");
    let font_size = (window_width / 8) as u16;
    let font = ttf_context.load_font(font_path, font_size).unwrap();
    
    let surface = font .render(&date)
        .blended(sdl2::pixels::Color::RGBA(50, 200, 50, 255)).unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let TextureQuery {width, height, ..} = texture.query();
    
    let target = sdl2::rect::Rect::new(
        (window_width - width as i32) / 2, 
        ((window_height - height as i32) / 2) + window_height / 4, 
        width, 
        height);
    
    if let Err(e) = canvas.copy(&texture, None, Some(target)) {
        eprintln!("Could not copy texture to canvas: {}", e);
    }
}