//! # Day 22: Crab Combat
//!
//! It only takes a few hours of sailing the ocean on a raft for boredom to sink in. Fortunately, you brought a small deck of space cards! You'd like to play a game of Combat, and there's even an opponent available: a small crab that climbed aboard your raft before you left.
//!
//! Fortunately, it doesn't take long to teach the crab the rules.
//!
//! Before the game starts, split the cards so each player has their own deck (your puzzle input). Then, the game consists of a series of rounds: both players draw their top card, and the player with the higher-valued card wins the round. The winner keeps both cards, placing them on the bottom of their own deck so that the winner's card is above the other card. If this causes a player to have all of the cards, they win, and the game ends.
//!
//! For example, consider the following starting decks:
//!
//! ```text
//! Player 1:
//! 9
//! 2
//! 6
//! 3
//! 1
//!
//! Player 2:
//! 5
//! 8
//! 4
//! 7
//! 10
//! ```
//!
//! This arrangement means that player 1's deck contains 5 cards, with 9 on top and 1 on the bottom; player 2's deck also contains 5 cards, with 5 on top and 10 on the bottom.
//!
//! The first round begins with both players drawing the top card of their decks: 9 and 5. Player 1 has the higher card, so both cards move to the bottom of player 1's deck such that 9 is above 5. In total, it takes 29 rounds before a player has all of the cards:
//!
//! ```text
//! -- Round 1 --
//! Player 1's deck: 9, 2, 6, 3, 1
//! Player 2's deck: 5, 8, 4, 7, 10
//! Player 1 plays: 9
//! Player 2 plays: 5
//! Player 1 wins the round!
//!
//! -- Round 2 --
//! Player 1's deck: 2, 6, 3, 1, 9, 5
//! Player 2's deck: 8, 4, 7, 10
//! Player 1 plays: 2
//! Player 2 plays: 8
//! Player 2 wins the round!
//!
//! -- Round 3 --
//! Player 1's deck: 6, 3, 1, 9, 5
//! Player 2's deck: 4, 7, 10, 8, 2
//! Player 1 plays: 6
//! Player 2 plays: 4
//! Player 1 wins the round!
//!
//! -- Round 4 --
//! Player 1's deck: 3, 1, 9, 5, 6, 4
//! Player 2's deck: 7, 10, 8, 2
//! Player 1 plays: 3
//! Player 2 plays: 7
//! Player 2 wins the round!
//!
//! -- Round 5 --
//! Player 1's deck: 1, 9, 5, 6, 4
//! Player 2's deck: 10, 8, 2, 7, 3
//! Player 1 plays: 1
//! Player 2 plays: 10
//! Player 2 wins the round!
//!
//! ...several more rounds pass...
//!
//! -- Round 27 --
//! Player 1's deck: 5, 4, 1
//! Player 2's deck: 8, 9, 7, 3, 2, 10, 6
//! Player 1 plays: 5
//! Player 2 plays: 8
//! Player 2 wins the round!
//!
//! -- Round 28 --
//! Player 1's deck: 4, 1
//! Player 2's deck: 9, 7, 3, 2, 10, 6, 8, 5
//! Player 1 plays: 4
//! Player 2 plays: 9
//! Player 2 wins the round!
//!
//! -- Round 29 --
//! Player 1's deck: 1
//! Player 2's deck: 7, 3, 2, 10, 6, 8, 5, 9, 4
//! Player 1 plays: 1
//! Player 2 plays: 7
//! Player 2 wins the round!
//!
//!
//! == Post-game results ==
//! Player 1's deck:
//! Player 2's deck: 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
//! ```
//!
//! Once the game ends, you can calculate the winning player's score. The bottom card in their deck is worth the value of the card multiplied by 1, the second-from-the-bottom card is worth the value of the card multiplied by 2, and so on. With 10 cards, the top card is worth the value on the card multiplied by 10. In this example, the winning player's score is:
//!
//! ```text
//!    3 * 10
//! +  2 *  9
//! + 10 *  8
//! +  6 *  7
//! +  8 *  6
//! +  5 *  5
//! +  9 *  4
//! +  4 *  3
//! +  7 *  2
//! +  1 *  1
//! = 306
//! ```
//!
//! So, once the game ends, the winning player's score is 306.
//!
//! Play the small crab in a game of Combat using the two decks you just dealt. What is the winning player's score?
//!
//! # Part Two
//!
//! You lost to the small crab! Fortunately, crabs aren't very good at recursion. To defend your honor as a Raft Captain, you challenge the small crab to a game of Recursive Combat.
//!
//! Recursive Combat still starts by splitting the cards into two decks (you offer to play with the same starting decks as before - it's only fair). Then, the game consists of a series of rounds with a few changes:
//!
//! Before either player deals a card, if there was a previous round in this game that had exactly the same cards in the same order in the same players' decks, the game instantly ends in a win for player 1. Previous rounds from other games are not considered. (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
//! Otherwise, this round's cards must be in a new configuration; the players begin the round by each drawing the top card of their deck as normal.
//! If both players have at least as many cards remaining in their deck as the value of the card they just drew, the winner of the round is determined by playing a new game of Recursive Combat (see below).
//! Otherwise, at least one player must not have enough cards left in their deck to recurse; the winner of the round is the player with the higher-value card.
//! As in regular Combat, the winner of the round (even if they won the round by winning a sub-game) takes the two cards dealt at the beginning of the round and places them on the bottom of their own deck (again so that the winner's card is above the other card). Note that the winner's card might be the lower-valued of the two cards if they won the round due to winning a sub-game. If collecting cards by winning the round causes a player to have all of the cards, they win, and the game ends.
//!
//! Here is an example of a small game that would loop forever without the infinite game prevention rule:
//!
//! ```text
//! Player 1:
//! 43
//! 19
//!
//! Player 2:
//! 2
//! 29
//! 14
//! ```
//!
//! During a round of Recursive Combat, if both players have at least as many cards in their own decks as the number on the card they just dealt, the winner of the round is determined by recursing into a sub-game of Recursive Combat. (For example, if player 1 draws the 3 card, and player 2 draws the 7 card, this would occur if player 1 has at least 3 cards left and player 2 has at least 7 cards left, not counting the 3 and 7 cards that were drawn.)
//!
//! To play a sub-game of Recursive Combat, each player creates a new deck by making a copy of the next cards in their deck (the quantity of cards copied is equal to the number on the card they drew to trigger the sub-game). During this sub-game, the game that triggered it is on hold and completely unaffected; no cards are removed from players' decks to form the sub-game. (For example, if player 1 drew the 3 card, their deck in the sub-game would be copies of the next three cards in their deck.)
//!
//! Here is a complete example of gameplay, where Game 1 is the primary game of Recursive Combat:
//!
//! ```text
//! === Game 1 ===
//!
//! -- Round 1 (Game 1) --
//! Player 1's deck: 9, 2, 6, 3, 1
//! Player 2's deck: 5, 8, 4, 7, 10
//! Player 1 plays: 9
//! Player 2 plays: 5
//! Player 1 wins round 1 of game 1!
//!
//! -- Round 2 (Game 1) --
//! Player 1's deck: 2, 6, 3, 1, 9, 5
//! Player 2's deck: 8, 4, 7, 10
//! Player 1 plays: 2
//! Player 2 plays: 8
//! Player 2 wins round 2 of game 1!
//!
//! -- Round 3 (Game 1) --
//! Player 1's deck: 6, 3, 1, 9, 5
//! Player 2's deck: 4, 7, 10, 8, 2
//! Player 1 plays: 6
//! Player 2 plays: 4
//! Player 1 wins round 3 of game 1!
//!
//! -- Round 4 (Game 1) --
//! Player 1's deck: 3, 1, 9, 5, 6, 4
//! Player 2's deck: 7, 10, 8, 2
//! Player 1 plays: 3
//! Player 2 plays: 7
//! Player 2 wins round 4 of game 1!
//!
//! -- Round 5 (Game 1) --
//! Player 1's deck: 1, 9, 5, 6, 4
//! Player 2's deck: 10, 8, 2, 7, 3
//! Player 1 plays: 1
//! Player 2 plays: 10
//! Player 2 wins round 5 of game 1!
//!
//! -- Round 6 (Game 1) --
//! Player 1's deck: 9, 5, 6, 4
//! Player 2's deck: 8, 2, 7, 3, 10, 1
//! Player 1 plays: 9
//! Player 2 plays: 8
//! Player 1 wins round 6 of game 1!
//!
//! -- Round 7 (Game 1) --
//! Player 1's deck: 5, 6, 4, 9, 8
//! Player 2's deck: 2, 7, 3, 10, 1
//! Player 1 plays: 5
//! Player 2 plays: 2
//! Player 1 wins round 7 of game 1!
//!
//! -- Round 8 (Game 1) --
//! Player 1's deck: 6, 4, 9, 8, 5, 2
//! Player 2's deck: 7, 3, 10, 1
//! Player 1 plays: 6
//! Player 2 plays: 7
//! Player 2 wins round 8 of game 1!
//!
//! -- Round 9 (Game 1) --
//! Player 1's deck: 4, 9, 8, 5, 2
//! Player 2's deck: 3, 10, 1, 7, 6
//! Player 1 plays: 4
//! Player 2 plays: 3
//! Playing a sub-game to determine the winner...
//!
//! === Game 2 ===
//!
//! -- Round 1 (Game 2) --
//! Player 1's deck: 9, 8, 5, 2
//! Player 2's deck: 10, 1, 7
//! Player 1 plays: 9
//! Player 2 plays: 10
//! Player 2 wins round 1 of game 2!
//!
//! -- Round 2 (Game 2) --
//! Player 1's deck: 8, 5, 2
//! Player 2's deck: 1, 7, 10, 9
//! Player 1 plays: 8
//! Player 2 plays: 1
//! Player 1 wins round 2 of game 2!
//!
//! -- Round 3 (Game 2) --
//! Player 1's deck: 5, 2, 8, 1
//! Player 2's deck: 7, 10, 9
//! Player 1 plays: 5
//! Player 2 plays: 7
//! Player 2 wins round 3 of game 2!
//!
//! -- Round 4 (Game 2) --
//! Player 1's deck: 2, 8, 1
//! Player 2's deck: 10, 9, 7, 5
//! Player 1 plays: 2
//! Player 2 plays: 10
//! Player 2 wins round 4 of game 2!
//!
//! -- Round 5 (Game 2) --
//! Player 1's deck: 8, 1
//! Player 2's deck: 9, 7, 5, 10, 2
//! Player 1 plays: 8
//! Player 2 plays: 9
//! Player 2 wins round 5 of game 2!
//!
//! -- Round 6 (Game 2) --
//! Player 1's deck: 1
//! Player 2's deck: 7, 5, 10, 2, 9, 8
//! Player 1 plays: 1
//! Player 2 plays: 7
//! Player 2 wins round 6 of game 2!
//! The winner of game 2 is player 2!
//!
//! ...anyway, back to game 1.
//! Player 2 wins round 9 of game 1!
//!
//! -- Round 10 (Game 1) --
//! Player 1's deck: 9, 8, 5, 2
//! Player 2's deck: 10, 1, 7, 6, 3, 4
//! Player 1 plays: 9
//! Player 2 plays: 10
//! Player 2 wins round 10 of game 1!
//!
//! -- Round 11 (Game 1) --
//! Player 1's deck: 8, 5, 2
//! Player 2's deck: 1, 7, 6, 3, 4, 10, 9
//! Player 1 plays: 8
//! Player 2 plays: 1
//! Player 1 wins round 11 of game 1!
//!
//! -- Round 12 (Game 1) --
//! Player 1's deck: 5, 2, 8, 1
//! Player 2's deck: 7, 6, 3, 4, 10, 9
//! Player 1 plays: 5
//! Player 2 plays: 7
//! Player 2 wins round 12 of game 1!
//!
//! -- Round 13 (Game 1) --
//! Player 1's deck: 2, 8, 1
//! Player 2's deck: 6, 3, 4, 10, 9, 7, 5
//! Player 1 plays: 2
//! Player 2 plays: 6
//! Playing a sub-game to determine the winner...
//!
//! === Game 3 ===
//!
//! -- Round 1 (Game 3) --
//! Player 1's deck: 8, 1
//! Player 2's deck: 3, 4, 10, 9, 7, 5
//! Player 1 plays: 8
//! Player 2 plays: 3
//! Player 1 wins round 1 of game 3!
//!
//! -- Round 2 (Game 3) --
//! Player 1's deck: 1, 8, 3
//! Player 2's deck: 4, 10, 9, 7, 5
//! Player 1 plays: 1
//! Player 2 plays: 4
//! Playing a sub-game to determine the winner...
//!
//! === Game 4 ===
//!
//! -- Round 1 (Game 4) --
//! Player 1's deck: 8
//! Player 2's deck: 10, 9, 7, 5
//! Player 1 plays: 8
//! Player 2 plays: 10
//! Player 2 wins round 1 of game 4!
//! The winner of game 4 is player 2!
//!
//! ...anyway, back to game 3.
//! Player 2 wins round 2 of game 3!
//!
//! -- Round 3 (Game 3) --
//! Player 1's deck: 8, 3
//! Player 2's deck: 10, 9, 7, 5, 4, 1
//! Player 1 plays: 8
//! Player 2 plays: 10
//! Player 2 wins round 3 of game 3!
//!
//! -- Round 4 (Game 3) --
//! Player 1's deck: 3
//! Player 2's deck: 9, 7, 5, 4, 1, 10, 8
//! Player 1 plays: 3
//! Player 2 plays: 9
//! Player 2 wins round 4 of game 3!
//! The winner of game 3 is player 2!
//!
//! ...anyway, back to game 1.
//! Player 2 wins round 13 of game 1!
//!
//! -- Round 14 (Game 1) --
//! Player 1's deck: 8, 1
//! Player 2's deck: 3, 4, 10, 9, 7, 5, 6, 2
//! Player 1 plays: 8
//! Player 2 plays: 3
//! Player 1 wins round 14 of game 1!
//!
//! -- Round 15 (Game 1) --
//! Player 1's deck: 1, 8, 3
//! Player 2's deck: 4, 10, 9, 7, 5, 6, 2
//! Player 1 plays: 1
//! Player 2 plays: 4
//! Playing a sub-game to determine the winner...
//!
//! === Game 5 ===
//!
//! -- Round 1 (Game 5) --
//! Player 1's deck: 8
//! Player 2's deck: 10, 9, 7, 5
//! Player 1 plays: 8
//! Player 2 plays: 10
//! Player 2 wins round 1 of game 5!
//! The winner of game 5 is player 2!
//!
//! ...anyway, back to game 1.
//! Player 2 wins round 15 of game 1!
//!
//! -- Round 16 (Game 1) --
//! Player 1's deck: 8, 3
//! Player 2's deck: 10, 9, 7, 5, 6, 2, 4, 1
//! Player 1 plays: 8
//! Player 2 plays: 10
//! Player 2 wins round 16 of game 1!
//!
//! -- Round 17 (Game 1) --
//! Player 1's deck: 3
//! Player 2's deck: 9, 7, 5, 6, 2, 4, 1, 10, 8
//! Player 1 plays: 3
//! Player 2 plays: 9
//! Player 2 wins round 17 of game 1!
//! The winner of game 1 is player 2!
//!
//!
//! == Post-game results ==
//! Player 1's deck:
//! Player 2's deck: 7, 5, 6, 2, 4, 1, 10, 8, 9, 3
//! ```
//!
//! After the game, the winning player's score is calculated from the cards they have in their original deck using the same rules as regular Combat. In the above game, the winning player's score is 291.
//!
//! Defend your honor as Raft Captain by playing the small crab in a game of Recursive Combat using the same two decks as before. What is the winning player's score?

