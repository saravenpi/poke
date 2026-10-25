use clap::{CommandFactory, Parser};
use poke::anim;
use poke::ball::BallType;
use poke::catch;
use poke::config::{self, Config};
use poke::cli::{Cli, Commands};
use poke::daily;
use poke::dex;
use poke::encounter;
use poke::lang::Language;
use poke::models::TrainerState;
use poke::species;
use poke::sprites;
use poke::storage;

/// Handles spawning and printing wild encounter to terminal.
fn handle_spawn(force: bool, gen: Option<u8>, config: &Config) {
    let mut state = storage::load_state();
    if let Some(daily_msg) = daily::check_daily(&mut state) {
        println!("{}\n", daily_msg);
    }
    match encounter::get_or_spawn_encounter(&mut state, config, force, gen) {
        Ok(enc) => {
            if let Some(sprite) = sprites::render_sprite(&enc.slug, enc.shiny) {
                println!("{}", sprite);
            }
            println!("{}", encounter::format_encounter(&enc, config.language));
        }
        Err(err) => eprintln!("{}", err),
    }
}

/// Throws a ball at the active wild Pokemon and plays the result.
fn handle_catch(ball_str: &str, config: &Config) {
    let Some(ball) = BallType::parse(ball_str) else {
        eprintln!("Unknown ball '{}'. Run 'poke shop' to see them all.", ball_str);
        return;
    };
    match catch::attempt_catch(&mut storage::load_state(), ball) {
        Ok(attempt) => anim::play(&attempt, config),
        Err(err) => eprintln!("{}", err),
    }
}

/// Prints the effective settings and config file locations.
fn handle_config(config: &Config) {
    let limit = config.generation_limit().map_or_else(|e| e, |g| format!("1-{}", g));
    println!("Generation limit:  {}", limit);
    println!("Language:          {}", config.language.code());
    println!("Animation:         {}", config.animation);
    println!("Encounter timeout: {}s", config.encounter_timeout_secs);
    println!("Config files (first found wins):");
    for path in config::config_paths() {
        println!("  {}", path.display());
    }
}

/// Directly renders a Pokemon sprite by name or ID.
fn handle_direct_display(pokemon: &str, shiny: bool) {
    let slug = species::find_by_query(pokemon).map_or(pokemon, |s| s.slug.as_str());
    if let Some(sprite) = sprites::render_sprite(slug, shiny) {
        println!("{}", sprite);
    } else {
        eprintln!("Pokemon '{}' not found.", pokemon);
    }
}

/// Handles Pokédex inspection and summaries.
fn handle_dex(query: Option<String>, state: &TrainerState, lang: Language) {
    match query {
        Some(q) => match dex::inspect_pokemon(&q, state, lang) {
            Ok(details) => println!("{}", details),
            Err(err) => eprintln!("{}", err),
        },
        None => println!("{}", dex::dex_summary(state)),
    }
}

/// Dispatches bag, store, dex, and trainer management operations.
fn dispatch_trainer(cmd: Commands, config: &Config) {
    let mut state = storage::load_state();
    match cmd {
        Commands::Config => handle_config(config),
        Commands::Bag => println!("{}", poke::bag::format_bag(&state)),
        Commands::Shop => println!("{}", poke::bag::format_shop(&state)),
        Commands::Buy { item, quantity } => match poke::bag::buy_item(&mut state, &item, quantity) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Commands::Dex { query } => handle_dex(query, &state, config.language),
        Commands::Box => println!("{}", dex::list_box(&state, config.language)),
        Commands::Release { index } => match dex::release_pokemon(&mut state, index, config.language) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Commands::Daily => match daily::check_daily(&mut state) {
            Some(msg) => println!("{}", msg),
            None => println!("Daily bonus already claimed! Streak: {} days.", state.streak),
        },
        Commands::Status => println!("{}", dex::status_line(&state)),
        Commands::Get { pokemon, shiny } => handle_direct_display(&pokemon, shiny),
        _ => {}
    }
}

/// Entry point parsing arguments and executing commands.
fn main() {
    let cli = Cli::parse();
    let config = config::load_config();
    match cli.command {
        None => match cli.pokemon {
            Some(name) => handle_direct_display(&name, cli.shiny),
            None => {
                let _ = Cli::command().print_help();
                println!();
            }
        },
        Some(Commands::Spawn { force, gen }) => handle_spawn(force, gen, &config),
        Some(Commands::Catch { ball }) => handle_catch(&ball, &config),
        Some(Commands::Berry) => match catch::feed_berry(&mut storage::load_state(), config.language) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Some(Commands::Run) => match catch::flee(&mut storage::load_state()) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Some(cmd) => dispatch_trainer(cmd, &config),
    }
}
