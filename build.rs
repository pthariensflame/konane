// build.rs
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

#[cfg(feature = "c_api")]
extern crate cheddar;

fn main() { handle_c_api(); }

#[cfg(not(feature = "c_api"))]
fn handle_c_api() {}

#[cfg(feature = "c_api")]
fn handle_c_api() {
  cheddar::Cheddar::new()
    .expect("unable to read cargo manifest")
    .module("c_api")
    .expect("malformed header path")
    .run_build(std::env::join_paths(&[std::env::var("OUT_DIR").expect("could not find output directory"),
                                      "include/konane.h".to_string()])
                 .expect("failure to join paths"));
}
