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

extern crate image as piston_image;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate piston_window;
extern crate sprite;
extern crate drag_controller;
use piston_image::GenericImage;
use gfx_device_gl::{Factory as GLFactory, Resources as GLResources};
use piston_window::*;
use sprite::*;
use drag_controller::*;

#[macro_use]
extern crate clap;

extern crate rand;
use rand::{Rng, StdRng};

const WHITE_PIECE_DATA: &'static [u8] = include_bytes!("../resources/white_piece.png");
const BLACK_PIECE_DATA: &'static [u8] = include_bytes!("../resources/black_piece.png");
const EMPTY_PIECE_DATA: &'static [u8] = include_bytes!("../resources/empty_piece.png");

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
      ::std::env::JoinPathsError, EnvJoinPaths, "path-joining environment error";
      ::piston_image::ImageError, PistonImage, "Piston engine image error";
      ::gfx_core::factory::CombinedError, GFXCombined, "GFX engine combined error";
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
  textures: SpriteTextures,
  window: &'a mut PistonWindow,
  drag_ctrl: &'a mut DragController,
  scene: &'a mut Scene<Texture<GLResources>>,
  sprite_map: &'a mut collections::HashMap<Pos, Uuid>,
  game: &'a mut Game,
  rng: &'a mut StdRng,
}

struct SpriteTextures {
  white_piece: Texture<GLResources>,
  black_piece: Texture<GLResources>,
  empty_piece: Texture<GLResources>,
}

fn setup(matches: clap::ArgMatches) -> errors::Result<()> {
  let mut window: PistonWindow = try!(WindowSettings::new("kōnane", [1000, 1000]).exit_on_esc(true).build());
  let textures = SpriteTextures {
    white_piece: try!(load_texture(WHITE_PIECE_DATA, &mut window.factory)),
    black_piece: try!(load_texture(BLACK_PIECE_DATA, &mut window.factory)),
    empty_piece: try!(load_texture(EMPTY_PIECE_DATA, &mut window.factory)),
  };
  let mut rng = try!(StdRng::new());
  let cxt = GameContext {
    args: matches,
    textures: textures,
    window: &mut window,
    drag_ctrl: &mut DragController::new(),
    scene: &mut Scene::new(),
    sprite_map: &mut collections::HashMap::new(),
    game: &mut if rng.gen() { Game::new_white() } else { Game::new_black() },
    rng: &mut rng,
  };
  setup_scene(cxt).and_then(run)
}

fn load_texture(texture_data: &[u8], factory: &mut GLFactory) -> errors::Result<Texture<GLResources>> {
  let texture_image = try!(piston_image::load_from_memory_with_format(texture_data,
                                                                      piston_image::ImageFormat::PNG))
                        .resize(100, 100, piston_image::Nearest);
  let texture_buffer = texture_image.as_rgba8().cloned().unwrap_or_else(|| texture_image.to_rgba());
  Ok(try!(Texture::from_image(factory, &texture_buffer, &TextureSettings::new())))
}

fn setup_scene(cxt: GameContext) -> errors::Result<GameContext> {
  for x in 0..10u8 {
    for y in 0..10u8 {
      if (x + y) % 2 == 0 {
      } else {
      }
    }
  }
  Ok(cxt)
}

fn run(cxt: GameContext) -> errors::Result<()> {
  let mut events = cxt.window.events();
  while let Some(event) = events.next(cxt.window) {
    cxt.scene.event(&event);
  }
  Ok(())
}
