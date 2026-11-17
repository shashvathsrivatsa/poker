pub mod game;
pub use game::*;

pub mod foundational;
pub use foundational::*;

pub mod utils;
pub use utils::*;

pub mod visualization;
pub use visualization::*;

pub use std::{
    array,
    cmp::Ordering,
    fmt::{ Debug, Display, Formatter },
    fs::File,
    io::{ BufReader, BufRead, Write },
};

pub use bitvec::prelude::*;
pub use itertools::Itertools;
pub use rand::{ Rng, rngs::StdRng, SeedableRng, seq::SliceRandom, thread_rng };
pub use rayon::prelude::*;
pub use thousands::Separable;

#[repr(C)]
struct hand_indexer_t([u8; 1024]); // oversized opaque, zeroed on init

unsafe extern "C" {
    fn hand_indexer_init(rounds: u32, cards_per_round: *const u8, indexer: *mut hand_indexer_t) -> bool;
    fn hand_index_last(indexer: *const hand_indexer_t, cards: *const u8) -> u64;
    fn hand_indexer_size(indexer: *const hand_indexer_t, round: u32) -> u64;
    fn hand_indexer_free(indexer: *mut hand_indexer_t);
}

