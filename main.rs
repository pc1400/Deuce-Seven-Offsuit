extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate embedded_graphics;
extern crate find_folder;
extern crate touch_visualizer;
extern crate piston_window;

mod game;
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
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Key, Button};
use rs_poker::core::Value;
use std::collections::HashMap;
use std::path::Path;
use graphics::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    card_image: String,

    flop1: String,
    flop2: String,
    flop3: String,
    turn: String,
    river_card: String,
    
    player1_card_image1: String,
    player1_card_image2: String,

    player2_card_image1: String,
    player2_card_image2: String,

    player3_card_image1: String,
    player3_card_image2: String,
    

    card_map: HashMap<(Suit, Value), String>,
    players: Vec<Player>,
    river: Hand,
    pot: i32,

    deck: Deck,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        

        const GREEN: [f32; 4] = [0.0, 0.75, 0.0, 1.0];
        const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
        const RED: [f32; 4] = [0.5, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        

        self.player1_card_image1 = self.card_map.get(&(self.players[0].cards[0].suit, self.players[0].cards[0].value)).unwrap().to_string();
        self.player1_card_image2 = self.card_map.get(&(self.players[0].cards[1].suit, self.players[0].cards[1].value)).unwrap().to_string();

        let table = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 375.0, 225.0));

        let outer_ring = rectangle::centered(rectangle::rectangle_by_corners(25.0, 25.0, 385.0, 235.0));
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let carpet   = Image::new().rect(rectangle::square(0.0, 0.0, 1000.0));

        let carpet_texture = Texture::from_path(Path::new("Playing Cards/PNG-cards-1.3/new_carpet.png"), &TextureSettings::new()).unwrap();

        let flop_card1   = Image::new().rect(rectangle::square(220.0, 225.0, 100.0));
        let flop_card2   = Image::new().rect(rectangle::square(335.0, 225.0, 100.0));
        let flop_card3   = Image::new().rect(rectangle::square(450.0, 225.0, 100.0));
        let turn_card   = Image::new().rect(rectangle::square(565.0, 225.0, 100.0));
        let river_card   = Image::new().rect(rectangle::square(680.0, 225.0, 100.0));

        let player1_card1   = Image::new().rect(rectangle::square(380.0, 410.0, 100.0));
        let player1_card2   = Image::new().rect(rectangle::square(485.0, 410.0, 100.0));

        let player2_card1   = Image::new().rect(rectangle::square(40.0, 50.0, 100.0));
        let player2_card2   = Image::new().rect(rectangle::square(145.0, 50.0, 100.0));

        let player3_card1   = Image::new().rect(rectangle::square(800.0, 50.0, 100.0));
        let player3_card2   = Image::new().rect(rectangle::square(695.0, 50.0, 100.0));

        let flop1_texture = Texture::from_path(Path::new(&self.flop1), &TextureSettings::new()).unwrap();
        let flop2_texture = Texture::from_path(Path::new(&self.flop2), &TextureSettings::new()).unwrap();
        let flop3_texture = Texture::from_path(Path::new(&self.flop3), &TextureSettings::new()).unwrap();

        let player1_card1_texture = Texture::from_path(Path::new(&self.player1_card_image1), &TextureSettings::new()).unwrap();
        let player1_card2_texture = Texture::from_path(Path::new(&self.player1_card_image2), &TextureSettings::new()).unwrap();

        let player2_card1_texture = Texture::from_path(Path::new(&self.player2_card_image1), &TextureSettings::new()).unwrap();
        let player2_card2_texture = Texture::from_path(Path::new(&self.player2_card_image2), &TextureSettings::new()).unwrap();

        let player3_card1_texture = Texture::from_path(Path::new(&self.player3_card_image1), &TextureSettings::new()).unwrap();
        let player3_card2_texture = Texture::from_path(Path::new(&self.player3_card_image2), &TextureSettings::new()).unwrap();

        let turn_texture = Texture::from_path(Path::new(&self.turn), &TextureSettings::new()).unwrap();
        let river_texture = Texture::from_path(Path::new(&self.river_card), &TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(RED, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            carpet.draw(&carpet_texture, &DrawState::new_alpha(), c.transform, gl);

            ellipse(DARK_GREEN, table, transform, gl);
            circle_arc(GREEN, 3.0, 0.0, 100.0, table, transform, gl);
            circle_arc(BLACK, 7.0, 0.0, 100.0, outer_ring, transform, gl);

            flop_card1.draw(&flop1_texture, &DrawState::new_alpha(), c.transform, gl);
            flop_card2.draw(&flop2_texture, &DrawState::new_alpha(), c.transform, gl);
            flop_card3.draw(&flop3_texture, &DrawState::new_alpha(), c.transform, gl);
            turn_card.draw(&turn_texture, &DrawState::new_alpha(), c.transform, gl);
            river_card.draw(&river_texture, &DrawState::new_alpha(), c.transform, gl);

            player1_card1.draw(&player1_card1_texture, &DrawState::new_alpha(), c.transform, gl);
            player1_card2.draw(&player1_card2_texture, &DrawState::new_alpha(), c.transform, gl);

            player2_card1.draw(&player2_card1_texture, &DrawState::new_alpha(), c.transform, gl);
            player2_card2.draw(&player2_card2_texture, &DrawState::new_alpha(), c.transform, gl);

            player3_card1.draw(&player3_card1_texture, &DrawState::new_alpha(), c.transform, gl);
            player3_card2.draw(&player3_card2_texture, &DrawState::new_alpha(), c.transform, gl);



        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Space => {
                    self.card_image = "Playing Cards/PNG-cards-1.3/5_of_diamonds.png".to_string();
                }
                Key::D => {
                    round(&mut self.players, &mut self.pot);
                    
                    if self.river.len() == 5 {
                        finish(&mut self.players, self.river.clone(), self.pot);

                        self.player2_card_image1 = self.card_map.get(&(self.players[1].cards[0].suit, self.players[1].cards[0].value)).unwrap().to_string();
                        self.player2_card_image2 = self.card_map.get(&(self.players[1].cards[1].suit, self.players[1].cards[1].value)).unwrap().to_string();

                        self.player3_card_image1 = self.card_map.get(&(self.players[2].cards[0].suit, self.players[2].cards[0].value)).unwrap().to_string();
                        self.player3_card_image2 = self.card_map.get(&(self.players[2].cards[1].suit, self.players[2].cards[1].value)).unwrap().to_string();
                        self.river = Hand::default();

                        for player in self.players.iter_mut() {
                            player.cards = Hand::default();
                        }

                        self.flop1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.flop2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.flop3 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.turn = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.river_card = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        
                        self.player1_card_image1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.player1_card_image2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        
                        self.player2_card_image1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.player2_card_image2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();

                        self.player3_card_image1 = "Playing Cards/PNG-cards-1.3/back.png".to_string();
                        self.player3_card_image2 = "Playing Cards/PNG-cards-1.3/back.png".to_string();

                        give_starting(&mut self.deck, &mut self.players);

                        self.player1_card_image1 = self.card_map.get(&(self.players[0].cards[0].suit, self.players[0].cards[0].value)).unwrap().to_string();
                        self.player1_card_image2 = self.card_map.get(&(self.players[0].cards[1].suit, self.players[0].cards[1].value)).unwrap().to_string();


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

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut capture_cursor = false;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Poker Table", [1000.0, 560.0])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut start_deck = Deck::default();
    let mut start_river = Hand::default();
    let mut pot: i32 = 0;

    let mut test_players = create_players(3);

    

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

        player1_card_image1: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        player1_card_image2: "Playing Cards/PNG-cards-1.3/back.png".to_string(),

        player2_card_image1: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        player2_card_image2: "Playing Cards/PNG-cards-1.3/back.png".to_string(),

        player3_card_image1: "Playing Cards/PNG-cards-1.3/back.png".to_string(),
        player3_card_image2: "Playing Cards/PNG-cards-1.3/back.png".to_string(),

        players: test_players.clone(),
        river: start_river.clone(),
        deck: start_deck,
        pot: 0,

    };

    

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {


        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }
    }
}

