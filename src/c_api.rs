use std::{iter, ptr, slice};

extern crate libc;

#[derive(Copy,Clone,Debug)]
#[repr(C)]
#[no_mangle]
pub struct KonaneGame(::Game);

impl From<::Game> for KonaneGame {
  fn from(game: ::Game) -> KonaneGame { KonaneGame(game) }
}

impl From<KonaneGame> for ::Game {
  fn from(game: KonaneGame) -> ::Game { game.0 }
}

#[allow(enum_variant_names)]
#[derive(Copy,Clone,Debug)]
#[repr(C)]
#[no_mangle]
pub enum KonaneOccupancy {
  KonaneOccupancyWhite,
  KonaneOccupancyBlack,
  KonaneOccupancyEmpty,
}

impl From<::Occupancy> for KonaneOccupancy {
  fn from(occupancy: ::Occupancy) -> KonaneOccupancy {
    match occupancy {
      ::Occupancy::White => KonaneOccupancy::KonaneOccupancyWhite,
      ::Occupancy::Black => KonaneOccupancy::KonaneOccupancyBlack,
      ::Occupancy::Empty => KonaneOccupancy::KonaneOccupancyEmpty,
    }
  }
}

impl From<KonaneOccupancy> for ::Occupancy {
  fn from(occupancy: KonaneOccupancy) -> ::Occupancy {
    match occupancy {
      KonaneOccupancy::KonaneOccupancyWhite => ::Occupancy::White,
      KonaneOccupancy::KonaneOccupancyBlack => ::Occupancy::Black,
      KonaneOccupancy::KonaneOccupancyEmpty => ::Occupancy::Empty,
    }
  }
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
#[no_mangle]
pub struct KonanePosition(::Position);

impl From<::Position> for KonanePosition {
  fn from(position: ::Position) -> KonanePosition { KonanePosition(position) }
}

impl From<KonanePosition> for ::Position {
  fn from(position: KonanePosition) -> ::Position { position.0 }
}

#[no_mangle]
pub extern "C" fn konane_game_create_white() -> *mut KonaneGame {
  Box::into_raw(Box::new(::Game::new_white().into()))
}

#[no_mangle]
pub extern "C" fn konane_game_create_black() -> *mut KonaneGame {
  Box::into_raw(Box::new(::Game::new_black().into()))
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_destroy(game_ref: *mut KonaneGame) {
  if game_ref.is_null() {
    let game_box = Box::from_raw(game_ref);
    drop(game_box);
  }
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_next_turn(game_ref: *mut KonaneGame,
                                               source_position_ref: *const KonanePosition,
                                               target_positions_ref: *const *const KonanePosition,
                                               target_positions_count: libc::size_t)
                                               -> bool {
  let mut result = false;
  if let Some(&initial_game) = game_ref.as_ref() {
    if let Some(&source_position) = source_position_ref.as_ref() {
      if !target_positions_ref.is_null() {
        let mut game: ::Game = initial_game.into();
        let target_positions: Box<iter::Iterator<Item = ::Position>> =
          Box::new(slice::from_raw_parts(target_positions_ref, target_positions_count as usize)
            .into_iter()
            .flat_map(|&target_position_ref: &*const KonanePosition| {
              target_position_ref.as_ref().into_iter()
            })
            .map(|&target_position: &KonanePosition| target_position.into()));
        if let Ok(_) = game.next_turn(source_position.into(), target_positions) {
          result = true;
        }
        ptr::write(game_ref, game.into());
      }
    }
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_can_move(game_ref: *const KonaneGame) -> bool {
  let mut result = false;
  if let Some(&initial_game) = game_ref.as_ref() {
    let game: ::Game = initial_game.into();
    result = game.can_move();
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_get_occupancy(game_ref: *const KonaneGame,
                                                   position_ref: *const KonanePosition)
                                                   -> KonaneOccupancy {
  let mut result = KonaneOccupancy::KonaneOccupancyEmpty;
  if let Some(&initial_game) = game_ref.as_ref() {
    if let Some(&position) = position_ref.as_ref() {
      let game: ::Game = initial_game.into();
      result = game[position.into()].into();
    }
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_get_current_player(game_ref: *const KonaneGame) -> KonaneOccupancy {
  let mut result = KonaneOccupancy::KonaneOccupancyEmpty;
  if let Some(&initial_game) = game_ref.as_ref() {
    let game: ::Game = initial_game.into();
    result = game.current_player().into();
  }
  result
}

#[no_mangle]
pub extern "C" fn konane_position_create(x: libc::uint8_t, y: libc::uint8_t) -> *mut KonanePosition {
  ::Position::new(x as u8, y as u8).map_or_else(ptr::null_mut,
                                                |position| Box::into_raw(Box::new(position.into())))
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_destroy(position_ref: *mut KonanePosition) {
  if position_ref.is_null() {
    let position_box = Box::from_raw(position_ref);
    drop(position_box);
  }
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_get_x(position_ref: *const KonanePosition) -> libc::uint8_t {
  let mut result = 10;
  if let Some(&initial_position) = position_ref.as_ref() {
    let position: ::Position = initial_position.into();
    result = position.x() as libc::uint8_t;
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_get_y(position_ref: *const KonanePosition) -> libc::uint8_t {
  let mut result = 10;
  if let Some(&initial_position) = position_ref.as_ref() {
    let position: ::Position = initial_position.into();
    result = position.y() as libc::uint8_t;
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_set_x(position_ref: *mut KonanePosition,
                                               x: libc::uint8_t)
                                               -> libc::uint8_t {
  let mut result = 10;
  if let Some(&initial_position) = position_ref.as_ref() {
    let mut position: ::Position = initial_position.into();
    result = position.set_x(x as u8).unwrap_or(10) as libc::uint8_t;
    ptr::write(position_ref, position.into());
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_set_y(position_ref: *mut KonanePosition,
                                               y: libc::uint8_t)
                                               -> libc::uint8_t {
  let mut result = 10;
  if let Some(&initial_position) = position_ref.as_ref() {
    let mut position: ::Position = initial_position.into();
    result = position.set_y(y as u8).unwrap_or(10) as libc::uint8_t;
    ptr::write(position_ref, position.into());
  }
  result
}

#[no_mangle]
pub unsafe extern "C" fn konane_position_biadjacency(position1_ref: *const KonanePosition,
                                                     position2_ref: *const KonanePosition)
                                                     -> *mut KonanePosition {
  let mut result = ptr::null_mut();
  if let Some(&initial_position1) = position1_ref.as_ref() {
    if let Some(&initial_position2) = position2_ref.as_ref() {
      let mut position1: ::Position = initial_position1.into();
      let mut position2: ::Position = initial_position2.into();
      result = position1.biadjacency(position2).map_or_else(ptr::null_mut, |position| {
        Box::into_raw(Box::new(position.into()))
      });
    }
  }
  result
}
