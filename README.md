# Impostors games better algorithm

## What is it

Small library aiming to reduce the probability to be chosen multiple times in a row as impostor in games like AmongUs.

It is just an exercise for myself, so don't expect much from it.

## Why

As noticed by players, in AmongUs - or other similar games - sessions, there is often players who are impostors a lot, and others who never play as impostor, showing that the player distribution algorithm is too random and kinda broken. So as an exercise, I wanted to try to create a better role distribution algorithm.

## How does it works

The library currently uses a weight system, giving an even probability for all players to be chosen at start, and then decreasing this probability for chosen players. This probability is then increased (slower than it has been decreased) when the player is not chosen, until it reaches its value at start again.

Currently, the start weight is set to 64. The weight is divided by four when the player is chosen, and doubled when the player is not chosen.

## Examples

Run `cargo run --example 10players` to get a demo of the library, with a 10 players game and 20 rounds.
