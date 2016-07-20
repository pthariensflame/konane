// lib.rs
// Copyright 2016 Alexander Altman
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;
use std::{fmt, hash, ops};

#[macro_use]
extern crate error_chain;

pub mod turn {
  pub enum Black {}

  pub enum White {}

  pub trait Turn {
    type Next: Turn;

    fn piece_type() -> super::Occupancy;
  }

  impl Turn for Black {
    type Next = White;

    fn piece_type() -> super::Occupancy { super::Occupancy::Black }
  }

  impl Turn for White {
    type Next = Black;

    fn piece_type() -> super::Occupancy { super::Occupancy::White }
  }
}

pub mod errors {
  error_chain! {
      types {
        Error, ErrorKind, ChainErr, Result;
      }
      links {}
      foreign_links {}
      errors {
        IllegalTarget(source_occ: ::Occupancy, source_pos: ::Position, target_pos: ::Position) {
          description("Cannot legally move indicated piece to target position")
          display("Cannot move {} piece at source {} to target {}", source_occ, source_pos, target_pos)
        }
        OcuppiedTarget(target_pos: ::Position) {
          description("Target position is not empty")
          display("Target {} is not empty", target_pos)
        }
        EmptySource(source_pos: ::Position) {
          description("Source position is empty")
          display("Source {} is not empty", source_pos)
        }
        WrongColor(source_occ: ::Occupancy, source_pos: ::Position, desired_occ: ::Occupancy) {
          description("Cannot legally move indicated piece during this turn")
          display("Cannot move {} piece at source {} during {} turn", source_occ, source_pos, desired_occ)
        }
        NoTargets(source_occ: ::Occupancy, source_pos: ::Position) {
          description("No target positions were given")
          display("No target positions were given for the {} piece at source {}", source_occ, source_pos)
        }
      }
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum Occupancy {
  White,
  Black,
  Empty,
}

impl fmt::Display for Occupancy {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Occupancy::White => write!(fmtr, "white"),
      Occupancy::Black => write!(fmtr, "black"),
      Occupancy::Empty => write!(fmtr, "empty"),
    }
  }
}

impl Occupancy {
  pub fn is_empty(&self) -> bool { *self == Occupancy::Empty }

  pub fn is_occupied(&self) -> bool { *self != Occupancy::Empty }

  pub fn is_white(&self) -> bool { *self == Occupancy::White }

  pub fn is_nonwhite(&self) -> bool { *self != Occupancy::White }

  pub fn is_black(&self) -> bool { *self == Occupancy::Black }

  pub fn is_nonblack(&self) -> bool { *self != Occupancy::Black }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub struct Papamu {
  board: [[Occupancy; 10]; 10],
}

impl Default for Papamu {
  fn default() -> Papamu {
    let mut board = [[Occupancy::White; 10]; 10];
    for (_, pos) in board.iter_mut().enumerate().flat_map(|(x, part)| {
      part.iter_mut().enumerate().filter(move |&(y, _)| (x + y) % 2 == 0)
    }) {
      *pos = Occupancy::Black;
    }
    Papamu { board: board }
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub struct Position {
  x: u8,
  y: u8,
}

impl fmt::Display for Position {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    let (x, y) = <(char, u8)>::from(*self);
    write!(fmtr, "position {}{}", x, y)
  }
}

impl Position {
  pub fn new(x: u8, y: u8) -> Option<Position> {
    if x <= 9 && y <= 9 { Some(Position { x: x, y: y }) } else { None }
  }

  pub fn x(&self) -> u8 { self.x }

  pub fn set_x(&mut self, new_x: u8) -> Option<u8> {
    if new_x <= 9 {
      let old_x = self.y;
      self.x = new_x;
      Some(old_x)
    } else {
      None
    }
  }

  pub fn y(&self) -> u8 { self.y }

  pub fn set_y(&mut self, new_y: u8) -> Option<u8> {
    if new_y <= 9 {
      let old_y = self.y;
      self.y = new_y;
      Some(old_y)
    } else {
      None
    }
  }

