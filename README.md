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
| `poke catch [ball]` | Throw a ball with a shake animation (`poke`, `great`, `ultra`, `net`, `nest`, `repeat`, `quick`, `timer`, `master`) |
| `poke berry` | Feed a Razz Berry (x1.5 catch rate, no fleeing for one throw) |
| `poke run` | Flee from the wild encounter |
| `poke config` | Show active settings and config file locations |
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

Throws follow the Generation 8 capture mechanics (https://dragonflycave.com/mechanics/gen-viii-capturing):

- Catch value `X = (3M - 2H) / 3M * rate * ball * low-level * berry`. Wild Pokémon always have full HP.
- Low-level bonus `(30 - level) / 10` below level 21.
- Shake check `Y = 65536 / (255 / X)^(3/16)`. A normal throw needs four checks to pass. `X >= 255` always catches.
- Critical capture: one check instead of four, chance `floor(min(255, X) * P / 6) / 256`. `P` grows with species caught: 0 up to 30, 0.5 up to 150, 1.0 up to 300, 1.5 up to 450, 2.0 up to 600, 2.5 above.
- Razz Berry counts as a status ailment (x1.5) and also stops fleeing for one throw. A failed throw otherwise lets the Pokémon flee 25% of the time.

| Ball | Price | Bonus |
|---|---|---|
| Poké Ball | 200 | 1x |
| Great Ball | 600 | 1.5x |
| Ultra Ball | 800 | 2x |
| Net Ball | 1,000 | 3.5x on Water and Bug |
| Nest Ball | 1,000 | `(41 - level) / 10` below level 30 |
| Repeat Ball | 1,000 | 3.5x on species already caught |
| Quick Ball | 1,000 | 5x on the first throw |
| Timer Ball | 1,000 | `1 + throws * 1229/4096`, max 4x |
| Master Ball | 50,000 | never fails |

### Economy and streak

- Starting package: 15 Poké Balls, 5 Great Balls, 1 Ultra Ball, 3 Razz Berries, and 500 ₽.
- Daily bonus: `200 + 20 * streak` ₽ (streak counted up to 30) and 5 Poké Balls. Streak milestones add balls: every 3 days 2 Great Balls, every 7 days 1 Ultra Ball, every 30 days 1 Master Ball. Missing a day resets the streak.
- Capture payout: `150 + 8 * level + 3 * (255 - catch rate)` ₽. Harder species pay more. A new Pokédex entry adds 500 ₽ and a shiny adds 2,500 ₽.
- Releasing a Pokémon to Professor Oak pays `25 + 5 * level` ₽ (+500 ₽ if shiny).

## Configuration

Settings live in `~/.config/poke/config.yml`, or `~/.poke.yml` if the first does not exist. Every key is optional. Unknown keys are rejected so typos show up as a warning.

```yaml
max_gen: unova            # 1-8 or a region: kanto johto hoenn sinnoh unova kalos alola galar
animation: true           # shake animation on 'poke catch' (also skipped when output is piped)
shiny_rate: 4096          # shiny odds as 1/N; omit for 4096 (1024 with the Shiny Charm)
encounter_timeout_secs: 600
```

`max_gen: unova` limits encounters to Black and White era Pokémon and earlier. Run `poke config` to see what is active.

## License

MIT
