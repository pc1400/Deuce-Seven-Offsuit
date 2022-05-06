# Deuce-Seven-Offsuit

Group Name: 
  -Duece Seven Offsuit
Group member names and NetIDs:
  -Conor Cunningham (conorrc2), Patrick Cunningham (pmc4)
  
Project Introduction:
We have chosen this project because over the last two semesters we have found a love for poker. The game itself can be learned in roughly an hour of playing, though after that hour, there is so much more to learn. As Engineering students, the statistical analysis that is prevelant with each hand is fascinating, and represents an edge that many fail to consider. What's more, the game has many non-technical aspects, such as analysisng your openents facial features or betting habbits. Finally, poker is a fun game to play socially and is often the highlight of our week if we have the chance to play.

Goals:
  -Make a playable version of poker that closely models a real game of poker
  -Make the game be playable by multiple people from differnet computers
  -Establish a computer player that potentially has a range of skill levels
  -Create a visually pleasing presentation that is easy for the user to interact with.
  -Create an good betting system, players are able to call, raise and fold. Based on the previous move of the player, the next player must respond. If the player raises, the next player must either call, fold or raise. If the player before checks, the next player can check. Players are able to 

System Overview:
  -Player struct
    -Keeps track of the cards in hand, paired with an PNG image of the actual card for the graphics portion of the program
    -Chip count, if this count ever drops below zero at the start of a game, they are removed from the game.
    -Keeps track of the players playing statuts, if the folded, all in, ect.
    -Player name
  -Dealer Struct
    -The heavy lifting of this function will be done using the poker.rs crate, this crate gives card structs, hand structs, and provides hand evaluations.
    -Poker crate is able to deal each player their hand, and deals the 5 cards on the table.
    -Checks players status
    -Determines who wins, this is done using the hand evaluation functions provided by poker crate.

    Our program also has a very basic AI system for the other players in the game. The computer will choose a random number between 1 and 3 to determine their move.
      -Check
      -Raise
      -Fold
    
    As a whole, the game moves in a circle, it starts by the dealer dealing two cards to every player. The initial bets are made by each player. Next, the "flop" of the three cards are shown. Then the process repeats. Each player makes a bett or simply calls, then the other players must make a decision based on the previous bet.

Possible Challenges:
  -Figuring out how to visualize the game, making a good presentation for the user.
  -Carrying the logic of Poker, making sure the deck is shuffled, cards are distrbuted correctly, hands are ranked properly.
  -User intreraction, building the game for multiple people
    -If we want to have an AI player if someone drops out
    -How do we handle if someone leaves the game early?
  -Keeping track of winnings across various rounds and players, transfering money from one player to another player
  -Managing memory when a player is eliminated, what happens to their seat?

References:
  -None other than the suggestion on the final project information document.





