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

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum Position {
  White,
  Black,
  Empty,
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

impl Ix {
  pub fn mk(x: u8, y: u8) -> Option<Ix> { if x <= 9 && y <= 9 { Some(Ix { x: x, y: y }) } else { None } }

  pub fn x(&self) -> u8 { self.x }

  pub fn y(&self) -> u8 { self.y }
}

impl From<Ix> for (u8, u8) {
  fn from(ix: Ix) -> (u8, u8) { (ix.x, ix.y) }
}

impl Index<Ix> for Papamu {
  type Output = Position;

  fn index(&self, ix: Ix) -> &Position { &self.board[usize::from(ix.x)][usize::from(ix.y)] }
}

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

impl<Tn: turn::Turn> Index<Ix> for Game<Tn> {
  type Output = Position;

  fn index(&self, ix: Ix) -> &Position { &self.papamu[ix] }
}
