use clap::{Parser, Subcommand};

/// Terminal Pokemon encounter and game CLI.
#[derive(Parser, Debug)]
#[command(name = "poke", author, version, about = "Terminal Pokemon encounters and game")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Fallback pokemon name or ID to display directly.
    #[arg(index = 1)]
    pub pokemon: Option<String>,

    /// Render fallback sprite in shiny form.
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,
}

/// Available CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Spawn or view the current wild Pokemon encounter.
    Spawn {
        /// Force roll a brand new encounter immediately.
        #[arg(short, long, default_value_t = false)]
        force: bool,

        /// Limit random encounter to a specific generation (1-8).
        #[arg(short, long)]
        gen: Option<u8>,
    },

    /// Throw a Poké Ball to catch the wild Pokemon.
    Catch {
        /// Ball type to throw: poke, great, ultra, or master.
        #[arg(default_value = "poke")]
        ball: String,
    },

    /// Feed a Razz Berry to calm the wild Pokemon.
    Berry,

    /// Flee from the wild Pokemon.
    Run,

    /// Inspect your trainer wallet and bag inventory.
    Bag,

    /// Browse items and balls in the Poké Mart.
    Shop,

    /// Purchase items from the shop.
    Buy {
        /// Item to buy: poke, great, ultra, master, berry, charm.
        item: String,

        /// Number of items to buy.
        #[arg(default_value_t = 1)]
        quantity: u32,
    },

    /// View Pokédex stats or inspect a specific species.
    Dex {
        /// Pokemon name or National Dex ID to inspect.
        query: Option<String>,
    },

    /// View all captured Pokémon in your PC storage box.
    Box,

    /// Release a Pokémon to Professor Oak for a reward.
    Release {
        /// Box index number of the Pokémon to release.
        index: usize,
    },

    /// Claim your daily trainer reward and check streak.
    Daily,

    /// Output a compact one-line summary for prompt integration.
    Status,

    /// Display a Pokemon sprite directly (drop-in compatibility).
    Get {
        /// Pokemon name or ID.
        pokemon: String,

        /// Display shiny sprite variant.
        #[arg(short, long, default_value_t = false)]
        shiny: bool,
    },
}
