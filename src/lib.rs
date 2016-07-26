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

#![allow(unknown_lints, block_in_if_condition_stmt)]

use std::marker::PhantomData;
use std::borrow::Borrow;
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
      IllegalTarget(source_occ: ::Occupancy,
                    source_pos: ::Position,
                    target_pos: ::Position) {
        description("Cannot legally move indicated piece to target position")
        display("Cannot move {} piece at source {} to target {}",
                source_occ, source_pos, target_pos)
      }
      OcuppiedTarget(target_pos: ::Position) {
        description("Target position is not empty")
        display("Target {} is not empty",
                target_pos)
      }
      EmptySource(source_pos: ::Position) {
        description("Source position is empty")
        display("Source {} is not empty",
                source_pos)
      }
      WrongColor(source_occ: ::Occupancy,
                 source_pos: ::Position,
                 desired_occ: ::Occupancy) {
        description("Cannot legally move indicated piece during this turn")
        display("Cannot move {} piece at source {} during {} turn",
                source_occ, source_pos, desired_occ)
      }
      NoTargets(source_occ: ::Occupancy,
                source_pos: ::Position) {
        description("No target positions were given")
        display("No target positions were given for the {} piece at source {}",
                source_occ, source_pos)
      }
      IllegalJump(source_occ: ::Occupancy,
                  source_pos: ::Position,
                  mid_occ: ::Occupancy,
                  mid_pos: ::Position,
                  target_pos: ::Position) {
        description("Cannot perform the indicated jump")
        display("Cannot jump the {} piece at {} over the currently-{} {} to {}",
                source_occ, source_pos, mid_occ, mid_pos, target_pos)
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
    fn two_apart(a: u8, b: u8) -> bool { (a < b && b - a == 2) || (b < a && a - b == 2) }
    if (self.y == other.y && two_apart(self.x, other.x)) ||
       (self.x == other.x && two_apart(self.y, other.y)) {
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

pub struct GameState<Tn: turn::Turn> {
  papamu: Papamu,
  phantom_turn: PhantomData<Tn>,
}

impl<Tn: turn::Turn> fmt::Debug for GameState<Tn> {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    fmtr.debug_struct("Game").field("papamu", &self.papamu).field("phantom_turn", &self.phantom_turn).finish()
  }
}

impl<Tn: turn::Turn> Default for GameState<Tn> {
  fn default() -> GameState<Tn> {
    GameState {
      papamu: Papamu::default(),
      phantom_turn: PhantomData,
    }
  }
}

#[allow(unknown_lints, expl_impl_clone_on_copy)]
impl<Tn: turn::Turn> Clone for GameState<Tn> {
  fn clone(&self) -> GameState<Tn> { *self }
}

impl<Tn: turn::Turn> PartialEq for GameState<Tn> {
  fn eq(&self, other: &GameState<Tn>) -> bool { self.papamu == other.papamu }

  fn ne(&self, other: &GameState<Tn>) -> bool { self.papamu != other.papamu }
}

impl<Tn: turn::Turn> Eq for GameState<Tn> {}

impl<Tn: turn::Turn> Copy for GameState<Tn> {}

impl<Tn: turn::Turn> hash::Hash for GameState<Tn> {
  fn hash<H: hash::Hasher>(&self, hshr: &mut H) {
    self.papamu.hash(hshr);
    self.phantom_turn.hash(hshr);
  }
}

impl<Tn: turn::Turn> ops::Index<Position> for GameState<Tn> {
  type Output = Occupancy;

  fn index(&self, ix: Position) -> &Occupancy { &self.papamu[ix] }
}

impl<Tn: turn::Turn> GameState<Tn> {
  pub fn papamu(&self) -> &Papamu { &self.papamu }

  fn next_subturn(&mut self, current: Position, target: Position) -> errors::Result<()> {
    if self[target].is_occupied() {
      try!(Err(errors::ErrorKind::OcuppiedTarget(target)))
    }
    if let Some(mid) = current.biadjacency(target) {
      if self[mid] != <Tn::Next as turn::Turn>::piece_type() {
        try!(Err(errors::ErrorKind::IllegalJump(self[current], current, self[mid], mid, target)))
      }
      self.papamu[current] = Occupancy::Empty;
      self.papamu[mid] = Occupancy::Empty;
      self.papamu[target] = Tn::piece_type();
    } else {
      try!(Err(errors::ErrorKind::IllegalTarget(self[current], current, target)))
    }
    Ok(())
  }

  pub fn next_turn<Ts: IntoIterator>(&self,
                                     source: Position,
                                     targets: Ts)
                                     -> errors::Result<GameState<Tn::Next>>
    where Ts::Item: Borrow<Position> {
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
      target = *target_ref.borrow();
      try!(game.next_subturn(current, target));
      current = target;
    }
    if targets_empty {
      try!(Err(errors::ErrorKind::NoTargets(self[source], source)))
    }
    Ok(GameState {
      papamu: game.papamu,
      phantom_turn: PhantomData,
    })
  }

  pub fn can_move(&self) -> bool {
    fn gen_adj(x0: u8, y0: u8) -> Vec<((u8, u8), (u8, u8))> {
      let mut x1_vec = Vec::<u8>::new();
      let mut y1_vec = Vec::<u8>::new();
      let mut x2_vec = Vec::<u8>::new();
      let mut y2_vec = Vec::<u8>::new();
      if x0 > 1 {
        x1_vec.push(x0 - 1);
        x2_vec.push(x0 - 2);
      }
      if x0 < 9 {
        x1_vec.push(x0 + 1);
        x2_vec.push(x0 + 2);
      }
      if y0 > 1 {
        y1_vec.push(y0 - 1);
        y2_vec.push(y0 - 2);
      }
      if y0 < 9 {
        y1_vec.push(y0 + 1);
        y2_vec.push(y0 + 2);
      }
      x1_vec.into_iter()
            .map(|x1| (x1, y0))
            .chain(y1_vec.into_iter().map(|y1| (x0, y1)))
            .zip(x2_vec.into_iter().map(|x2| (x2, y0)).chain(y2_vec.into_iter().map(|y2| (x0, y2))))
            .collect()
    }
    for x0 in 0..10u8 {
      for y0 in 0..10u8 {
        for ((x1, y1), (x2, y2)) in gen_adj(x0, y0) {
          if self[Position::new(x2, y2).unwrap_or_else(|| unreachable!())].is_empty() &&
             self[Position::new(x1, y1).unwrap_or_else(|| unreachable!())] ==
             <Tn::Next as turn::Turn>::piece_type() {
            return true;
          }
        }
      }
    }
    false
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum Game {
  White(GameState<turn::White>),
  Black(GameState<turn::Black>),
}

impl From<GameState<turn::White>> for Game {
  fn from(game: GameState<turn::White>) -> Game { Game::White(game) }
}

impl From<GameState<turn::Black>> for Game {
  fn from(game: GameState<turn::Black>) -> Game { Game::Black(game) }
}

impl ops::Index<Position> for Game {
  type Output = Occupancy;

  fn index(&self, ix: Position) -> &Occupancy { &self.papamu()[ix] }
}

impl Game {
  pub fn by_color<R,
                  WR: Into<R>,
                  BR: Into<R>,
                  F: FnOnce(GameState<turn::White>) -> WR,
                  G: FnOnce(GameState<turn::Black>) -> BR>
    (self,
     f: F,
     g: G)
     -> R {
    match self {
      Game::White(game) => f(game).into(),
      Game::Black(game) => g(game).into(),
    }
  }

  #[allow(unknown_lints, needless_lifetimes)]
  pub fn by_color_ref<'a,
                      R,
                      WR: Into<R>,
                      BR: Into<R>,
                      F: FnOnce(&'a GameState<turn::White>) -> WR,
                      G: FnOnce(&'a GameState<turn::Black>) -> BR>
    (&'a self,
     f: F,
     g: G)
     -> R {
    match *self {
      Game::White(ref game) => f(game).into(),
      Game::Black(ref game) => g(game).into(),
    }
  }

  #[allow(unknown_lints, needless_lifetimes)]
  pub fn by_color_mut<'a,
                      R,
                      WR: Into<R>,
                      BR: Into<R>,
                      F: FnOnce(&'a mut GameState<turn::White>) -> WR,
                      G: FnOnce(&'a mut GameState<turn::Black>) -> BR>
    (&'a mut self,
     f: F,
     g: G)
     -> R {
    match *self {
      Game::White(ref mut game) => f(game).into(),
      Game::Black(ref mut game) => g(game).into(),
    }
  }

  pub fn current_player(&self) -> Occupancy { self.by_color_ref(|_| Occupancy::White, |_| Occupancy::Black) }

  pub fn new_white() -> Game { Game::White(GameState::default()) }

  pub fn new_black() -> Game { Game::Black(GameState::default()) }

  pub fn to_white(self) -> Option<GameState<turn::White>> { self.by_color(Some, |_| None) }

  pub fn to_black(self) -> Option<GameState<turn::Black>> { self.by_color(|_| None, Some) }

  pub fn as_white(&self) -> Option<&GameState<turn::White>> { self.by_color_ref(Some, |_| None) }

  pub fn as_black(&self) -> Option<&GameState<turn::Black>> { self.by_color_ref(|_| None, Some) }

  pub fn as_white_mut(&mut self) -> Option<&mut GameState<turn::White>> { self.by_color_mut(Some, |_| None) }

  pub fn as_black_mut(&mut self) -> Option<&mut GameState<turn::Black>> { self.by_color_mut(|_| None, Some) }

  pub fn papamu(&self) -> &Papamu { self.by_color_ref(GameState::papamu, GameState::papamu) }

  pub fn next_turn<Ts: IntoIterator>(&mut self, source: Position, targets: Ts) -> errors::Result<()>
    where Ts::Item: Borrow<Position> {
    *self = match *self {
      Game::White(game) => Game::Black(try!(game.next_turn(source, targets))),
      Game::Black(game) => Game::White(try!(game.next_turn(source, targets))),
    };
    Ok(())
  }

  pub fn can_move(&self) -> bool { self.by_color_ref(GameState::can_move, GameState::can_move) }
}

#[cfg(feature = "c-api")]
#[doc = "false"]
pub use c_api::*;

#[cfg(feature = "c-api")]
#[doc = "false"]
mod c_api;
