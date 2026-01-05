use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a new empty deck
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Create the Tavern deck (player deck) with specified number of Jesters
    pub fn create_tavern_deck(jester_count: u8) -> Self {
        let mut cards = Vec::new();

        // Add all numbered cards (2-10) in all suits
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in [
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
            ] {
                cards.push(Card::new(suit, rank));
            }
        }

        // Add 4 Animal Companions (Aces)
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            cards.push(Card::new(suit, Rank::Ace));
        }

        // Add Jesters
        for _ in 0..jester_count {
            cards.push(Card::new(Suit::Hearts, Rank::Jester)); // Jesters don't have suits, but we use Hearts
        }

        let mut deck = Self { cards };
        deck.shuffle();
        deck
    }

    /// Create the Castle deck (enemy deck) with proper layering:
    /// 4 Kings (bottom), 4 Queens (middle), 4 Jacks (top)
    /// Suits within each layer are randomized
    pub fn create_castle_deck() -> Self {
        let mut cards = Vec::new();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let mut rng = thread_rng();

        // Create Kings (bottom layer) - added first so they're at start of vector
        let mut kings: Vec<Card> = suits.iter().map(|&s| Card::new(s, Rank::King)).collect();
        kings.shuffle(&mut rng);
        cards.extend(kings);

        // Create Queens (middle layer)
        let mut queens: Vec<Card> = suits.iter().map(|&s| Card::new(s, Rank::Queen)).collect();
        queens.shuffle(&mut rng);
        cards.extend(queens);

        // Create Jacks (top layer) - added last so they're at end of vector and drawn first via pop()
        let mut jacks: Vec<Card> = suits.iter().map(|&s| Card::new(s, Rank::Jack)).collect();
        jacks.shuffle(&mut rng);
        cards.extend(jacks);

        Self { cards }
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draw a card from the top of the deck
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Draw multiple cards
    pub fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }

    /// Add a card to the top of the deck
    pub fn add_to_top(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Add multiple cards to the bottom of the deck
    pub fn add_multiple_to_bottom(&mut self, cards: Vec<Card>) {
        for card in cards.into_iter().rev() {
            self.cards.insert(0, card);
        }
    }

    /// Get the number of cards remaining
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
