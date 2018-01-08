extern crate luffar_lib;
extern crate piston_window;
extern crate opengl_graphics;

use luffar_lib::{GameState, Player};
use piston_window::{rectangle, Context, PistonWindow, EventLoop, Input, WindowSettings, OpenGL, Button, Event, Loop, Transformed, ButtonState, MouseButton, Motion};
use opengl_graphics::GlGraphics;

fn render(context: Context, graphics: &mut GlGraphics, state: &GameState) {
  let black = [0.0, 0.0, 0.0, 1.0];
  let white = [1.0, 1.0, 1.0, 1.0];
  let red = [5.0, 0.0, 0.0, 1.0];
  let green = [0.0, 0.5, 0.0, 1.0];
  let cell_size = 100.0;

  piston_window::clear(black, graphics);
  //println!("render start, width: {}, height: {}", state.board.len(), state.board[0].len());

  for y in 0..state.board.len() {
    for x in 0..state.board[y].len() {
      //println!("y: {}, x: {}", y, x);
      let cell = state.board[y][x];
      let color = match cell {
        None => white,
        Some(Player::Circle) => red,
        Some(Player::Cross) => green
      };
      let x_pos = x as f64 * (cell_size + 1.0);
      let y_pos = y as f64 * (cell_size + 1.0);
      rectangle(color, [0.0, 0.0, cell_size, cell_size], context.transform.trans(x_pos, y_pos), graphics);
    }
  }
}

fn main() {
  let mut state = luffar_lib::start(3, 3);

  let opengl_version = OpenGL::V3_2;
  let mut window: PistonWindow = WindowSettings::new("Luffarschack", [1024, 1024])
    .opengl(opengl_version)
    .samples(8)
    .exit_on_esc(true)
    .build()
    .unwrap();
  
  window.set_ups(0);
  window.set_max_fps(10);

  let mut graphics = GlGraphics::new(opengl_graphics::OpenGL::V3_2);

  let mut mouse_x = 0.0;
  let mut mouse_y = 0.0;

  while let Some(event) = window.next() {
    match event {
      Event::Loop(Loop::Render(render_args)) => {
        graphics.draw(render_args.viewport(), |c, g| render(c, g, &state));
      },
      Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
        mouse_x = x;
        mouse_y = y;
      }
      Event::Input(Input::Button(input)) => {
        if input.state == ButtonState::Release {
          match input.button {
            Button::Mouse(MouseButton::Left) => {
              let cell_x = (mouse_x / 100.0).floor() as usize;
              let cell_y = (mouse_y / 100.0).floor() as usize;

              println!("mouse_x: {}, mouse_y: {}, x: {}, y: {}", mouse_x, mouse_y, cell_x, cell_y);

              state = luffar_lib::do_turn(state, cell_y, cell_x);

              println!("state: {:?}", state);
            },
            _ => {}
          }
        }
      },
      _ => {}
    };
  }
}
