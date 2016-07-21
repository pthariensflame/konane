// main.rs
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

extern crate konane;
use konane::{Game, Occupancy, Position};

#[macro_use]
extern crate error_chain;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

#[macro_use]
extern crate clap;

mod errors {
  error_chain! {
    types {
      Error, ErrorKind, ChainErr, Result;
    }
    links {
      ::konane::errors::Error, ::konane::errors::ErrorKind, Game;
    }
    foreign_links {
      ::opengl_graphics::error::Error, OpenGL, "OpenGL graphics error";
    }
    errors {}
  }
}

fn main() { let gl = GlGraphics::new(OpenGL::V4_5); }
