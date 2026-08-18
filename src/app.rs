use alfred_workflow_rs::{Icon, Item, ItemText};
use anyhow::{Result, anyhow};
use html_escape::decode_html_entities;
use url::Url;

use crate::models::SearchResult;

const BREADCRUMB_MAX_LENGTH: usize = 75;
const ELLIPSIS: &str = "...";

/// Builds the placeholder shown before the user enters a search query.
pub fn placeholder_item() -> Item {
    Item::new("Search the React docs...").set_icon(Icon::new("icon.png"))
}

/// Converts ranked React search results into Alfred items in provider order.
pub fn items_from_results(results: &[SearchResult]) -> Result<Vec<Item>> {
    results.iter().map(item_from_result).collect()
}

/// Builds the Google fallback shown when Algolia returns no hits.
pub fn google_fallback_item(query: &str) -> Result<Item> {
    let url = Url::parse_with_params(
        "https://www.google.com/search",
        [("q", format!("React {query}"))],
    )?;

    Ok(Item::builder("No matching answers found")
        .subtitle("Shall I try and search Google?")
        .arg(url.as_str())
        .text(ItemText::new(url.as_str()))
        .quick_look_url(url.as_str())
        .icon(Icon::new("google.png"))
        .valid(true)
        .build()?)
}

fn item_from_result(result: &SearchResult) -> Result<Item> {
    let level = result.hierarchy_level()?;
    let title = result.hierarchy.level(level).ok_or_else(|| {
        anyhow!(
            "Algolia result {} is missing hierarchy level {level}",
            result.object_id
        )
    })?;
    let mut builder = Item::builder(title)
        .uid(&result.object_id)
        .arg(&result.url)
        .text(ItemText::new(&result.url).with_large_type(title))
        .quick_look_url(&result.url)
        .icon(Icon::new("icon.png"))
        .valid(true);

    if level > 0 {
        let breadcrumb = result
            .hierarchy
            .values()
            .filter(|value| *value != title)
            .collect::<Vec<_>>()
            .join(" > ");
        let breadcrumb = decode_html_entities(&breadcrumb);
        builder = builder.subtitle(truncate(&breadcrumb, BREADCRUMB_MAX_LENGTH));
    }

    Ok(builder.build()?)
}

fn truncate(value: &str, max_length: usize) -> String {
    if value.chars().count() <= max_length {
        return value.to_owned();
    }

    let keep = max_length.saturating_sub(ELLIPSIS.chars().count());
    value.chars().take(keep).chain(ELLIPSIS.chars()).collect()
}

#[cfg(test)]
#[path = "tests/app.rs"]
mod tests;
