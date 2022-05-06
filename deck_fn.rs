use rand::Rng;
use std::io::*;
use rs_poker::core::{Hand, Rank, Rankable};
use rs_poker::core::Deck;




#[derive(Clone)]

pub struct Player{
    pub cards: Hand,
    name: String,
    chips: i32,
    pub playing: bool,
    pub eligable: bool,
    pub card1: String,
    pub card2: String,
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
            card1: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
            card2: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        };
        player.push(p);
    }
    return player;
}


pub fn deal2(d: &mut Deck, h: &mut Hand) {
    let mut rng = rand::thread_rng();
    let loc = rng.gen_range(1..d.len());
    let mut count = 1;
    for card in d.iter() {
        if count == loc {
            h.push(*card);
        }
        count += 1;
    }
    for card in h.iter() {
            d.remove(*card);
    }
}

pub fn display_player(hand: &Hand) {
    println!("");
    println!(" -----      -----");
    println!("|{}    |    |{}    |", hand[0].suit.to_char(), hand[1].suit.to_char());
    println!("|  {}  |    |  {}  |", hand[0].value.to_char(), hand[1].value.to_char());
    println!("|    {}|    |    {}|", hand[0].suit.to_char(), hand[1].suit.to_char());
    println!(" -----      -----");
}

pub fn river_prep(hands: &mut Vec<Player>, river: Hand) {
    for hand in hands {
        for card in river.iter() {
            hand.cards.push(*card);
        }
    }
}

pub fn finish(hands: &mut Vec<Player>, river: Hand, pot: &mut i32) {
    river_prep(hands, river);
    let mut l = 0;
    let mut result: Rank = Rank::HighCard(0);
    let mut n:String = "hey".to_string();
    let mut h = Hand::default();
    for (pos,hand) in hands.clone().iter().enumerate() {
        if hand.eligable {
            if hand.cards.rank() > result {
                result = hand.cards.rank();
                l = pos;
                h = hand.clone().cards;
                n = hand.name.to_string();
            }
        }
    }
    print!("{} is the winner! You win {} dollars", n.to_string(), pot);
    display_player(&h);
    for (pos,hand) in hands.iter_mut().enumerate() {
        if pos == l {
            hand.chips += *pot;
        }
    }
    println!("");
    hands[0].playing = true;
    hands[0].eligable = true;
    
    *pot = 0;

}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn players_remaining(players: &mut Vec<Player>) -> i32 {
    let mut player_count:i32 = 0;
    for player in players {
        if player.playing {
            player_count += 1;
        }
    }
    return player_count;
}

pub fn round(hands: &mut Vec<Player>, pot: &mut i32) {
    let cur_bet = 0;
        println!("The board");
        auto_bet(hands, cur_bet, pot, "".to_string());
        
}

pub fn auto_bet(hands: &mut Vec<Player>, cur_bet: i32, pot: &mut i32, raiser:String) {
    let mut count = 0;
    let mut b = cur_bet;
    let size = hands.len();
    let mut l = "".to_string();
    
    for (pos, hand) in hands.clone().iter().enumerate() {
        if hand.name == raiser && cur_bet == b{ return; }
        if hand.playing {
            if hand.name != hands[0].name {
                let mut rng = rand::thread_rng();
                let loc: i32 = rng.gen_range(1..4);
                match loc {
                    1 => {
                        hands[pos].playing = false;
                        hands[pos].eligable = false;
                        println!("{} has folded, {} chips remaining", hand.name.to_string(), hands[pos].chips); 
                    }
                    2 => {
                        if hands[pos].chips >= b {
                            *pot += b;
                            hands[pos].chips -= b;
                            println!("{} has called, {} chips remaining", hand.name.to_string(), hands[pos].chips);
                        } else {
                            hands[pos].playing = false;
                            hands[pos].eligable = false;
                            println!("{} has folded, {} chips remaining", hand.name.to_string(), hands[pos].chips);
                        }
                        
                    }
                    3 => {
                        let old = b;
                        if hands[pos].chips >= b  {
                            b = ((b as f32) * 1.25) as i32;
                            if b >= hand.chips {
                                b = hand.chips;
                                hands[pos].playing = false;
                                hands[pos].chips = 0;
                            } else {
                                l = hand.clone().name;
                                hands[pos].chips -= b;
                            }
                            *pot += b;
                            if b != old {
                                println!("{} has raised the bet to {}, {} chips remaining", hand.name.to_string(), &b, hands[pos].chips);
                            }
                        } else {
                            hands[pos].playing = false;
                            hands[pos].eligable = false;
                            println!("{} has folded, {} chips remaining", hand.name.to_string(), hands[pos].chips);
                        }
                        
                    }
                    _ => {}
                }
            } else {
                display_player(&hand.cards);
                println!("{} dollars remaining: {} to call, -1 to fold, 0 to check: ", hand.chips, b);
                let mut bet = get_input().trim().parse::<i32>().unwrap();
                if bet == -1 {
                    hands[pos].playing = false;
                    hands[pos].eligable = false;
                    println!("{} has folded", hand.name);
                } else {
                    if cur_bet > hands[pos].chips {
                        println!("Go all in or fold");
                        bet = get_input().trim().parse::<i32>().unwrap();
                    } else if bet < b {
                        while bet < b {
                            println!("The minimum bet is {}, try again", b);
                            bet = get_input().trim().parse::<i32>().unwrap();
                        }
                    } else if bet > hand.chips {
                        println!("You do not have that many chips, try again or go all in");
                        bet = get_input().trim().parse::<i32>().unwrap();
                    }
                    if bet == hand.chips {
                        println!("{} Has gone all in", hand.name);
                        hands[pos].playing = false;
                    }
                    if bet != b {
                        println!("The curernt highest is {:?}", l);
                        l = hand.clone().name;
                        b = bet;
                        println!("The new highest is {:?}", l);

                    }
                    *pot += bet;
                    hands[pos].chips -= bet;
                }
                count += 1;
            }
        }
    }
    let name = l.clone().to_string();
    if (count == size || l == raiser) && (cur_bet == b) { return; } 
    

    else { auto_bet(hands, b, pot, name.to_string()); }
}