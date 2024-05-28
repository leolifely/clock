use os_info::Type;
use sdl2::render::{Texture, TextureQuery};

pub fn get_distro_logo_path() -> String {
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Arch => "assets/arch.png".to_string(),
        Type::AIX => "assets/aix.png".to_string(),
        Type::AlmaLinux => "assets/alma.png".to_string(),
        Type::Alpaquita => "assets/alpaquita.png".to_string(),
        Type::Alpine => "assets/alpine.png".to_string(),
        Type::Amazon => "assets/amazon.png".to_string(),
        Type::Android => "assets/android.png".to_string(),
        Type::Artix => "assets/artix.png".to_string(),
        Type::CentOS => "assets/centos.png".to_string(),
        Type::Debian => "assets/debian.png".to_string(),
        Type::DragonFly => "assets/dragonfly.png".to_string(),
        Type::Emscripten => "assets/emscripten.png".to_string(),
        Type::EndeavourOS => "assets/endeavour.png".to_string(),
        Type::Fedora => "assets/fedora.png".to_string(),
        Type::FreeBSD => "assets/freebsd.png".to_string(),
        Type::Garuda => "assets/garuda.png".to_string(),
        Type::Gentoo => "assets/gentoo.png".to_string(),
        Type::HardenedBSD => "assets/hardenedbsd.png".to_string(),
        Type::Illumos => "assets/illumos.png".to_string(),
        Type::Kali => "assets/kali.png".to_string(),
        Type::Mabox => "assets/mabox.png".to_string(),
        Type::Macos => "assets/macos.png".to_string(),
        Type::Manjaro => "assets/manjaro.png".to_string(),
        Type::Mariner => "assets/mariner.png".to_string(),
        Type::MidnightBSD => "assets/midnightbsd.png".to_string(),
        Type::Mint => "assets/mint.png".to_string(),
        Type::NetBSD => "assets/netbsd.png".to_string(),
        Type::NixOS => "assets/nixos.png".to_string(),
        Type::OpenBSD => "assets/openbsd.png".to_string(),
        Type::OpenCloudOS => "assets/opencloudos.png".to_string(),
        Type::OracleLinux => "assets/oracle.png".to_string(),
        Type::Pop => "assets/pop.png".to_string(),
        Type::Raspbian => "assets/raspberrypios.png".to_string(),
        Type::RedHatEnterprise => "assets/redhatenterpriselinux.png".to_string(),
        Type::Redhat => "assets/redhat.png".to_string(),
        Type::Redox => "assets/redox.png".to_string(),
        Type::RockyLinux => "assets/rocky.png".to_string(),
        Type::SUSE => "assets/suse.png".to_string(),
        Type::Solus => "assets/solus.png".to_string(),
        Type::Ubuntu => "assets/ubuntu.png".to_string(),
        Type::Ultramarine => "assets/ultramarine.png".to_string(),
        Type::Void => "assets/void.png".to_string(),
        Type::Windows => "assets/windows.png".to_string(),
        Type::openEuler => "assets/openeuler.png".to_string(),
        Type::openSUSE => "assets/opensuse.png".to_string(),
        _ => "assets/unknown.png".to_string(),
    }
}

pub fn draw_logo(logo_texture: &Texture, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.clear();
    let (window_width, window_height) = canvas.output_size().unwrap();
    let TextureQuery { width, height, .. } = logo_texture.query();

    let scale_factor_width = (window_width as f32 / 2.2) / width as f32;
    let scale_factor_height = (window_height as f32 / 4.0) / height as f32;

    // Use the smaller scale factor to maintain aspect ratio
    let scale_factor = scale_factor_width.min(scale_factor_height);

    let new_width = (width as f32 * scale_factor) as u32;
    let new_height = (height as f32 * scale_factor) as u32;


    let target = sdl2::rect::Rect::new(
        (window_width as i32 - new_width as i32) / 2,
        0,
        new_width,
        new_height,
    );
    if let Err(e) = canvas.copy(logo_texture, None, Some(target)) {
        eprintln!("Could not copy logo texture to canvas: {}", e);
    }
}