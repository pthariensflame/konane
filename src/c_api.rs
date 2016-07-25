#![allow(unknown_lints)]

#[repr(C)]
#[no_mangle]
pub struct KonaneGame(::Game);

#[allow(enum_variant_names)]
#[repr(C)]
#[no_mangle]
pub enum KonaneOccupancy {
  KonaneOccupancyWhite,
  KonaneOccupancyBlack,
  KonaneOccupancyEmpty,
}

impl From<::Occupancy> for KonaneOccupancy {
  fn from(occupancy: ::Occupancy) -> KonaneOccupancy {
    match occupancy {
      ::Occupancy::White => KonaneOccupancy::KonaneOccupancyWhite,
      ::Occupancy::Black => KonaneOccupancy::KonaneOccupancyBlack,
      ::Occupancy::Empty => KonaneOccupancy::KonaneOccupancyEmpty,
    }
  }
}

impl From<KonaneOccupancy> for ::Occupancy {
  fn from(occupancy: KonaneOccupancy) -> ::Occupancy {
    match occupancy {
      KonaneOccupancy::KonaneOccupancyWhite => ::Occupancy::White,
      KonaneOccupancy::KonaneOccupancyBlack => ::Occupancy::Black,
      KonaneOccupancy::KonaneOccupancyEmpty => ::Occupancy::Empty,
    }
  }
}

#[no_mangle]
pub extern "C" fn konane_game_create_white() -> *mut KonaneGame {
  Box::into_raw(Box::new(KonaneGame(::Game::new_white())))
}

#[no_mangle]
pub extern "C" fn konane_game_create_black() -> *mut KonaneGame {
  Box::into_raw(Box::new(KonaneGame(::Game::new_black())))
}

#[no_mangle]
pub unsafe extern "C" fn konane_game_destroy(game: *mut KonaneGame) { drop(Box::from_raw(game)); }
