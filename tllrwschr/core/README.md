# tllrwschr-core

## Constraints

System has following constraints:

- Only games with teams and players can be simulated
- Inputs are either digital Button, text or number inputs
- Slave devices can only send events, no orders. They can receive orders from the central unit
- Master devices can send orders and must receive a response, even if only a OK!

## Idle mode

Idle mode is the default mode of the program.
In this mode you can select a game to start or settings to adjust.

## Director

Once a game is selected, the game logic can be started through the `Director`.
The director reads the game's instructions and schedules the appropiate behaviours.
It reacts and reacts based on the rules of the game. The `Director` is also responsible
for ending a running game.

