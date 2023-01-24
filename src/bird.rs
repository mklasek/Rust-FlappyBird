use glam::Vec2;
use ggez::graphics;
use ggez::{GameResult, Context};

use crate::{WINDOW_HEIGHT, g};

pub struct Bird
{
    xy: Vec2,
    dy: f32,
    radius: f32,
    mesh: graphics::Mesh,
}

//new, update, draw
impl Bird
{
    pub fn new(ctx: &mut Context, r: f32) -> GameResult<Bird> 
    {
        let mesh = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::fill(), 
            [0.0, 0.0], 
            r, 
            0.05, 
            graphics::Color::GREEN
        )?;

        return Ok(Bird 
        {
            xy: Vec2::new(100.0, WINDOW_HEIGHT / 2.0),
            dy:0.0,
            radius: r,
            mesh: mesh,
        });
    }

    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        self.dy += g * dt.as_secs_f32();
        self.xy.y += self.dy;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult
    {
        graphics::draw(ctx, &self.mesh, (self.xy,))?;
        Ok(())
    }
}

//getters
impl Bird
{
    pub fn get_xy(&self) -> Vec2
    {
        return self.xy;
    }

    pub fn get_radius(&self) -> f32
    {
        return self.radius;
    }
}


//actions
impl Bird
{
    pub fn bump(&mut self, v: f32)
    {
        self.dy = -v;
    }
}