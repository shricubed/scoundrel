use std::collections::VecDeque;
use super::card::Card;
use rand::{Rng, seq::SliceRandom};

#[derive(PartialEq, Debug)]
pub struct Deck(VecDeque<Card>);

impl Deck {
    pub fn from(v: Vec<Card>) -> Self {
        Deck(VecDeque::from(v))
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop_back()
    }

    pub fn put_back(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.0.push_front(card);
        }
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.make_contiguous().shuffle(&mut rng);
    }
}
