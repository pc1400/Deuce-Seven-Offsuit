

//use crate::core::card::{Card, Suit, Value};
use std::collections::hash_set::{IntoIter, Iter};
use std::collections::HashSet;
use std::{usize, primitive};
use rs_poker::core::{Card, Rankable};
use rs_poker::core::Deck;
use rs_poker::core::Rank;
use rs_poker::core::Hand;
use rand::seq::SliceRandom;
use rand::Rng;


pub struct Player {
    cards: Hand,
    name: String,
}