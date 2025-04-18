use super::{
    card::Card,
    deck::Deck
};

#[derive(PartialEq, Debug)]
pub struct Room {
    encounters: [Option<Card>; 4],
}

impl Room {
    pub fn new() -> Self {
        Room {
            encounters: [None, None, None, None],
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Card>> {
        self.encounters.iter()
    }

    pub fn clear(&mut self) -> Vec<Card> {
        let temp: Vec<Card> = self.encounters.into_iter().collect::Option<Vec<Card>>()
            .unwrap_or_default();
        self.encounters = [None, None, None, None];
        temp
    }

    pub fn build_room(&mut self, deck: &mut Deck) {
        for i in 0..self.encounters.len() {
            if self.encounters[i].is_none() {
                self.encounters[i] = deck.draw()
            }
        }
    }

    pub fn take(&mut self, i: usize) -> Option<Card> {
        let c = self.encounters[i];
        self.encounters[i] = None;
        c
    }
}
