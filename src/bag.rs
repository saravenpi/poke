use crate::models::{BallType, Inventory, TrainerState};
use crate::storage;

/// Formats current wallet and inventory contents.
pub fn format_bag(state: &TrainerState) -> String {
    let inv = &state.inventory;
    let charm_status = if inv.shiny_charm { "Active" } else { "None" };
    format!(
        "=== Trainer Bag ===\n\
         Wallet: {} ₽\n\n\
         Poké Balls:\n  \
           ⚪ Poké Ball:    {}\n  \
           🔵 Great Ball:   {}\n  \
           🟡 Ultra Ball:   {}\n  \
           🟣 Master Ball:  {}\n\n\
         Items:\n  \
           🍓 Razz Berry:   {}\n  \
           ✨ Shiny Charm:  {}",
        state.pokedollars,
        inv.poke_balls,
        inv.great_balls,
        inv.ultra_balls,
        inv.master_balls,
        inv.razz_berries,
        charm_status
    )
}

/// Formats the shop catalogue and prices.
pub fn format_shop(state: &TrainerState) -> String {
    format!(
        "=== Poké Mart ===\n\
         Wallet: {} ₽\n\n\
         Items for sale:\n  \
           poke    - Poké Ball     (200 ₽)   - Standard capture rate\n  \
           great   - Great Ball    (600 ₽)   - 1.5x capture rate\n  \
           ultra   - Ultra Ball    (1,200 ₽) - 2.0x capture rate\n  \
           master  - Master Ball   (50,000 ₽)- 100% guaranteed catch\n  \
           berry   - Razz Berry    (300 ₽)   - Calms Pokemon & +50% catch rate\n  \
           charm   - Shiny Charm   (25,000 ₽)- Quadruples shiny spawn chances\n\n\
         Use 'poke buy <item> [quantity]' to purchase.",
        state.pokedollars
    )
}

/// Helper to add purchased quantity to trainer inventory.
fn apply_purchase(inv: &mut Inventory, item: &str, count: u32) {
    match item.to_lowercase().as_str() {
        "poke" | "pokeball" => inv.poke_balls += count,
        "great" | "greatball" => inv.great_balls += count,
        "ultra" | "ultraball" => inv.ultra_balls += count,
        "master" | "masterball" => inv.master_balls += count,
        "berry" | "razz" => inv.razz_berries += count,
        "charm" | "shinycharm" => inv.shiny_charm = true,
        _ => {}
    }
}

/// Purchases an item from the shop if funds are sufficient.
pub fn buy_item(state: &mut TrainerState, item: &str, qty: u32) -> Result<String, String> {
    let count = qty.max(1);
    let unit_price = match item.to_lowercase().as_str() {
        "poke" | "pokeball" => BallType::Poke.price(),
        "great" | "greatball" => BallType::Great.price(),
        "ultra" | "ultraball" => BallType::Ultra.price(),
        "master" | "masterball" => BallType::Master.price(),
        "berry" | "razz" => 300,
        "charm" | "shinycharm" => {
            if state.inventory.shiny_charm {
                return Err("You already own the Shiny Charm!".to_string());
            }
            25000
        }
        _ => return Err(format!("Unknown shop item: '{}'. Run 'poke shop'.", item)),
    };

    let total = unit_price * count;
    if state.pokedollars < total {
        return Err(format!("Not enough Poké Dollars! Need {} ₽, have {} ₽.", total, state.pokedollars));
    }

    state.pokedollars -= total;
    apply_purchase(&mut state.inventory, item, count);
    storage::save_state(state).map_err(|e| e.to_string())?;
    Ok(format!("Purchased {}x {} for {} ₽!", count, item, total))
}
