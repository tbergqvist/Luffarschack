#[macro_use]
extern crate stdweb;
extern crate luffar_lib;

use stdweb::web::{INode, document};

use stdweb::web::event::ClickEvent;

use luffar_lib::{GameState, Player};

fn update_status_text(game_state: &GameState) {
  let status_element = document().query_selector("#status").unwrap();
    status_element.set_text_content(
      match game_state.winner {
        Some(luffar_lib::Winner::Circle) => "Game Over! O wins!",
        Some(luffar_lib::Winner::Cross) => "Game Over! X wins!",
        Some(luffar_lib::Winner::Draw) => "Game Over! No one won!",
      None => match game_state.next_player {Player::Circle => "O turn", Player::Cross => "X turn"},
    },
  );
}

fn on_click(game_state: GameState, y_pos: usize, x_pos: usize) {
  let state = luffar_lib::do_turn(game_state, y_pos, x_pos);
  update_status_text(&state);
  show_board(&state);
}

fn show_board(state: &GameState) {
  let board_element = document().query_selector("table").unwrap();
  
  while board_element.has_child_nodes() {
    board_element.remove_child(&board_element.first_child().unwrap()).unwrap();
  }
  
  for y in 0..state.board.len() {
    let tr = document().create_element("tr");
    board_element.append_child(&tr);
    for x in 0..state.board[y].len() {
      let cell = state.board[y][x];
      let td = document().create_element("td");
      td.set_text_content(match cell { Some(Player::Cross) => "X", Some(Player::Circle) => "O", None => " " });
      let cloned_state = state.clone();
      js! {
        @{td.as_ref()}.onclick = @{move |_: ClickEvent|on_click(cloned_state.clone(), y, x)};
      };

      tr.append_child(&td);
    }
  }
}

fn main() {
    stdweb::initialize();
    
    let state = luffar_lib::start(10, 5);
    show_board(&state);
    update_status_text(&state);

    stdweb::event_loop();
}