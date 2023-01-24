#![allow(clippy::unnecessary_wraps)]
#![windows_subsystem = "windows"]

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use glam::*;
use ggez::timer;
use ggez_egui::EguiBackend;

mod bird;
use bird::Bird;

mod wallpair;
use wallpair::{WallPair, Walls};

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const g: f32 = 9.0;

struct GameState
{
    egui_backend: EguiBackend,
    state: i32,
    wall_spawn_timer: std::time::Duration,
    bird: Bird,
    walls: Walls,
}

impl GameState 
{
    fn new(ctx: &mut Context) -> GameResult<GameState> 
    {
        let bird = Bird::new(ctx, 15.0)?;
        let walls = Walls::new(10);

        let s = GameState 
        { 
            egui_backend: EguiBackend::default(),
            state: 0,
            wall_spawn_timer: std::time::Duration::new(0, 0),
            bird: bird,
            walls: walls,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState
{
    fn update(&mut self, ctx: &mut Context) -> GameResult 
    {
        //egui logic
        let egui_ctx = self.egui_backend.ctx(); 
        /*
        egui::Window::new("")
            .fixed_rect(egui::Rect {min: GUI_MIN, max: GUI_MAX})
            .collapsible(false)
            .title_bar(false)
            .show(&egui_ctx, |ui|
            {
                //score labels
                let score_player = egui::Label::new(format!("Player\n{}", self.player.score))
                                                .text_color(egui::Color32::YELLOW)
                                                .text_style(egui::TextStyle::Heading);
                ui.put(egui::Rect::from_center_size(egui::pos2(50.0, 840.0), egui::vec2(100.0, 20.0)), score_player);
                let score_ai = egui::Label::new(format!("Computer\n{}", self.ai.score))
                                            .text_color(egui::Color32::YELLOW)
                                            .text_style(egui::TextStyle::Heading);
                ui.put(egui::Rect::from_center_size(egui::pos2(WINDOW_WIDTH - 50.0, 840.0), egui::vec2(100.0, 20.0)), score_ai);

                //help text
                if self.state == 0
                {
                    let instructions = egui::Label::new("Press SPACE to start")
                                                    .text_color(egui::Color32::WHITE)
                                                    .text_style(egui::TextStyle::Heading);
                    ui.put(egui::Rect::from_center_size(egui::pos2(WINDOW_WIDTH / 2.0, 850.0), egui::vec2(200.0, 50.0)), instructions);
                }

                //fill the empty space
                ui.allocate_space(ui.available_size());
            }); 
        */

        //game logic
        let dt = timer::delta(ctx);
        //if game is running
        if self.state == 1
        {
            self.bird.update(ctx, dt)?;
            self.walls.update(ctx, dt)?;

            self.wall_spawn_timer += dt;
            if self.wall_spawn_timer.as_secs_f32() > 1.5
            {
                self.walls.spawn(ctx, 250.0, 150.0)?;
                self.wall_spawn_timer = std::time::Duration::new(0, 0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        graphics::clear(ctx, Color::BLUE);

        //draw egui
        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;

        //draw game
        self.bird.draw(ctx)?;
        self.walls.draw(ctx)?;
        
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) 
    {
        if self.state == 0
        {
            if keycode == KeyCode::Space
            {
                self.state = 1;
            }
        }
        else if self.state == 1
        {
            if keycode == KeyCode::Space
            {
                self.bird.bump(7.0);
            }
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, _x: f32, _y: f32)
    {
        self.egui_backend.input.mouse_button_down_event(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, _x: f32, _y: f32) 
    {
		self.egui_backend.input.mouse_button_up_event(button);
	}

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) 
    {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}


pub fn main() -> GameResult 
{
    let cb = ggez::ContextBuilder::new("FLAPPY COCK", "ggez")
    .window_setup(ggez::conf::WindowSetup::default().title("FLAPPY COCK").vsync(true))
    .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}


