extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate embedded_graphics;
extern crate find_folder;
extern crate touch_visualizer;
extern crate piston_window;

mod deck_fn;
use deck_fn::*;

use piston::window::WindowSettings;
use crate::piston::PressEvent;
use rs_poker::core::Deck;
use rs_poker::core::Hand;
use rs_poker::core::Suit;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings,};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, Key, Button};
use rs_poker::core::Value;
use std::collections::HashMap;
use std::path::Path;
use graphics::*;

pub struct App {
    gl: GlGraphics,
    card_image: String,
    flop1: String,
    flop2: String,
    flop3: String,
    turn: String,
    river_card: String,
    card_map: HashMap<(Suit, Value), String>,
    players: Vec<Player>,
    pot: i32,
    new_game: bool,
    deck: Deck,
    river: Hand,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        
        const GREEN: [f32; 4] = [0.0, 0.75, 0.0, 1.0];
        const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const RED: [f32; 4] = [0.5, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let table = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 375.0, 225.0));
        let outer_ring = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 385.0, 235.0));
        let carpet   = Image::new().rect(rectangle::square(0.0, 0.0, 1000.0));
        let carpet2   = Image::new().rect(rectangle::square(1000.0, 0.0, 1000.0));
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let player_count = self.players.len();

        let carpet_texture = Texture::from_path(Path::new("Playing Cards/PNG-cards-1.3/new_carpet.png"), &TextureSettings::new()).unwrap();
        let mut texture_list: Vec<(Texture, Texture)> = Vec::new();

        for player in self.players.iter() {
            if (player.clone().get_name() == self.players[0].clone().get_name() || self.new_game) && player.eligable {
                texture_list.push((Texture::from_path(Path::new(&self.card_map.get(&(player.cards[0].suit, player.cards[0].value)).unwrap().to_string()), 
            &TextureSettings::new()).unwrap(), Texture::from_path(Path::new(&self.card_map.get(&(player.cards[1].suit, player.cards[1].value)).unwrap().to_string()), &TextureSettings::new()).unwrap()));
            } else {
                texture_list.push((Texture::from_path(Path::new(&"Playing Cards/PNG-cards-1.3/back.png".to_string()), 
            &TextureSettings::new()).unwrap(), Texture::from_path(Path::new(&"Playing Cards/PNG-cards-1.3/back.png".to_string()), &TextureSettings::new()).unwrap()));
            }
        }

        let flop_card1   = Image::new().rect(rectangle::square(320.0, 225.0, 100.0));
        let flop_card2   = Image::new().rect(rectangle::square(435.0, 225.0, 100.0));
        let flop_card3   = Image::new().rect(rectangle::square(550.0, 225.0, 100.0));
        let turn_card   = Image::new().rect(rectangle::square(665.0, 225.0, 100.0));
        let river_card   = Image::new().rect(rectangle::square(780.0, 225.0, 100.0));

        let player1_card1   = Image::new().rect(rectangle::square(505.0, 410.0, 100.0));
        let player1_card2   = Image::new().rect(rectangle::square(610.0, 410.0, 100.0));

        let player2_card1   = Image::new().rect(rectangle::square(205.0, 50.0, 100.0));
        let player2_card2   = Image::new().rect(rectangle::square(310.0, 50.0, 100.0));

        let player3_card1   = Image::new().rect(rectangle::square(805.0, 50.0, 100.0));
        let player3_card2   = Image::new().rect(rectangle::square(910.0, 50.0, 100.0));

        let player4_card1   = Image::new().rect(rectangle::square(505.0, 20.0, 100.0));
        let player4_card2   = Image::new().rect(rectangle::square(610.0, 20.0, 100.0));

        let player5_card1   = Image::new().rect(rectangle::square(210.0, 380.0, 100.0));
        let player5_card2   = Image::new().rect(rectangle::square(315.0, 380.0, 100.0));

        let player6_card1   = Image::new().rect(rectangle::square(805.0, 380.0, 100.0));
        let player6_card2   = Image::new().rect(rectangle::square(910.0, 380.0, 100.0));

        let player7_card1   = Image::new().rect(rectangle::square(60.0, 225.0, 100.0));
        let player7_card2   = Image::new().rect(rectangle::square(165.0, 225.0, 100.0));

        let player8_card1   = Image::new().rect(rectangle::square(935.0, 225.0, 100.0));
        let player8_card2   = Image::new().rect(rectangle::square(1040.0, 225.0, 100.0));

        let flop1_texture = Texture::from_path(Path::new(&self.flop1), &TextureSettings::new()).unwrap();
        let flop2_texture = Texture::from_path(Path::new(&self.flop2), &TextureSettings::new()).unwrap();
        let flop3_texture = Texture::from_path(Path::new(&self.flop3), &TextureSettings::new()).unwrap();

        let turn_texture = Texture::from_path(Path::new(&self.turn), &TextureSettings::new()).unwrap();
        let river_texture = Texture::from_path(Path::new(&self.river_card), &TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            if player_count < 3 {
                println!("No more players, please press escape.");
                return;
            }

            clear(RED, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            carpet.draw(&carpet_texture, &DrawState::new_alpha(), c.transform, gl);
            carpet2.draw(&carpet_texture, &DrawState::new_alpha(), c.transform, gl);

            ellipse(DARK_GREEN, table, transform, gl);
            circle_arc(GREEN, 3.0, 0.0, 100.0, table, transform, gl);
            circle_arc(BLACK, 7.0, 0.0, 100.0, outer_ring, transform, gl);

            flop_card1.draw(&flop1_texture, &DrawState::new_alpha(), c.transform, gl);
            flop_card2.draw(&flop2_texture, &DrawState::new_alpha(), c.transform, gl);
            flop_card3.draw(&flop3_texture, &DrawState::new_alpha(), c.transform, gl);
            turn_card.draw(&turn_texture, &DrawState::new_alpha(), c.transform, gl);
            river_card.draw(&river_texture, &DrawState::new_alpha(), c.transform, gl);

            player1_card1.draw(&texture_list[0].0, &DrawState::new_alpha(), c.transform, gl);
            player1_card2.draw(&texture_list[0].1, &DrawState::new_alpha(), c.transform, gl);
        
            player2_card1.draw(&texture_list[1].0, &DrawState::new_alpha(), c.transform, gl);
            player2_card2.draw(&texture_list[1].1, &DrawState::new_alpha(), c.transform, gl);

            player3_card1.draw(&texture_list[2].0, &DrawState::new_alpha(), c.transform, gl);
            player3_card2.draw(&texture_list[2].1, &DrawState::new_alpha(), c.transform, gl);

            if player_count >= 4 {
                player4_card1.draw(&texture_list[3].0, &DrawState::new_alpha(), c.transform, gl);
                player4_card2.draw(&texture_list[3].1, &DrawState::new_alpha(), c.transform, gl);
            }
            if player_count >= 5 {
                player5_card1.draw(&texture_list[4].0, &DrawState::new_alpha(), c.transform, gl);
                player5_card2.draw(&texture_list[4].1, &DrawState::new_alpha(), c.transform, gl);
            }
            if player_count >= 6 {
                player6_card1.draw(&texture_list[5].0, &DrawState::new_alpha(), c.transform, gl);
                player6_card2.draw(&texture_list[5].1, &DrawState::new_alpha(), c.transform, gl);
            }
            if player_count >= 7 {
                player7_card1.draw(&texture_list[6].0, &DrawState::new_alpha(), c.transform, gl);
                player7_card2.draw(&texture_list[6].1, &DrawState::new_alpha(), c.transform, gl);
            }
            if player_count >= 8 {
                player8_card1.draw(&texture_list[7].0, &DrawState::new_alpha(), c.transform, gl);
                player8_card2.draw(&texture_list[7].1, &DrawState::new_alpha(), c.transform, gl);
            }
        });
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::D => {
                    if self.new_game {
                        for (loc,player) in self.players.clone().iter().enumerate() {
                            if player.clone().get_chips() <= 0 {
                                println!("{} has ran out of chips!", player.clone().get_name());
                                if player.clone().get_name() == self.players[0].clone().get_name() {
                                    println!("GAME OVER - YOU LOST");
                                    break;
                                }
                                self.players.remove(loc-1);
                            }   
                        }
                        for i in 1..self.players.len() {
                            self.players[i].card1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                            self.players[i].card2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                            self.players[i].cards = Hand::default();
                            self.players[i].playing = true;
                            self.players[i].eligable = true;
                        }
                        self.river = Hand::default();
                        self.flop1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.flop2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.flop3 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.turn = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.river_card = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        give_starting(&mut self.deck, &mut self.players);
                        self.new_game = false;
                        return;
                    }
                    round(&mut self.players, &mut self.pot);
                    if self.river.len() == 5 || players_remaining(&mut self.players) == 1 {
                        for i in 1..self.players.len() {
                            if self.players[i].eligable {
                                self.players[i].card1 = self.card_map.get(&(self.players[i].cards[0].suit, self.players[i].cards[0].value))
                                .unwrap().to_string();
                                self.players[i].card2 = self.card_map.get(&(self.players[i].cards[1].suit, self.players[i].cards[1].value))
                                .unwrap().to_string();
                            }
                        }
                        finish(&mut self.players, self.river.clone(), &mut self.pot);
                        if self.players.len() == 1 {
                            for p in self.players.clone() {
                                println!("{} is the winner!!!! Better luck next time boyz", p.get_name());
                            }
                        }
                        self.new_game = true;
                    } else if self.river.len() == 0 {
                        deal2(&mut self.deck,&mut self.river);
                        deal2(&mut self.deck,&mut self.river);
                        deal2(&mut self.deck,&mut self.river);
                        self.flop1 = self.card_map.get(&(self.river[0].suit, self.river[0].value)).unwrap().to_string();
                        self.flop2 = self.card_map.get(&(self.river[1].suit, self.river[1].value)).unwrap().to_string();
                        self.flop3 = self.card_map.get(&(self.river[2].suit, self.river[2].value)).unwrap().to_string();
                    } else {
                        deal2(&mut self.deck, &mut self.river);
                        if self.turn == "Playing Cards/PNG-cards-1.3/back.png" {
                            self.turn = self.card_map.get(&(self.river[3].suit, self.river[3].value)).unwrap().to_string();
                        } else {
                            self.river_card = self.card_map.get(&(self.river[4].suit, self.river[4].value)).unwrap().to_string();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
}

fn get_input() -> String {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed");
        buffer
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Poker Table", [1200.0, 560.0])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    println!("How many people are playing?");
    
    let amount = get_input().trim().parse::<i32>().unwrap();
    assert!(amount > 2 && amount <= 8);

    let mut test_players = create_players(amount);
    let mut start_deck = Deck::default();

    give_starting(&mut start_deck, &mut test_players);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        card_image: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        card_map: HashMap::from([
            ((Suit::Club, Value::Two), "Playing Cards/PNG-cards-1.3/2_of_clubs.png".to_string()),
            ((Suit::Club, Value::Three), "Playing Cards/PNG-cards-1.3/3_of_clubs.png".to_string()),
            ((Suit::Club, Value::Four), "Playing Cards/PNG-cards-1.3/4_of_clubs.png".to_string()),
            ((Suit::Club, Value::Five), "Playing Cards/PNG-cards-1.3/5_of_clubs.png".to_string()),
            ((Suit::Club, Value::Six), "Playing Cards/PNG-cards-1.3/6_of_clubs.png".to_string()),
            ((Suit::Club, Value::Seven), "Playing Cards/PNG-cards-1.3/7_of_clubs.png".to_string()),
            ((Suit::Club, Value::Eight), "Playing Cards/PNG-cards-1.3/8_of_clubs.png".to_string()),
            ((Suit::Club, Value::Nine), "Playing Cards/PNG-cards-1.3/9_of_clubs.png".to_string()),
            ((Suit::Club, Value::Ten), "Playing Cards/PNG-cards-1.3/10_of_clubs.png".to_string()),
            ((Suit::Club, Value::Jack), "Playing Cards/PNG-cards-1.3/jack_of_clubs2.png".to_string()),
            ((Suit::Club, Value::Queen), "Playing Cards/PNG-cards-1.3/queen_of_clubs2.png".to_string()),
            ((Suit::Club, Value::King), "Playing Cards/PNG-cards-1.3/king_of_clubs2.png".to_string()),
            ((Suit::Club, Value::Ace), "Playing Cards/PNG-cards-1.3/ace_of_clubs.png".to_string()),
            
            ((Suit::Spade, Value::Two), "Playing Cards/PNG-cards-1.3/2_of_spades.png".to_string()),
            ((Suit::Spade, Value::Three), "Playing Cards/PNG-cards-1.3/3_of_spades.png".to_string()),
            ((Suit::Spade, Value::Four), "Playing Cards/PNG-cards-1.3/4_of_spades.png".to_string()),
            ((Suit::Spade, Value::Five), "Playing Cards/PNG-cards-1.3/5_of_spades.png".to_string()),
            ((Suit::Spade, Value::Six), "Playing Cards/PNG-cards-1.3/6_of_spades.png".to_string()),
            ((Suit::Spade, Value::Seven), "Playing Cards/PNG-cards-1.3/7_of_spades.png".to_string()),
            ((Suit::Spade, Value::Eight), "Playing Cards/PNG-cards-1.3/8_of_spades.png".to_string()),
            ((Suit::Spade, Value::Nine), "Playing Cards/PNG-cards-1.3/9_of_spades.png".to_string()),
            ((Suit::Spade, Value::Ten), "Playing Cards/PNG-cards-1.3/10_of_spades.png".to_string()),
            ((Suit::Spade, Value::Jack), "Playing Cards/PNG-cards-1.3/jack_of_spades2.png".to_string()),
            ((Suit::Spade, Value::Queen), "Playing Cards/PNG-cards-1.3/queen_of_spades2.png".to_string()),
            ((Suit::Spade, Value::King), "Playing Cards/PNG-cards-1.3/king_of_spades2.png".to_string()),
            ((Suit::Spade, Value::Ace), "Playing Cards/PNG-cards-1.3/ace_of_spades.png".to_string()),

            ((Suit::Heart, Value::Two), "Playing Cards/PNG-cards-1.3/2_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Three), "Playing Cards/PNG-cards-1.3/3_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Four), "Playing Cards/PNG-cards-1.3/4_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Five), "Playing Cards/PNG-cards-1.3/5_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Six), "Playing Cards/PNG-cards-1.3/6_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Seven), "Playing Cards/PNG-cards-1.3/7_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Eight), "Playing Cards/PNG-cards-1.3/8_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Nine), "Playing Cards/PNG-cards-1.3/9_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Ten), "Playing Cards/PNG-cards-1.3/10_of_hearts.png".to_string()),
            ((Suit::Heart, Value::Jack), "Playing Cards/PNG-cards-1.3/jack_of_hearts2.png".to_string()),
            ((Suit::Heart, Value::Queen), "Playing Cards/PNG-cards-1.3/queen_of_hearts2.png".to_string()),
            ((Suit::Heart, Value::King), "Playing Cards/PNG-cards-1.3/king_of_hearts2.png".to_string()),
            ((Suit::Heart, Value::Ace), "Playing Cards/PNG-cards-1.3/ace_of_hearts.png".to_string()),

            ((Suit::Diamond, Value::Two), "Playing Cards/PNG-cards-1.3/2_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Three), "Playing Cards/PNG-cards-1.3/3_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Four), "Playing Cards/PNG-cards-1.3/4_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Five), "Playing Cards/PNG-cards-1.3/5_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Six), "Playing Cards/PNG-cards-1.3/6_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Seven), "Playing Cards/PNG-cards-1.3/7_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Eight), "Playing Cards/PNG-cards-1.3/8_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Nine), "Playing Cards/PNG-cards-1.3/9_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Ten), "Playing Cards/PNG-cards-1.3/10_of_diamonds.png".to_string()),
            ((Suit::Diamond, Value::Jack), "Playing Cards/PNG-cards-1.3/jack_of_diamonds2.png".to_string()),
            ((Suit::Diamond, Value::Queen), "Playing Cards/PNG-cards-1.3/queen_of_diamonds2.png".to_string()),
            ((Suit::Diamond, Value::King), "Playing Cards/PNG-cards-1.3/king_of_diamonds2.png".to_string()),
            ((Suit::Diamond, Value::Ace), "Playing Cards/PNG-cards-1.3/ace_of_diamonds.png".to_string()),


            
        ]),
        flop1: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        flop2: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        flop3: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        turn: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        river_card: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        players: test_players.clone(),
        river: Hand::default(),
        deck: start_deck,
        new_game: false,
        pot: 0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(b) = e.press_args() {
            app.press(&b);
        }
    }
}
