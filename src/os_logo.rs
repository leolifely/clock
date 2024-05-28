use os_info::Type;
use sdl2::render::{Texture, TextureQuery};

pub fn get_distro_logo_path() -> &'static[u8] {
    let os_info = os_info::get();
    match os_info.os_type() {
        Type::Arch => include_bytes!("assets/arch.png"),
        Type::AIX => include_bytes!("assets/aix.png"),
        Type::AlmaLinux => include_bytes!("assets/alma.png"),
        Type::Alpaquita => include_bytes!("assets/alpaquita.png"),
        Type::Alpine => include_bytes!("assets/alpine.png"),
        Type::Amazon => include_bytes!("assets/amazon.png"),
        Type::Android => include_bytes!("assets/android.png"),
        Type::Artix => include_bytes!("assets/artix.png"),
        Type::CentOS => include_bytes!("assets/centos.png"),
        Type::Debian => include_bytes!("assets/debian.png"),
        Type::DragonFly => include_bytes!("assets/dragonfly.png"),
        Type::Emscripten => include_bytes!("assets/emscripten.png"),
        Type::EndeavourOS => include_bytes!("assets/endeavour.png"),
        Type::Fedora => include_bytes!("assets/fedora.png"),
        Type::FreeBSD => include_bytes!("assets/freebsd.png"),
        Type::Garuda => include_bytes!("assets/garuda.png"),
        Type::Gentoo => include_bytes!("assets/gentoo.png"),
        Type::HardenedBSD => include_bytes!("assets/hardenedbsd.png"),
        Type::Illumos => include_bytes!("assets/illumos.png"),
        Type::Kali => include_bytes!("assets/kali.png"),
        Type::Mabox => include_bytes!("assets/mabox.png"),
        Type::Macos => include_bytes!("assets/macos.png"),
        Type::Manjaro => include_bytes!("assets/manjaro.png"),
        Type::Mariner => include_bytes!("assets/mariner.png"),
        Type::MidnightBSD => include_bytes!("assets/midnightbsd.png"),
        Type::Mint => include_bytes!("assets/mint.png"),
        Type::NetBSD => include_bytes!("assets/netbsd.png"),
        Type::NixOS => include_bytes!("assets/nixos.png"),
        Type::OpenBSD => include_bytes!("assets/openbsd.png"),
        Type::OpenCloudOS => include_bytes!("assets/opencloudos.png"),
        Type::OracleLinux => include_bytes!("assets/oracle.png"),
        Type::Pop => include_bytes!("assets/pop.png"),
        Type::Raspbian => include_bytes!("assets/raspberrypios.png"),
        Type::RedHatEnterprise => include_bytes!("assets/redhatenterpriselinux.png"),
        Type::Redhat => include_bytes!("assets/redhat.png"),
        Type::Redox => include_bytes!("assets/redox.png"),
        Type::RockyLinux => include_bytes!("assets/rocky.png"),
        Type::SUSE => include_bytes!("assets/suse.png"),
        Type::Solus => include_bytes!("assets/solus.png"),
        Type::Ubuntu => include_bytes!("assets/ubuntu.png"),
        Type::Ultramarine => include_bytes!("assets/ultramarine.png"),
        Type::Void => include_bytes!("assets/void.png"),
        Type::Windows => include_bytes!("assets/windows.png"),
        Type::openEuler => include_bytes!("assets/openeuler.png"),
        Type::openSUSE => include_bytes!("assets/opensuse.png"),
        _ => include_bytes!("assets/amazon.png"),
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