use std::collections::HashMap;

/// Part one answer.
pub fn run_ex1() -> usize {
    let ((_, mut deck1), (_, mut deck2)) = parse_decks(include_str!("input.txt"));
    match run_game(&mut deck1, &mut deck2) {
        Player(1) => calculate_score(&deck1),
        Player(2) => calculate_score(&deck2),
        _ => unreachable!(),
    }
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let ((_, mut deck1), (_, mut deck2)) = parse_decks(include_str!("input.txt"));
    let mut memory = GameMemory::default();
    match run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1) {
        Player(1) => calculate_score(&deck1),
        Player(2) => calculate_score(&deck2),
        _ => unreachable!(),
    }
}

/// Card.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Card(usize);

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Player.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Player(usize);

/// Deck.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Take the first card available from the deck.
    pub fn take_card(&mut self) -> Option<Card> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }

    /// Add card at the end of the deck.
    ///
    /// # Arguments
    ///
    /// * `card` - Card
    fn add_card(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Check if the deck is empty.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get deck length.
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Clone `count` cards from the beginning of the deck into a new deck.
    ///
    /// # Arguments
    ///
    /// * `count` - Cards count to clone
    fn clone_until(&self, count: usize) -> Self {
        Self(self.0[..count].to_vec())
    }
}

