use crate::deck_fn::{deal2, give_starting, create_players, finish, round, Player};

use rs_poker::core::{Hand};


//use std::ptr::null;
use rs_poker::core::Deck;
mod deck_fn;

// use crate::deck_fn::displayHand;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
pub fn main() {
   

    println!("How many people are playing?");
    let amount = get_input().trim().parse::<i32>().unwrap();
    assert!(amount > 0 && amount <= 8);

    let mut players = create_players(amount);

    while players.len() != 1 {
        println!("Starting a new round now!");
        println!("-----------------------------------------------------");
        let mut deck = Deck::default();
        let mut pot: i32 = 0;
    
        let mut river = Hand::default();
    
        deal2(&mut deck,&mut river);
        deal2(&mut deck,&mut river);
        deal2(&mut deck,&mut river);
        give_starting(&mut deck, &mut players);

        round(&mut players, &mut pot, river.clone());
        deal2(&mut deck,&mut river);
        round(&mut players, &mut pot, river.clone());
        deal2(&mut deck,&mut river);
        round(&mut players, &mut pot, river.clone());
        finish(&mut players, river, pot);
        for (loc,player) in players.clone().iter_mut().enumerate() {
            if player.clone().get_chips() <= 0 {
                println!("{} has ran out of chips!", player.clone().get_name());
                if loc >= players.len() {
                    players.remove(players.len() - 1);
                } else {
                    players.remove(loc);
                }
            }
        }
        for player in players.iter_mut() {
            player.playing = true;
            player.eligable = true;
        }
        if players.len() == 1 {
            for p in players.clone() {
                println!("{} is the winner!!!! Better luck next time boyz", p.get_name());
            }
        }
    }

}


