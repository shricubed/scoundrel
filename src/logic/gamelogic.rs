use super::{
    card::{Card, Rank, Suit},
    deck::Deck,
    room::Room,
}

use lazy_static::lazy_static;

pub struct GameState {
    pub deck: Deck,
    pub room: Room,
    pub discard: Vec<Card>,
    pub health: u8,
    pub num_disc: u8,
    pub curr_hand: Option<Card>,
    pub skipped: bool,
    pub used_potion: bool,
}



impl Card {
    fn val(&self) -> u8 {
        match self.rank {
            Rank::Ace => 14,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

lazy_static! {
    static ref dungeon: Vec<Card> = vec![];
    static ref diamonds: Vec<Card> = (2..10)
        .map(|i| Card {
            suit: Suit::Diamonds,
            rank: Rank::from(i),
        })
        .collect();
    static ref hearts: Vec<Card> = (2..10)
        .map(|i| Card {
            suit: Suit::Hearts,
            rank: Rank::from(i),
        })
        .collect();
    static ref spades: Vec<Card> = (2..14)
        .map(|i| Card {
            suit: Suit::Spades,
            rank: Rank::from(i),
        })
        .collect();
    static ref clubs: Vec<Card> = (2..14)
        .map(|i| Card {
            suit: Suit::Clubs,
            rank: Rank::from(i),
        })
        .collect();

    dungeon.append(&mut diamonds.clone());
    dungeon.append(&mut hearts.clone());
    dungeon.append(&mut spades.clone());
    dungeon.append(&mut clubs.clone());

}

impl GameState {
    pub fn start_game<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut deck = Deck::from(dungeon.clone());
        deck.shuffle(rng);
        let mut game_state = GameState {
            deck,
            room: Room::empty(),
            discard: vec![],
            health: 20,
            num_disc: 0,
            curr_hand: None,
            skipped: false,
            used_potion: false,
        };
        game_state.room.build_room(&mut game_state.deck);
        game_state


    }

    pub fn skip(&mut self) {
        if self.skipped {
            return;
        }

        let mut room = self.room.clear();
        self.deck.put_back(&mut room);

        self.room.build_room(&mut self.deck);
        self.skipped = true;

    }



}