impl std::fmt::Debug for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

/// Game step result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStepResult {
    /// Game is still running.
    Running,
    /// Game is finished, won by `Player`.
    Finished(Player),
}

/// Game memory.
#[derive(Debug, Default)]
pub struct GameMemory {
    cache: HashMap<(Deck, Deck), Player>,
    rounds: HashMap<usize, Vec<(Deck, Deck)>>,
}

/// Parse decks and players from input string.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_decks(input: &str) -> ((Player, Deck), (Player, Deck)) {
    let mut blocks = input.trim().split("\n\n");

    (
        parse_deck(blocks.next().unwrap()),
        parse_deck(blocks.next().unwrap()),
    )
}

/// Parse one deck and player from input string.
///
/// # Arguments
///
/// * `input` - Input string
fn parse_deck(input: &str) -> (Player, Deck) {
    let mut lines = input.trim().lines();
    let player_id: usize = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Player ")
        .unwrap()
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();

    let cards: Vec<Card> = lines.map(|n| Card(n.trim().parse().unwrap())).collect();

    (Player(player_id), Deck(cards))
}

/// Execute a game step.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
pub fn game_step(deck1: &mut Deck, deck2: &mut Deck) -> GameStepResult {
    if deck1.is_empty() {
        GameStepResult::Finished(Player(2))
    } else if deck2.is_empty() {
        GameStepResult::Finished(Player(1))
    } else {
        let card1 = deck1.take_card().unwrap();
        let card2 = deck2.take_card().unwrap();

        if card1 > card2 {
            deck1.add_card(card1);
            deck1.add_card(card2);
        } else {
            deck2.add_card(card2);
            deck2.add_card(card1);
        }

        GameStepResult::Running
    }
}

