//! The simplest possible example that does something.

use ggez::{
    event,
    glam::*,
    graphics::{self, Image},
    Context, GameResult,
};
use ggez_atlas::atlas::{TextureAtlas, TextureAtlasBuilder};

struct MainState {
    pos_x: f32,
    texture_atlas: TextureAtlas,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut tb = TextureAtlasBuilder::default();
        for i in 0..4 {
            let path = format!("/tile_000{i}.png");
            let image = Image::from_path(ctx, path.clone())?;
            tb.add_texture(path, image);
        }

        let texture_atlas = tb.build(ctx)?;

        Ok(MainState {
            pos_x: 0.0,
            texture_atlas,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.texture_atlas.image, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
