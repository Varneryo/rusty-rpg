extern crate ggez;
extern crate specs;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    text: graphics::Text,
    canvas: graphics::Canvas,
    frames: usize,
    draw_with_canvas: bool,
}

impl MainState {
    /* Function type? */
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        /* Figure out what is going on here with the contexts */
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World!", &font)?;
        let canvas = graphics::Canvas::with_window_size(ctx)?;

        let s = MainState {
            text,
            canvas,
            draw_with_canvas : false,
            frames: 0,
        };
        Ok(s)   /* what does this do? */
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult <()> {
        let dest_point = graphics::Point2::new(10.0, 10.0);

        if self.draw_with_canvas {
            println!("Drawing with canvas");
            graphics::set_background_color(ctx, graphics::Color::from((64,0,0,0)));
            graphics::clear(ctx);
            graphics::set_canvas(ctx, Some(&self.canvas));
            graphics::set_background_color(ctx, graphics::Color::from((255,255,255,128)));
            graphics::clear(ctx);

            graphics::draw_ex(
                ctx,
                &self.text,
                graphics::DrawParam{
                    dest: dest_point,
                    color: Some(graphics::Color::from((0,0,0,255))),
                    ..Default::default()
                },
            )?;
            graphics::set_canvas(ctx, None);
            graphics::draw_ex(
                ctx,
                &self.canvas,
                graphics::DrawParam{
                    color: Some(graphics::Color::from((255,255,255,128))),
                    ..Default::default()
                },
            )?;
        } else {
            println!("Drawing without canvas");
            graphics::set_canvas(ctx, None);
            graphics::set_background_color(ctx, graphics::Color::from((64,0,0,0)));
            graphics::clear(ctx);
            graphics::draw_ex(
                ctx,
                &self.text,
                graphics::DrawParam{
                    dest: dest_point,
                    color: Some(graphics::Color::from((0,0,0,255))),
                    ..Default::default()
                },
            )?;
         
        }
        graphics::present(ctx); /* what does this function do*/
        
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {})", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
/* switches canvas mode on keypress */
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: ggez::event::Keycode,
        _keymod: ggez::event::Mod,
        repeat: bool,
        ){
            if !repeat{
                self.draw_with_canvas = !self.draw_with_canvas;
                println!("Canvas on: {})", self.draw_with_canvas);
            }
        }
}
pub fn main() {
    println!("Hello World! Starting Main!");
    let c = conf::Conf { /*create a new Conf*/
        window_setup: conf::WindowSetup {
            samples: conf::NumSamples::Two,
            ..Default::default()
        },
        ..Default::default()
    };
/*create a new context*/
   let ctx = &mut Context::load_from_conf("helloworld","ggez",c).unwrap();

   if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path,true);
   }

   let state = &mut MainState::new(ctx).unwrap();
   if let Err(e) = event::run(ctx,state){
    println!("Error encountered:{}",e);
   } else {
    println!("Game exited cleanly.");
   }
}
