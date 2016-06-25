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

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Position {
  White,
  Black,
  Empty,
}

impl Position {
  pub fn is_occupied(&self) -> bool {
    self != &Position::Empty
  }

  pub fn is_empty(&self) -> bool {
    self == &Position::Empty
  }

  pub fn is_white(&self) -> bool {
    self == &Position::White
  }

  pub fn is_black(&self) -> bool {
    self == &Position::Black
  }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub struct Papamu {
  board: [[Position; 10]; 10],
}

impl Default for Papamu {
  fn default() -> Self {
    let mut board = [[Position::White; 10]; 10];
    for x in 0..9 {
      for y in 0..9 {
        if (x + y) % 2 == 0 {
          board[x][y] = Position::Black;
        }
      }
    }
    Papamu {
      board: board,
    }
  }
}

impl Index<(u8, u8)> for Papamu {
  type Output = Position;
  
  fn index(&self, (x, y): (u8, u8)) -> &Position {
    assert!(x >= 1);
    assert!(x <= 10);
    assert!(y >= 1);
    assert!(y <= 10);
    &self.board[(x - 1) as usize][(y - 1) as usize]
  }
}
