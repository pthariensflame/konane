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

use std::{collections, env, io};

extern crate konane;
use konane::{Game, Occupancy};
use konane::Position as Pos;

extern crate uuid;
use uuid::*;

#[macro_use]
extern crate error_chain;

extern crate gfx_device_gl;
extern crate piston_window;
extern crate sprite;
extern crate drag_controller;
use gfx_device_gl::{Factory as GLFactory, Resources as GLResources};
use piston_window::*;
use sprite::*;
use drag_controller::*;

#[macro_use]
extern crate clap;

extern crate rand;
use rand::{Rng, StdRng};

fn main() {
  let mut clap_app = clap::App::new("kōnane")
    .version(crate_version!())
    .author(crate_authors!())
    .about("The ancient polynesian game of kōnane")
    .arg(clap::Arg::with_name("generate bash completions")
      .short("G")
      .long("gen-bash-completions")
      .help("Generate a bash completion file to standard output"))
    .setting(clap::AppSettings::ColoredHelp);
  let matches = clap_app.clone().get_matches();
  if matches.is_present("generate bash completions") {
    clap_app.gen_completions_to(env::args().nth(0).expect("no executable name found"),
                                clap::Shell::Bash,
                                &mut io::stdout());
  } else {
    setup(matches).expect("kōnane encountered an error");
  }
}

mod errors {
  error_chain! {
    types {
      Error, ErrorKind, ChainErr, Result;
    }
    links {
      ::konane::errors::Error, ::konane::errors::ErrorKind, Game;
    }
    foreign_links {
      ::clap::Error, Clap, "clap error";
      ::uuid::ParseError, UUIDParse, "UUID parse error";
      ::std::io::Error, IO, "I/O error";
    }
    errors {
      PistonGlyph(inner: ::piston_window::GlyphError) {
        description("Piston engine glyph error")
        display("Piston engine glyph error: {:?}", inner)
      }
    }
  }

  impl From<::piston_window::GlyphError> for Error {
    fn from(inner: ::piston_window::GlyphError) -> Error { ErrorKind::PistonGlyph(inner).into() }
  }
}

struct GameContext<'a> {
  args: clap::ArgMatches<'a>,
  window: &'a mut PistonWindow,
  drag_ctrl: &'a mut DragController,
  scene: &'a mut Scene<Texture<GLResources>>,
  sprite_ids: &'a mut SpriteIDs,
  game: &'a mut Game,
  rng: &'a mut StdRng,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash,Default)]
struct SpriteIDs {
}

fn setup(matches: clap::ArgMatches) -> errors::Result<()> {
  let rng = &mut try!(StdRng::new());
  let mut cxt = GameContext {
    args: matches,
    window: &mut try!(WindowSettings::new("kōnane", [1000, 1000]).exit_on_esc(true).build()),
    drag_ctrl: &mut DragController::new(),
    scene: &mut Scene::new(),
    sprite_ids: &mut SpriteIDs::default(),
    game: &mut if rng.gen() { Game::new_white() } else { Game::new_black() },
    rng: rng,
  };
  try!(setup_scene(&mut cxt));
  run(cxt)
}

fn setup_scene(cxt: &mut GameContext) -> errors::Result<()> { Ok(()) }

fn run(cxt: GameContext) -> errors::Result<()> {
  let mut events = cxt.window.events();
  while let Some(event) = events.next(cxt.window) {
    cxt.scene.event(&event);
  }
  Ok(())
}
