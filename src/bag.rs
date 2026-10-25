use crate::ball::{BallType, ALL_BALLS};
use crate::models::TrainerState;
use crate::storage;

const BERRY_PRICE: u32 = 300;
const CHARM_PRICE: u32 = 25_000;

/// Something the Poké Mart sells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Ball(BallType),
    Berry,
    Charm,
}

impl Item {
    /// Resolves a shop keyword to an item.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_lowercase().as_str() {
            "berry" | "razz" => Some(Self::Berry),
            "charm" | "shinycharm" => Some(Self::Charm),
            other => BallType::parse(other).map(Self::Ball),
        }
    }

    /// Unit price in Pokédollars.
    pub fn price(self) -> u32 {
        match self {
            Self::Ball(ball) => ball.price(),
            Self::Berry => BERRY_PRICE,
            Self::Charm => CHARM_PRICE,
        }
    }
}

/// Formats current wallet and inventory contents.
pub fn format_bag(state: &TrainerState) -> String {
    let inv = &state.inventory;
    let mut out = format!("=== Trainer Bag ===\nWallet: {} ₽\n\nBalls:\n", state.pokedollars);
    for ball in ALL_BALLS.iter().filter(|b| inv.count(**b) > 0) {
        out.push_str(&format!("  {:<12} x{}\n", ball.name(), inv.count(*ball)));
    }
    out.push_str(&format!(
        "\nItems:\n  {:<12} x{}\n  {:<12} {}",
        "Razz Berry",
        inv.razz_berries,
        "Shiny Charm",
        if inv.shiny_charm { "active" } else { "none" }
    ));
    out
}

/// Formats the shop catalogue and prices.
pub fn format_shop(state: &TrainerState) -> String {
    let mut out = format!("=== Poké Mart ===\nWallet: {} ₽\n\n", state.pokedollars);
    for ball in ALL_BALLS {
        out.push_str(&format!("  {:<7} {:<12} {:>6} ₽  {}\n", ball.key(), ball.name(), ball.price(), ball.blurb()));
    }
    let extras = [
        ("berry", "Razz Berry", BERRY_PRICE, "+50% catch rate, no fleeing for one throw"),
        ("charm", "Shiny Charm", CHARM_PRICE, "shiny odds x4"),
    ];
    for (key, name, price, blurb) in extras {
        out.push_str(&format!("  {:<7} {:<12} {:>6} ₽  {}\n", key, name, price, blurb));
    }
    out.push_str("\nUse 'poke buy <item> [quantity]' to purchase.");
    out
}

/// Purchases an item from the shop if funds are sufficient.
pub fn buy_item(state: &mut TrainerState, raw: &str, qty: u32) -> Result<String, String> {
    let item = Item::parse(raw).ok_or_else(|| format!("Unknown shop item: '{}'. Run 'poke shop'.", raw))?;
    if item == Item::Charm && state.inventory.shiny_charm {
        return Err("You already own the Shiny Charm!".to_string());
    }
    let count = if item == Item::Charm { 1 } else { qty.max(1) };
    let total = item.price().saturating_mul(count);
    if state.pokedollars < total {
        return Err(format!("Not enough Poké Dollars! Need {} ₽, have {} ₽.", total, state.pokedollars));
    }

    state.pokedollars -= total;
    match item {
        Item::Ball(ball) => state.inventory.add(ball, count),
        Item::Berry => state.inventory.razz_berries = state.inventory.razz_berries.saturating_add(count),
        Item::Charm => state.inventory.shiny_charm = true,
    }
    storage::save_state(state)?;
    Ok(format!("Purchased {}x {} for {} ₽!", count, raw.trim(), total))
}
