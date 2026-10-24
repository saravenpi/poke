# poke

Terminal Pokemon encounters, catching, Pokédex tracking, and economy.

Runs on shell startup. Displays a wild Pokemon without blocking your terminal prompt. If you want to play, catch it with commands. If not, continue your terminal session uninterrupted.

## Installation

### With cargo

```sh
cargo install --path .
```

### With facile

```sh
facile install poke
```

## Shell setup

Add the following to your `~/.zshrc` or `~/.bashrc`:

```sh
command -v poke >/dev/null 2>&1 && poke spawn
```

Each new terminal session rolls a wild encounter using authentic game apparition rates. Encounters stay active for 10 minutes or until caught or fled.

## Commands

| Command | Description |
|---|---|
| `poke` | Display help and available commands |
| `poke spawn` | Display or roll the current wild encounter |
| `poke spawn --force` | Force roll a new encounter immediately |
| `poke catch [ball]` | Throw a ball (`poke`, `great`, `ultra`, `master`) |
| `poke berry` | Feed a Razz Berry (+50% catch rate, stops flee for 1 turn) |
| `poke run` | Flee from the wild encounter |
| `poke bag` | View wallet and inventory |
| `poke shop` | Browse items for sale |
| `poke buy <item> [qty]` | Buy balls, berries, or the Shiny Charm |
| `poke dex` | View Pokédex stats overview |
| `poke dex <pokemon>` | Inspect a Pokemon's stats and encounter history |
| `poke box` | List all captured Pokémon in storage |
| `poke release <index>` | Release a duplicate to Professor Oak for cash |
| `poke daily` | Check daily login streak and claim bonus |
| `poke status` | Compact one-line summary for starship or prompt scripts |
| `poke <name>` | Render a specific Pokemon sprite directly |

## Mechanics

### Apparition rates

Encounters use official National Dex generation tables and species capture rates to weight spawns:

- Common (Pidgey, Rattata, Bidoof, Wooloo): high encounter frequency
- Uncommon (Machop, Bellsprout, Geodude): moderate frequency
- Rare (Starters, Pikachu, Eevee, Dratini): low frequency
- Epic (Dragonite, Tyranitar, Garchomp): very rare
- Legendary & Mythical (Mewtwo, Zapdos, Rayquaza): extremely rare (1/1000 base weight)

### Capture formula

Catch calculations use the official capture score formula scaled with remaining HP, species capture rate, ball multiplier, and berry bonuses. Master Balls guarantee a capture. If a throw fails, wild Pokémon have a chance to flee unless calmed by a Razz Berry.

### Economy and streak

- Starting package: 15 Poké Balls, 5 Great Balls, 1 Ultra Ball, 3 Razz Berries, and 500 ₽.
- Daily logins award 250+ ₽ and 5 Poké Balls, with milestone bonuses for 3-day and 7-day streaks.
- Successful captures award cash based on Pokémon level and rarity.
- Releasing duplicates to Professor Oak grants cash bounties.

## License

MIT
