use clap::Parser;
use poke::catch::{self, CatchOutcome};
use poke::cli::{Cli, Commands};
use poke::daily;
use poke::dex;
use poke::encounter;
use poke::models::{BallType, TrainerState};
use poke::sprites;
use poke::storage;

/// Resolves user ball input string to typed BallType enum.
fn parse_ball(raw: &str) -> BallType {
    match raw.to_lowercase().as_str() {
        "great" | "greatball" | "blue" => BallType::Great,
        "ultra" | "ultraball" | "yellow" => BallType::Ultra,
        "master" | "masterball" | "purple" => BallType::Master,
        _ => BallType::Poke,
    }
}

/// Handles spawning and printing wild encounter to terminal.
fn handle_spawn(force: bool, gen: Option<u8>) {
    let mut state = storage::load_state();
    if let Some(daily_msg) = daily::check_daily(&mut state) {
        println!("{}\n", daily_msg);
    }
    let enc = encounter::get_or_spawn_encounter(&mut state, force, gen);
    if let Some(sprite) = sprites::render_sprite(&enc.slug, enc.shiny) {
        println!("{}", sprite);
    }
    println!("{}", encounter::format_encounter(&enc));
}

/// Executes a Poké Ball throw against the active wild Pokemon.
fn handle_catch(ball_str: &str) {
    let mut state = storage::load_state();
    let ball = parse_ball(ball_str);
    match catch::attempt_catch(&mut state, ball) {
        Ok(CatchOutcome::Caught { pokemon, reward }) => {
            println!("*shake* ... *shake* ... *shake* ... *click*!");
            println!("Gotcha! {} was caught!", pokemon.name);
            println!("+{} ₽ earned. Added to Pokédex!", reward);
        }
        Ok(CatchOutcome::Escaped { shakes, slug }) => {
            let dots = "*shake* ... ".repeat(shakes as usize);
            println!("{}\nOh no! The wild {} broke free!", dots.trim(), slug);
            println!("Use 'poke catch' to try again or 'poke berry'.");
        }
        Ok(CatchOutcome::Fled { shakes, name }) => {
            let dots = "*shake* ... ".repeat(shakes as usize);
            println!("{}\nOh no! {} fled into the tall grass!", dots.trim(), name);
        }
        Err(err) => eprintln!("{}", err),
    }
}

/// Directly renders a Pokemon sprite by name or ID.
fn handle_direct_display(pokemon: &str, shiny: bool) {
    if let Some(sprite) = sprites::render_sprite(pokemon, shiny) {
        println!("{}", sprite);
    } else {
        eprintln!("Pokemon '{}' not found.", pokemon);
    }
}

/// Handles Pokédex inspection and summaries.
fn handle_dex(query: Option<String>, state: &TrainerState) {
    match query {
        Some(q) => match dex::inspect_pokemon(&q, state) {
            Ok(details) => println!("{}", details),
            Err(err) => eprintln!("{}", err),
        },
        None => println!("{}", dex::dex_summary(state)),
    }
}

/// Dispatches bag, store, dex, and trainer management operations.
fn dispatch_trainer(cmd: Commands) {
    let mut state = storage::load_state();
    match cmd {
        Commands::Bag => println!("{}", poke::bag::format_bag(&state)),
        Commands::Shop => println!("{}", poke::bag::format_shop(&state)),
        Commands::Buy { item, quantity } => match poke::bag::buy_item(&mut state, &item, quantity) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Commands::Dex { query } => handle_dex(query, &state),
        Commands::Box => println!("{}", dex::list_box(&state)),
        Commands::Release { index } => match dex::release_pokemon(&mut state, index) {
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
    match cli.command {
        None => match cli.pokemon {
            Some(name) => handle_direct_display(&name, cli.shiny),
            None => handle_spawn(false, None),
        },
        Some(Commands::Spawn { force, gen }) => handle_spawn(force, gen),
        Some(Commands::Catch { ball }) => handle_catch(&ball),
        Some(Commands::Berry) => match catch::feed_berry(&mut storage::load_state()) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Some(Commands::Run) => match catch::flee(&mut storage::load_state()) {
            Ok(msg) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
        },
        Some(cmd) => dispatch_trainer(cmd),
    }
}
