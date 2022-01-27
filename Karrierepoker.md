# Karrierepoker algorithm

## Keymapping

`select_player` -> `SlaveInput::ButtonPress { primary }`
`confirm` -> `SlaveInput::ButtonPress { secondary }`
`skip` -> `SlaveInput::ButtonPress { primary }`

For all other devices send out a keymap list:

Vec<ButtonSettings>

## Algorithm

`Input, Master: PhaseCommand { abort_game }` aborts entire game plan, sets back to menu
    - Respond with `Result<(), Infallible>`

Request/Response method:
- Slave input fn: SlaveInput
- Master input fn: MasterInput
- Response signature: -> (Option(Vec<GameResponse>), Option(PhaseEvent))

### Setup phase

`Player` contains player id

- `(min,max) players`
- `all_players : Player[]`

- Wait for `Input, Master, GameSelected`
  - Respond with 
```
// Display options on screen which are available during the phase
Output, Master, DisplayOptions { only_phase: bool, style_options: opt: Vec<String> 
    {
        "phase_restart"
        "phase_end"
    }})
```

- For each `Input, Slave, ButtonPressDigital { any, player }`, add `player` to `all_players`
    - Respond `Output, All, Some(PlayerAdded { player_id })`
    - Respond with `Output, All, `

- OR UNTIL `Input, Master: end_phase`,
  - Respond with `Output, Game, Result<PhaseDone { init : Phase, }, PhaseNotEndedError>`

- If phase is done, go to next phase

### Game phase

- Game response

- carry over `all_players` Simply as read-only reference now
- `finished_players` All players which have no cards left, in correct order
- `round_active_players` All active players in the round (those with cards left)
- `turn_active_players` Current turn only, until one player makes a trick or everyone save one has skipped

- Wait for EITHER
    - Confirm `Input, Slave, ButtonPressDigital { confirm, current_player }`
      - Respond with `Output, Slave[curent_player], ToggleLight { false }`
      - Respond with `Output, Slave[next_player], ToggleLight { true }`
      - Respond with `Output, Master, SetCurrentPlayer { player_id }`
    - OR Skip `Input, Slave, ButtonPressDigital { skip, current_player }`
      - Remove `current_player` from `turn_active_players`
      - Respond with `Output, Slave[next_player], ToggleLight { true }`
    - OR `Input, Master, Order { "skip_player": String: }`
- Then set `current_player = next_player`, etc.