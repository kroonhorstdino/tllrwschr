# tllrwshr
A programmable game button system

#### Name origin
The name is derived from the lowest rank in the card game "Karrierepoker" (_en.: career poker_) called "Tellerwäscher" (_en.: dish washer_)


## Structure

`tllrwshr-core` is responsible for handling all the logic sorrounding the initialization and running of the game modes.
To be able to communicate, several additional crates are handling the I/O through terminals, guis, ssh, and so on.
For each communication protocol a `communicator` object handles the flow of information between `core` and `communicator`.
This is enabled through the use of `mpsc` channels. Either kind of device (master/slave) can be registered through this object
The default communicators are `terminal` and `ssh`. To start with additional communicators, cli arguments can be provided.
Adding more at runtime should also be possible.

### Communicators

Communicators are also responsible for handling the registering of master and slave devices.
Even if multiple master devices are registered, all retain the right to control the master unit.
For this to run smoothly, each device's state must be updated with events including those from master device inputs.

### Game plan

The game plan encodes the rules and actions of a game. It is used by the game runner to determine
which signals to use as triggerss and which actions to take next.

### tllrwshr-core

Provides the entire "game logic" for the master box. It does not handle displaying the state or commands.
IO is handled by the several "connector" crates.
