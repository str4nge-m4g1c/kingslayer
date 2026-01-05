use crate::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub hand: Vec<Card>,
    pub max_hand_size: usize,
    pub name: String,
}

impl Player {
    pub fn new(name: String, max_hand_size: usize) -> Self {
        Self {
            hand: Vec::new(),
            max_hand_size,
            name,
        }
    }

    /// Add a card to the player's hand if not at max
    pub fn draw_card(&mut self, card: Card) -> bool {
        if self.hand.len() < self.max_hand_size {
            self.hand.push(card);
            true
        } else {
            false
        }
    }

    /// Draw multiple cards up to max hand size
    pub fn draw_multiple(&mut self, cards: Vec<Card>) -> Vec<Card> {
        let mut remaining = Vec::new();
        for card in cards {
            if !self.draw_card(card) {
                remaining.push(card);
            }
        }
        remaining
    }

    /// Remove a card from hand by index
    pub fn play_card(&mut self, index: usize) -> Option<Card> {
        if index < self.hand.len() {
            Some(self.hand.remove(index))
        } else {
            None
        }
    }

    /// Remove multiple cards from hand by indices (must be sorted in descending order)
    pub fn play_cards(&mut self, mut indices: Vec<usize>) -> Vec<Card> {
        indices.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
        let mut cards = Vec::new();
        for index in indices {
            if let Some(card) = self.play_card(index) {
                cards.push(card);
            }
        }
        cards.reverse(); // Return in original order
        cards
    }

    /// Calculate total value of cards at given indices
    pub fn calculate_value(&self, indices: &[usize]) -> u8 {
        indices
            .iter()
            .filter_map(|&i| self.hand.get(i))
            .map(|card| card.value())
            .sum()
    }

    /// Check if player can discard enough to survive damage
    pub fn can_survive(&self, damage: u8) -> bool {
        let total_value: u8 = self.hand.iter().map(|c| c.value()).sum();
        total_value >= damage
    }

    /// Get the current hand size
    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    /// Check if player is at max hand size
    pub fn is_hand_full(&self) -> bool {
        self.hand.len() >= self.max_hand_size
    }
}
