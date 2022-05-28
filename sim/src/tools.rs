use bevy::prelude::Image;

pub fn circle() -> Image {
    let size = 100;
    let falloff = 3;
    let half = size / 2;
    let mut data = vec![0; (size * size) as usize * 4];

    for i in 0..size {
        for j in 0..size {
            let x = i - half;
            let y = j - half;

            if x * x + y * y < half * half {
                let dist = f64::from(x * x + y * y).sqrt();
                let falloff = (dist - f64::from(half - falloff)) / f64::from(falloff);
                let falloff = 1.0 - falloff.max(0.0);

                let index = (i * size + j) as usize;
                let data = &mut data[index * 4..][..4];
                data.fill((f64::from(0xff) * falloff) as u8);
            }
        }
    }

    let size = size as u32;

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
    )
}

pub fn oblique_circle() -> Image {
    let size: i32 = 100;
    let falloff = 30;
    let half = size / 2;
    let mut data = vec![0; (size * size) as usize * 4];

    for i in 0..size {
        for j in 0..size {
            let x = i - half;
            let y = j - half - 30;

            if x.abs() + y.abs() < half {
                let dist = f64::from(x.abs() + y.abs());
                let falloff = (dist - f64::from(half - falloff)) / f64::from(falloff);
                let falloff = 1.0 - falloff.max(0.0);

                let index = (i * size + j) as usize;
                let data = &mut data[index * 4..][..4];
                data.fill((f64::from(0xff) * falloff) as u8);
            }
        }
    }

    let size = size as u32;

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
    )
}
