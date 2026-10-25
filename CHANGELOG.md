# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-10-25

### Added

- `language` config setting (`en`, `ja`, `fr`, `es`, `de`) for Pokémon names. Lookups accept names in any language.

### Fixed

- Nidoran #29 and #32 had swapped names (#29 is the female, #32 the male).

## [0.2.0] - 2026-10-25

### Added

- Shake animation on `poke catch` (terminal only, disable with `animation: false`).
- Net, Nest, Repeat, Quick and Timer balls with Generation 8 multipliers.
- Critical captures, low-level catch bonus and exact Generation 8 shake checks.
- `~/.config/poke/config.yml` and `~/.poke.yml`: `max_gen` (number or region), `animation`, `shiny_rate`, `encounter_timeout_secs`.
- `poke config` shows the active settings.
- New Pokédex entry bonus and a Master Ball for every 30-day streak.

### Changed

- Capture payouts scale with species difficulty; daily cash grows with the streak up to 30 days.
- Ultra Ball costs 800 ₽.
- Ball counts reset to the starter kit on first run (wallet, Pokédex and box are kept).
- `POKE_SHINY_RATE` is replaced by `shiny_rate` in the config.

### Fixed

- `buy` no longer overflows on huge quantities.
- `spawn --gen` rejects values outside 1-8 instead of panicking.
- A save that fails to parse is copied to `state.json.bak` before being replaced.
- Save errors while spawning are reported instead of ignored.

## [0.1.1] - 2026-10-24

### Changed

- `poke`: running with no arguments displays the help message instead of spawning an encounter.

## [0.1.0] - 2026-10-24

### Added

- Non-blocking terminal Pokemon encounters on shell startup.
- Authentic generation apparition rates and catch calculations for 905 species.
- Economy system with Pokédollars (₽), daily login streaks, and Poké Mart shop.
- Inventory management for Poké Balls, Great Balls, Ultra Balls, Master Balls, and Razz Berries.
- Pokédex tracking for seen and caught species with shiny detection.
- PC storage box and release bounties to Professor Oak.
- Compact one-line status prompt integration (`poke status`).
- Direct sprite rendering compatibility (`poke <name>`).
- Facile package manager and mise support.
