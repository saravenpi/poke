use showie::Trim;

use crate::Data;

/// Normalizes a Pokemon slug into a matching sprite filename.
pub fn clean_slug(slug: &str) -> String {
    slug.replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase()
}

/// Loads and renders a single Pokemon sprite into colored ANSI blocks.
pub fn render_sprite(slug: &str, shiny: bool) -> Option<String> {
    let clean = clean_slug(slug);
    let subfolder = if shiny { "shiny" } else { "regular" };
    let primary_path = format!("{}/{}.png", subfolder, clean);
    let fallback_path = format!("regular/{}.png", clean);

    let bytes = Data::get(&primary_path)
        .or_else(|| Data::get(&fallback_path))
        .or_else(|| Data::get("regular/unknown.png"))?
        .data;

    let img = image::load_from_memory(&bytes).ok()?.trim();
    Some(showie::render(&img))
}