/// Run game until completion.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
pub fn run_game(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    loop {
        if let GameStepResult::Finished(player) = game_step(deck1, deck2) {
            return player;
        }
    }
}

/// Execute a recursive game step.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
/// * `memory` - Game memory
/// * `game_number` - Game number
/// * `round_number` - Round number
pub fn recursive_game_step(
    deck1: &mut Deck,
    deck2: &mut Deck,
    memory: &mut GameMemory,
    game_number: usize,
) -> GameStepResult {
    // Check if round is already present, based on deck contents
    if !memory.rounds.is_empty() {
        let decks_set = (deck1.clone(), deck2.clone());
        if memory
            .rounds
            .get(&game_number)
            .unwrap()
            .contains(&decks_set)
        {
            return GameStepResult::Finished(Player(1));
        }
    }

    // Store current decks in memory
    memory
        .rounds
        .get_mut(&game_number)
        .unwrap()
        .push((deck1.clone(), deck2.clone()));

    if deck1.is_empty() {
        GameStepResult::Finished(Player(2))
    } else if deck2.is_empty() {
        GameStepResult::Finished(Player(1))
    } else {
        let card1 = deck1.take_card().unwrap();
        let card2 = deck2.take_card().unwrap();

        // Check for recursion
        let winning_player = {
            if deck1.len() >= card1.0 && deck2.len() >= card2.0 {
                let mut deck1_clone = deck1.clone_until(card1.0);
                let mut deck2_clone = deck2.clone_until(card2.0);
                run_recursive_game(&mut deck1_clone, &mut deck2_clone, memory, game_number + 1)
            } else {
                // Normal game
                if card1 > card2 {
                    Player(1)
                } else {
                    Player(2)
                }
            }
        };

        match winning_player {
            Player(1) => {
                deck1.add_card(card1);
                deck1.add_card(card2);
            }
            Player(2) => {
                deck2.add_card(card2);
                deck2.add_card(card1);
            }
            _ => unreachable!(),
        }

        GameStepResult::Running
    }
}

