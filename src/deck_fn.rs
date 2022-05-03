use rs_poker::core::card::{Card};
use rand::Rng;
use std::io::*;
use std::ptr::null;
use rs_poker::core::{Hand, Rank, Rankable};
use rs_poker::core::Deck;

#[derive(Clone)]

pub struct Player{
    cards: Hand,
    name: String,
    chips: i32,
    pub playing: bool,
    pub eligable: bool,
}

impl Player {
    pub fn get_chips(self) -> i32 {
        return self.chips;
    }
    pub fn get_name(self) -> String {
        return self.name;
    }
}

pub fn give_starting(d: &mut Deck, players: &mut Vec<Player>) {
    for (loc, player) in players.clone().iter().enumerate() {
        if player.chips == 0 {
            players.remove(loc);
        }
    }

    for player in players.iter_mut() {
        let mut hand = Hand::default();
        deal2(d, &mut hand);
        deal2(d, &mut hand);
        player.cards = hand;
    }
}

pub fn create_players(amount: i32) -> Vec<Player> {
    let mut player = Vec::new();
    for _i in 0..amount {
        println!("Enter your name :");
        let mut input_string = String::new();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        let p = Player {
            cards: Hand::default(),
            name: input_string,
            chips: 100,
            playing: true,
            eligable: true,
        };
        player.push(p);
    }
    return player;
}


pub fn deal2(d: &mut Deck, h: &mut Hand) {
    let mut rng = rand::thread_rng();
    let loc = rng.gen_range(1..52);
    let mut count = 1;
    let mut x = Card {
        value: rs_poker::core::Value::Nine,
        suit: rs_poker::core::Suit::Spade,
    };
    for card in d.iter() {
        if count == loc {
            x = *card;
        }
        count += 1;
    }
    h.push(x);
    d.remove(x);
}

pub fn display_player(hand: &Hand, name: String) {
    print!("{}", name);
    println!(" -----      -----");
    println!("|{}    |    |{}    |", hand[0].suit.to_char(), hand[1].suit.to_char());
    println!("|  {}  |    |  {}  |", hand[0].value.to_char(), hand[1].value.to_char());
    println!("|    {}|    |    {}|", hand[0].suit.to_char(), hand[1].suit.to_char());
    println!(" -----      -----");
}

pub fn print_river3(hand: &Hand) {
    println!(" -----      -----      -----");
    println!("|{}    |    |{}    |    |{}    |", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char());
    println!("|  {}  |    |  {}  |    |  {}  |", hand[0].value.to_char(), hand[1].value.to_char(), hand[2].value.to_char());
    println!("|    {}|    |    {}|    |    {}|", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char());
    println!(" -----      -----      -----");
}

pub fn print_river4(hand: &Hand) {
    println!(" -----      -----      -----     -----");
    println!("|{}    |    |{}    |    |{}    |   |{}    |", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char(), hand[3].suit.to_char());
    println!("|  {}  |    |  {}  |    |  {}  |   |  {}  |", hand[0].value.to_char(), hand[1].value.to_char(), hand[2].value.to_char(), hand[3].value.to_char());
    println!("|    {}|    |    {}|    |    {}|   |    {}|", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char(), hand[3].suit.to_char());
    println!(" -----      -----      -----     -----");
}

pub fn print_river5(hand: &Hand) {
    println!(" -----      -----      -----     -----     -----");
    println!("|{}    |    |{}    |    |{}    |   |{}    |   |{}    |", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char(), hand[3].suit.to_char(), hand[4].suit.to_char());
    println!("|  {}  |    |  {}  |    |  {}  |   |  {}  |   |  {}  |", hand[0].value.to_char(), hand[1].value.to_char(), hand[2].value.to_char(), hand[3].value.to_char(), hand[4].value.to_char());
    println!("|    {}|    |    {}|    |    {}|   |    {}|   |    {}|", hand[0].suit.to_char(), hand[1].suit.to_char(), hand[2].suit.to_char(), hand[3].suit.to_char(), hand[4].suit.to_char());
    println!(" -----      -----      -----     -----     -----");
}

pub fn river_prep(hands: &mut Vec<Player>, river: Hand) {
    for hand in hands {
        for card in river.iter() {
            hand.cards.push(*card);
        }
    }
}

pub fn finish(hands: &mut Vec<Player>, river: Hand, pot: i32) {
    river_prep(hands, river);
    let mut l = 0;
    let mut result: Rank = Rank::HighCard(0);
    let mut n:String = "hey".to_string();
    for (pos,hand) in hands.clone().iter().enumerate() {
        if hand.eligable {
            if hand.cards.rank() > result {
                result = hand.cards.rank();
                l = pos;
                n = hand.name.to_string();
            }
        }
    }
    print!("{} is the winner! You win {} dollars", n.to_string(), pot);
    for (pos,hand) in hands.iter_mut().enumerate() {
        if pos == l {
            hand.chips += pot;
        }
    }
    println!("");
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn round(hands: &mut Vec<Player>, pot: &mut i32, river: Hand) {
    if hands.len() == 1 {
        return;
    }
    let cur_bet = 0;
        println!("The board");
        if river.len() == 3 {
            print_river3(&river);
        } else if river.len() == 4 {
            print_river4(&river);
        } else {
            print_river5(&river);
        }
        bet(hands, cur_bet, pot, 100);
        
}

pub fn bet(hands: &mut Vec<Player>, cur_bet: i32, pot: &mut i32, loc:usize) {
    let mut count = 0;
    let mut b = cur_bet;
    let size = hands.len();
    let mut l:usize = 0;
    for (pos, hand) in hands.iter_mut().enumerate() {
        if size == 1 as usize || pos == loc { return; }
        if hand.playing {
            display_player(&hand.cards, hand.clone().name);
            println!("{} dollars remaining: {} to call, -1 to fold, 0 to check: ", hand.chips, b);
            let mut bet = get_input().trim().parse::<i32>().unwrap();
            if bet == -1 {
                hand.playing = false;
                hand.eligable = false;
                println!("{} has folded - yikes", hand.name);
            } else {
                if bet > hand.chips {
                    println!("You do not have that many chips, try again or go all in");
                    bet = get_input().trim().parse::<i32>().unwrap();
                }
                if bet == hand.chips {
                    println!("{} Has gone all in", hand.name);
                    hand.playing = false;
                }
                if bet != b {
                    l = pos;
                    b = bet;
                }
                *pot += bet;
                hand.chips -= bet;
            }
            count += 1;
        }
    }
    if (count == hands.len() || l == loc) && (cur_bet == b) { return; } 

    else { bet(hands, b, pot, l); }

}
