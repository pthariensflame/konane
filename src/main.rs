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

extern crate rand;
use rand::{Rng, StdRng};

const OPENGL_VERSIONS_STR: [&'static str; 12] = ["2.0", "2.1", "3.0", "3.1", "3.2", "3.3", "4.0", "4.1",
                                                 "4.2", "4.3", "4.4", "4.5"];
const OPENGL_VERSIONS_ENUM: [OpenGL; 12] = [OpenGL::V2_0,
                                            OpenGL::V2_1,
                                            OpenGL::V3_0,
                                            OpenGL::V3_1,
                                            OpenGL::V3_2,
                                            OpenGL::V3_3,
                                            OpenGL::V4_0,
                                            OpenGL::V4_1,
                                            OpenGL::V4_2,
                                            OpenGL::V4_3,
                                            OpenGL::V4_4,
                                            OpenGL::V4_5];
fn opengl_version_str_to_enum(value: &str) -> OpenGL {
  OPENGL_VERSIONS_ENUM[OPENGL_VERSIONS_STR.binary_search(&value).expect("impossible")]
}
fn opengl_version_enum_to_str(value: OpenGL) -> &'static str {
  OPENGL_VERSIONS_STR[OPENGL_VERSIONS_ENUM.binary_search(&value).expect("impossible")]
}
const OPENGL_VERSION_DEFAULT: OpenGL = OpenGL::V3_2;

fn main() {
  let matches = clap::App::new("kōnane")
    .version(crate_version!())
    .author("Alexander Ronald Altman <alexanderaltman@me.com>")
    .about("The ancient polynesian game of kōnane")
    .arg(clap::Arg::with_name("OpenGL version")
      .long("opengl")
      .short("g")
      .default_value(opengl_version_enum_to_str(OPENGL_VERSION_DEFAULT))
      .help("The version of OpenGL to use")
      .possible_values(&OPENGL_VERSIONS_STR))
    .setting(clap::AppSettings::ColoredHelp)
    .get_matches();
  let gl_version = opengl_version_str_to_enum(matches.value_of("OpenGL version").expect("impossible"));
  setup(matches, gl_version).expect("kōnane encountered an error");
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
      ::std::io::Error, IO, "I/O error";
    }
    errors {
      OpenGL(inner: ::opengl_graphics::error::Error) {
        description("OpenGL graphics error")
        display("OpenGL graphics error: {}", inner)
      }
    }
  }

  impl From<::opengl_graphics::error::Error> for Error {
    fn from(inner: ::opengl_graphics::error::Error) -> Error { ErrorKind::OpenGL(inner).into() }
  }
}

struct Context<'a> {
  args: clap::ArgMatches<'a>,
  gl_version: OpenGL,
  window: &'a mut Window,
  gl: &'a mut GlGraphics,
  game: &'a mut Game,
  rng: &'a mut StdRng,
}

fn setup(matches: clap::ArgMatches, gl_version: OpenGL) -> errors::Result<()> {
  let rng = &mut try!(StdRng::new());
  let cxt = Context {
    args: matches,
    gl_version: gl_version,
    window: &mut try!(WindowSettings::new("kōnane", [1000, 1000])
      .opengl(gl_version)
      .exit_on_esc(true)
      .build()),
    gl: &mut GlGraphics::new(gl_version),
    game: &mut if rng.gen() { Game::new_white() } else { Game::new_black() },
    rng: rng,
  };
  run(cxt)
}

fn run(cxt: Context) -> errors::Result<()> {
  let mut events = cxt.window.events();
  while let Some(event) = events.next(cxt.window) {
    match event {
      Event::Render(params) => (),
      Event::AfterRender(params) => (),
      Event::Update(params) => (),
      Event::Idle(params) => (),
      Event::Input(params) => (),
    }
  }
  Ok(())
}