  pub fn biadjacency(&self, other: Position) -> Option<Position> {
    if (self.y == other.y && self.x ^ other.x == 0b00000010) ||
       (self.x == other.x && self.y ^ other.y == 0b00000010) {
      Some(Position {
        x: (self.x + other.x) / 2,
        y: (self.y + other.y) / 2,
      })
    } else {
      None
    }
  }
}

impl From<Position> for (u8, u8) {
  fn from(ix: Position) -> (u8, u8) { (ix.x, ix.y) }
}

impl From<Position> for (char, u8) {
  fn from(ix: Position) -> (char, u8) { ((ix.x + 65) as char, ix.y) }
}

impl From<Position> for (u8, char) {
  fn from(ix: Position) -> (u8, char) { (ix.x, (ix.y + 65) as char) }
}

impl From<Position> for (char, char) {
  fn from(ix: Position) -> (char, char) { ((ix.x + 65) as char, (ix.y + 65) as char) }
}

impl ops::Index<Position> for Papamu {
  type Output = Occupancy;

  fn index(&self, ix: Position) -> &Occupancy { &self.board[usize::from(ix.x)][usize::from(ix.y)] }
}

impl ops::IndexMut<Position> for Papamu {
  fn index_mut(&mut self, ix: Position) -> &mut Occupancy {
    &mut self.board[usize::from(ix.x)][usize::from(ix.y)]
  }
}

pub struct Game<Tn: turn::Turn> {
  papamu: Papamu,
  phantom_turn: PhantomData<Tn>,
}

impl<Tn: turn::Turn> fmt::Debug for Game<Tn> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    fmtr.debug_struct("Game").field("papamu", &self.papamu).field("phantom_turn", &self.phantom_turn).finish()
  }
}

impl<Tn: turn::Turn> Default for Game<Tn> {
  fn default() -> Game<Tn> {
    Game {
      papamu: Papamu::default(),
      phantom_turn: PhantomData,
    }
  }
}

#[allow(unknown_lints, expl_impl_clone_on_copy)]
impl<Tn: turn::Turn> Clone for Game<Tn> {
  fn clone(&self) -> Game<Tn> { *self }
}

impl<Tn: turn::Turn> PartialEq for Game<Tn> {
  fn eq(&self, other: &Game<Tn>) -> bool { self.papamu == other.papamu }

  fn ne(&self, other: &Game<Tn>) -> bool { self.papamu != other.papamu }
}

impl<Tn: turn::Turn> Eq for Game<Tn> {}

impl<Tn: turn::Turn> Copy for Game<Tn> {}

impl<Tn: turn::Turn> hash::Hash for Game<Tn> {
  fn hash<H: hash::Hasher>(&self, hshr: &mut H) {
    self.papamu.hash(hshr);
    self.phantom_turn.hash(hshr);
  }
}

impl<Tn: turn::Turn> ops::Index<Position> for Game<Tn> {
  type Output = Occupancy;

  fn index(&self, ix: Position) -> &Occupancy { &self.papamu[ix] }
}

impl<Tn: turn::Turn> Game<Tn> {
  pub fn papamu(&self) -> &Papamu { &self.papamu }

  fn next_subturn(&mut self, current: Position, target: Position) -> errors::Result<()> {
    if self[target].is_occupied() {
      try!(Err(errors::ErrorKind::OcuppiedTarget(target)))
    }
    self.papamu[current] = Occupancy::Empty;
    self.papamu[target] = Tn::piece_type();
    Ok(())
  }

