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

use std::ops::Index;
use std::marker::PhantomData;
use std::fmt;

#[macro_use]
extern crate error_chain;

pub mod turn {
  pub enum Black {}

  pub enum White {}

  pub trait Turn {
    type Next: Turn;

    fn piece_type() -> super::Position;
  }

  impl Turn for Black {
    type Next = White;

    fn piece_type() -> super::Position { super::Position::Black }
  }

  impl Turn for White {
    type Next = Black;

    fn piece_type() -> super::Position { super::Position::White }
  }

  pub mod errors {
    error_chain! {
      types {
        Error, ErrorKind, ChainErr, Result;
      }
      links {}
      foreign_links {}
      errors {
        IllegalTarget(source_pos: ::Position, target_pos: ::Position) {
          description("Cannot move {} piece from ")
        }
      }
    }
  }
}

pub mod errors {
  error_chain! {
    types {
      Error, ErrorKind, ChainErr, Result;
    }
    links {
      ::turn::errors::Error, ::turn::errors::ErrorKind, Turn;
    }
    foreign_links {}
    errors {}
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum Position {
  White,
  Black,
  Empty,
}

impl fmt::Display for Position {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Position::White => write!(fmtr, "white"),
      Position::Black => write!(fmtr, "black"),
      Position::Empty => write!(fmtr, "empty"),
    }
  }
}

impl Position {
  pub fn is_occupied(&self) -> bool { self != &Position::Empty }

  pub fn is_empty(&self) -> bool { self == &Position::Empty }

  pub fn is_white(&self) -> bool { self == &Position::White }

  pub fn is_black(&self) -> bool { self == &Position::Black }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub struct Papamu {
  board: [[Position; 10]; 10],
}

impl Default for Papamu {
  fn default() -> Papamu {
    let mut board = [[Position::White; 10]; 10];
    for (_, pos) in board.iter_mut().enumerate().flat_map(|(x, part)| {
      part.iter_mut().enumerate().filter(move |&(y, _)| (x + y) % 2 == 0)
    }) {
      *pos = Position::Black;
    }
    Papamu { board: board }
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub struct Ix {
  x: u8,
  y: u8,
}

impl fmt::Display for Ix {
  fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
    let (x, y) = <(char, u8)>::from(*self);
    write!(fmtr, "position {}{}", x, y)
  }
}

impl Ix {
  pub fn mk(x: u8, y: u8) -> Option<Ix> { if x <= 9 && y <= 9 { Some(Ix { x: x, y: y }) } else { None } }

  pub fn x(&self) -> u8 { self.x }

  pub fn set_x(&mut self, new_x: u8) -> bool {
    if new_x <= 9 {
      self.x = new_x;
      true
    } else {
      false
    }
  }

  pub fn y(&self) -> u8 { self.y }

  pub fn set_y(&mut self, new_y: u8) -> bool {
    if new_y <= 9 {
      self.y = new_y;
      true
    } else {
      false
    }
  }
}

impl From<Ix> for (u8, u8) {
  fn from(ix: Ix) -> (u8, u8) { (ix.x, ix.y) }
}

impl From<Ix> for (char, u8) {
  fn from(ix: Ix) -> (char, u8) { ((ix.x + 65) as char, ix.y) }
}

impl From<Ix> for (u8, char) {
  fn from(ix: Ix) -> (u8, char) { (ix.x, (ix.y + 65) as char) }
}

impl From<Ix> for (char, char) {
  fn from(ix: Ix) -> (char, char) { ((ix.x + 65) as char, (ix.y + 65) as char) }
}

impl Index<Ix> for Papamu {
  type Output = Position;

  fn index(&self, ix: Ix) -> &Position { &self.board[usize::from(ix.x)][usize::from(ix.y)] }
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

impl<Tn: turn::Turn> Copy for Game<Tn> {}

impl<Tn: turn::Turn> Index<Ix> for Game<Tn> {
  type Output = Position;

  fn index(&self, ix: Ix) -> &Position { &self.papamu[ix] }
}

impl<Tn: turn::Turn> Game<Tn> {
  pub fn papamu(&self) -> Papamu { self.papamu }
}