/// Run recursive game until completion.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
/// * `memory` - Game memory
/// * `game_number` - Game number
fn run_recursive_game(
    deck1: &mut Deck,
    deck2: &mut Deck,
    memory: &mut GameMemory,
    game_number: usize,
) -> Player {
    // Shortcuts for sub-games
    if game_number > 1 {
        let max_deck1_card = deck1.0.iter().max().unwrap();
        let max_deck2_card = deck2.0.iter().max().unwrap();

        if max_deck1_card > max_deck2_card {
            // Player 1 should win, because only he can win when infinite loop detected
            return Player(1);
        }
    }

    // Prepare rounds for game
    memory.rounds.entry(game_number).or_insert(vec![]);

    loop {
        if let GameStepResult::Finished(player) =
            recursive_game_step(deck1, deck2, memory, game_number)
        {
            // Remove unneeded memory
            memory.rounds.remove(&game_number);
            return player;
        }
    }
}

/// Calculate score for deck.
///
/// # Arguments
///
/// * `deck` - Deck
pub fn calculate_score(deck: &Deck) -> usize {
    let deck_size = deck.0.len();
    deck.0
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, card)| acc + (deck_size - idx) * card.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 31754;
    const EX2_OUTPUT: usize = 35436;

    const SAMPLE: &str = indoc::indoc! {"
        Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10
    "};

    const INFINITE_SAMPLE: &str = indoc::indoc! {"
        Player 1:
        43
        19

        Player 2:
        2
        29
        14
    "};

    #[test]
    fn test_parse_decks() {
        let ((player1, deck1), (player2, deck2)) = parse_decks(SAMPLE);
        assert_eq!(player1, Player(1));
        assert_eq!(deck1.0, vec![Card(9), Card(2), Card(6), Card(3), Card(1)]);

        assert_eq!(player2, Player(2));
        assert_eq!(deck2.0, vec![Card(5), Card(8), Card(4), Card(7), Card(10)]);
    }

    #[test]
    fn test_game_step() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);

        let result = game_step(&mut deck1, &mut deck2);
        assert_eq!(result, GameStepResult::Running);
        assert_eq!(
            deck1.0,
            vec![Card(2), Card(6), Card(3), Card(1), Card(9), Card(5)]
        );
        assert_eq!(deck2.0, vec![Card(8), Card(4), Card(7), Card(10)]);
    }

    #[test]
    fn test_run_game() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);

        run_game(&mut deck1, &mut deck2);
        assert_eq!(deck1.0, vec![]);
        assert_eq!(
            deck2.0,
            vec![
                Card(3),
                Card(2),
                Card(10),
                Card(6),
                Card(8),
                Card(5),
                Card(9),
                Card(4),
                Card(7),
                Card(1)
            ]
        );

        assert_eq!(calculate_score(&deck1), 0);
        assert_eq!(calculate_score(&deck2), 306);
    }

    #[test]
    fn test_run_recursive_game_infinite() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(INFINITE_SAMPLE);
        let mut memory = GameMemory::default();

        let player = run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1);
        assert_eq!(player, Player(1));
    }

    #[test]
    fn test_run_recursive_game_sample() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);
        let mut memory = GameMemory::default();

        let player = run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1);
        assert_eq!(player, Player(2));
        assert_eq!(deck1.0, vec![]);
        assert_eq!(
            deck2.0,
            vec![
                Card(7),
                Card(5),
                Card(6),
                Card(2),
                Card(4),
                Card(1),
                Card(10),
                Card(8),
                Card(9),
                Card(3)
            ]
        );
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
