struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn get_cards(self) {
        for c in self.cards {
            print!("{}, {}", c.get_suit(), c.get_name());
        }
    }
}