use glam::Vec2;
use ggez::graphics;
use ggez::{GameResult, Context};
use rand;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Walls
{
    walls: Vec<WallPair>
}

impl Walls
{
    pub fn new(capacity: usize) -> Self
    {
        return Walls 
        {
            walls: Vec::with_capacity(capacity)
        };
    }

    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        for (i, pair) in &mut self.walls.iter_mut().enumerate()
        {
            pair.update(ctx, dt)?;
        }

        if !self.walls.is_empty()
        {
            if self.walls[0].get_x() < -self.walls[0].get_width()
            {
                self.walls.remove(0);
            }
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult
    {
        for pair in &self.walls
        {
            pair.draw(ctx)?;
        }
        Ok(())
    }

    pub fn spawn(&mut self, ctx: &mut Context, gap: f32, width: f32) -> GameResult
    {
        self.walls.push(WallPair::new(ctx, gap, width)?);
        Ok(())
    }
}


pub struct WallPair
{
    dx: f32,
    rect_upper: graphics::Rect,
    rect_lower: graphics::Rect,
    mesh_upper: graphics::Mesh,
    mesh_lower: graphics::Mesh,
}

impl WallPair
{
    pub fn new(ctx: &mut Context, gap: f32, width: f32) -> GameResult<WallPair>
    {
        let ratio = rand::random::<f32>() * 0.6 + 0.2;
        let upper_height = ratio * WINDOW_HEIGHT - gap / 2.0;
        let lower_height = (1.0 - ratio) * WINDOW_HEIGHT - gap / 2.0;

        let mut rect1 = graphics::Rect::new(-width / 2.0, -upper_height / 2.0, width, upper_height);
        let mesh1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect1,
            graphics::Color::RED
        )?;
        rect1.x = WINDOW_WIDTH;
        rect1.y = 0.0 + upper_height / 2.0;

        let mut rect2 = graphics::Rect::new(-width / 2.0, -lower_height / 2.0, width, lower_height);
        let mesh2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect2,
            graphics::Color::RED
        )?;
        rect2.x = WINDOW_WIDTH;
        rect2.y = WINDOW_HEIGHT - lower_height / 2.0;

        return Ok(WallPair {
            dx: 5.0,
            rect_upper: rect1,
            rect_lower: rect2,
            mesh_upper: mesh1,
            mesh_lower: mesh2
        });
    }

    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        self.rect_upper.x -= self.dx;
        self.rect_lower.x -= self.dx;

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult
    {
        graphics::draw(ctx, &self.mesh_upper, ([self.rect_upper.x, self.rect_upper.y],))?;
        graphics::draw(ctx, &self.mesh_lower, ([self.rect_lower.x, self.rect_lower.y],))?;
        Ok(())
    }
}

//getters
impl WallPair
{
    pub fn get_x(&self) -> f32
    {
        return self.rect_upper.x;
    }

    pub fn get_width(&self) -> f32
    {
        return self.rect_upper.w;
    }
}