  pub fn next_turn<T: ops::Deref<Target = Position>, Ts: IntoIterator<Item = T>>
    (&self,
     source: Position,
     targets: Ts)
     -> errors::Result<Game<Tn::Next>> {
    let mut game = *self;
    if game[source].is_empty() {
      try!(Err(errors::ErrorKind::EmptySource(source)))
    }
    if game[source] != Tn::piece_type() {
      try!(Err(errors::ErrorKind::WrongColor(game[source], source, Tn::piece_type())))
    }
    let mut current = source;
    let mut target: Position;
    let mut targets_empty = true;
    for target_ref in targets {
      targets_empty = false;
      target = *target_ref;
      try!(game.next_subturn(current, target));
      current = target;
    }
    if targets_empty {
      try!(Err(errors::ErrorKind::NoTargets(self[source], source)))
    }
    Ok(Game {
      papamu: game.papamu,
      phantom_turn: PhantomData,
    })
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum AnyGame {
  White(Game<turn::White>),
  Black(Game<turn::Black>),
}

impl From<Game<turn::White>> for AnyGame {
  fn from(game: Game<turn::White>) -> AnyGame { AnyGame::White(game) }
}

impl From<Game<turn::Black>> for AnyGame {
  fn from(game: Game<turn::Black>) -> AnyGame { AnyGame::Black(game) }
}

impl ops::Index<Position> for AnyGame {
  type Output = Occupancy;

  fn index(&self, ix: Position) -> &Occupancy { &self.papamu()[ix] }
}

impl AnyGame {
  pub fn by_color<R,
                  WR: Into<R>,
                  BR: Into<R>,
                  F: FnOnce(Game<turn::White>) -> WR,
                  G: FnOnce(Game<turn::Black>) -> BR>
    (self,
     f: F,
     g: G)
     -> R {
    match self {
      AnyGame::White(game) => f(game).into(),
      AnyGame::Black(game) => g(game).into(),
    }
  }

  #[allow(unknown_lints, needless_lifetimes)]
  pub fn by_color_ref<'a,
                      R,
                      WR: Into<R>,
                      BR: Into<R>,
                      F: FnOnce(&'a Game<turn::White>) -> WR,
                      G: FnOnce(&'a Game<turn::Black>) -> BR>
    (&'a self,
     f: F,
     g: G)
     -> R {
    match *self {
      AnyGame::White(ref game) => f(game).into(),
      AnyGame::Black(ref game) => g(game).into(),
    }
  }

  #[allow(unknown_lints, needless_lifetimes)]
  pub fn by_color_mut<'a,
                      R,
                      WR: Into<R>,
                      BR: Into<R>,
                      F: FnOnce(&'a mut Game<turn::White>) -> WR,
                      G: FnOnce(&'a mut Game<turn::Black>) -> BR>
    (&'a mut self,
     f: F,
     g: G)
     -> R {
    match *self {
      AnyGame::White(ref mut game) => f(game).into(),
      AnyGame::Black(ref mut game) => g(game).into(),
    }
  }

  pub fn current_player(&self) -> Occupancy { self.by_color_ref(|_| Occupancy::White, |_| Occupancy::Black) }

  pub fn to_white_game(self) -> Option<Game<turn::White>> { self.by_color(Some, |_| None) }

  pub fn to_black_game(self) -> Option<Game<turn::Black>> { self.by_color(|_| None, Some) }

  pub fn as_white_game(&self) -> Option<&Game<turn::White>> { self.by_color_ref(Some, |_| None) }

  pub fn as_black_game(&self) -> Option<&Game<turn::Black>> { self.by_color_ref(|_| None, Some) }

  pub fn as_white_game_mut(&mut self) -> Option<&mut Game<turn::White>> { self.by_color_mut(Some, |_| None) }

  pub fn as_black_game_mut(&mut self) -> Option<&mut Game<turn::Black>> { self.by_color_mut(|_| None, Some) }

  pub fn papamu(&self) -> &Papamu { self.by_color_ref(Game::papamu, Game::papamu) }

  pub fn next_turn<T: ops::Deref<Target = Position>, Ts: IntoIterator<Item = T>>(&mut self,
                                                                                 source: Position,
                                                                                 targets: Ts)
                                                                                 -> errors::Result<()> {
    *self = match *self {
      AnyGame::White(game) => AnyGame::Black(try!(game.next_turn(source, targets))),
      AnyGame::Black(game) => AnyGame::White(try!(game.next_turn(source, targets))),
    };
    Ok(())
  }
}
