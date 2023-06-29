use std::collections::HashMap;

use ggez::graphics::Image as GImage;
use ggez::graphics::ImageFormat;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameResult;
use texture_packer::exporter::ImageExporter;
use texture_packer::texture::Texture;
use texture_packer::TexturePacker;
use texture_packer::TexturePackerConfig;

pub struct TextureAtlasBuilder<H: std::hash::Hash> {
    pub images: HashMap<H, GImage>,
    pub packer_conf: TexturePackerConfig,
}

impl Default for TextureAtlasBuilder<String> {
    fn default() -> Self {
        let config = TexturePackerConfig {
            max_width: 1024,
            max_height: 1024,
            allow_rotation: false,
            texture_outlines: false,
            border_padding: 2,
            texture_padding: 2,
            ..Default::default()
        };
        Self {
            packer_conf: config,
            images: HashMap::default(),
        }
    }
}

pub struct TextureAtlas<H: std::hash::Hash> {
    pub image: GImage,
    pub size: mint::Point2<u32>,
    pub textures: HashMap<H, Rect>,
}

impl<H: std::hash::Hash + std::cmp::Eq + std::clone::Clone> TextureAtlasBuilder<H> {
    pub fn add_texture(&mut self, hash: H, image: GImage) {
        self.images.insert(hash, image);
    }

    pub fn build(&mut self, ctx: &mut Context) -> GameResult<TextureAtlas<H>> {
        let mut packer = TexturePacker::new_skyline(self.packer_conf);
        for (hash, image) in self.images.iter() {
            let pixels = image.to_pixels(ctx)?;
            let img = image::DynamicImage::ImageRgba8(
                image::RgbaImage::from_raw(image.width(), image.height(), pixels).unwrap(),
            );
            packer.pack_own(hash.clone(), img).unwrap();
        }
        let mut textures = HashMap::default();
        for (hash, frame) in packer.get_frames() {
            let rect = Rect {
                x: frame.frame.x as f32,
                y: frame.frame.y as f32,
                w: frame.frame.w as f32,
                h: frame.frame.h as f32,
            };
            textures.insert(hash.clone(), rect);
        }

        let exporter = ImageExporter::export(&packer).unwrap();
        let final_img = exporter.into_rgba8();

        Ok(TextureAtlas {
            image: GImage::from_pixels(
                ctx,
                final_img.into_raw().as_slice(),
                ImageFormat::Rgba8UnormSrgb,
                packer.width(),
                packer.height(),
            ),
            size: ggez::glam::UVec2::new(packer.width(), packer.height()).into(),
            textures,
        })
    }
}
