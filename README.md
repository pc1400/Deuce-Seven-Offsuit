# Deuce-Seven-Offsuit

Group Name: 
  -Duece Seven Offsuit
Group member names and NetIDs:
  -Conor Cunningham (conorrc2), Patrick Cunningham (pmc4)
  
Project Introduction:
We have chosen this project because over the last two semesters we have found a love for poker.
-Analytics
-Analyzing emotions
-Social aspect

Goals:
  -Make a playable version of poker that closely models a real game of poker
  -Make the game be playable by multiple people from differnet computers
  -Establish a computer player that potentially has a range of skill levels
  -Create a visually pleasing presentation that is easy for the user to interact with.
  -Create an good betting system, players are able to call, raise and fold. Based on the previous move of the player, the next player must respond. If the player raises, the next player must either call, fold or raise. If the player before checks, the next player can check. Players are able to 

System Overview:
  -Player struct
    -Is able to keep track of chip count, if the count ever drops below 0 they are kicked out, or given the option to buy back in.
    -Keeps track of the players total winnings.
    -Cards in hand
    -Place bet
  -Deck of cards struct
    -Array of 52
    -Shuffle function
    -Remove card function
  -Dealer Struct
    -Checks players status
    -Determines who wins
    -Deals the cards

If we have time, we want to create an AI that is able to analyze the chance of winning based on the cards in their hand, bets accordingly. Additionally, we would need to create a visible board that the players can look at, a graphic interface. Would be fun to add a luck aspect to the game, maybe if someone is really heating up after winning a couple hands they are more likely to get dealt a better hand. As a whole, the game moves in a circle, it starts by the dealer dealing two cards to every player. The initial bets are made by each player. Next, the "flop" of the three cards are shown. Then the process repeats. Each player makes a bett or simply calls, then the other players must make a decision based on the previous bet.

Possible Challenges:
  -Figuring out how to visualize the game, making a good presentation for the user.
  -Carrying the logic of Poker, making sure the deck is shuffled, cards are distrbuted correctly, hands are ranked properly.
  -User intreraction, building the game for multiple people
    -If we want to have an AI player if someone drops out
    -How do we handle if someone leaves the game early?
  -Keeping track of winnings across various rounds and players, transfering money from one player to another player
  -Managing memory when a player is eliminated, what happens to their seat?

References:
  -None othet than the suggestion on the final project information document.
