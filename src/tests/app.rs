use super::*;
use crate::models::SearchResultHierarchy;

fn result(result_type: &str) -> SearchResult {
    SearchResult {
        object_id: "hooks".to_owned(),
        result_type: result_type.to_owned(),
        url: "https://react.dev/reference/react/hooks".to_owned(),
        hierarchy: SearchResultHierarchy {
            lvl0: "React &amp; APIs".to_owned(),
            lvl1: Some("Built-in React Hooks".to_owned()),
            lvl2: None,
            lvl3: None,
            lvl4: None,
            lvl5: None,
            lvl6: None,
        },
        content: None,
    }
}

#[test]
fn items_from_results_preserves_provider_order() -> Result<()> {
    let mut second = result("content");
    second.object_id = "second".to_owned();

    let items = items_from_results(&[result("lvl1"), second])?;

    assert_eq!(
        items.iter().map(Item::uid).collect::<Vec<_>>(),
        vec![Some("hooks"), Some("second")]
    );
    Ok(())
}

#[test]
fn content_item_uses_root_title_without_breadcrumb() -> Result<()> {
    let items = items_from_results(&[result("content")])?;

    assert_eq!(
        (items[0].title(), items[0].subtitle()),
        ("React &amp; APIs", None)
    );
    Ok(())
}

#[test]
fn item_uses_selected_hierarchy_level_as_title() -> Result<()> {
    let items = items_from_results(&[result("lvl1")])?;

    assert_eq!(items[0].title(), "Built-in React Hooks");
    Ok(())
}

#[test]
fn breadcrumb_excludes_title_and_decodes_html_entities() -> Result<()> {
    let items = items_from_results(&[result("lvl1")])?;

    assert_eq!(items[0].subtitle(), Some("React & APIs"));
    Ok(())
}

#[test]
fn breadcrumb_excludes_repeated_title_values() -> Result<()> {
    let mut search_result = result("lvl2");
    search_result.hierarchy.lvl0 = "Built-in React Hooks".to_owned();
    search_result.hierarchy.lvl2 = Some("Built-in React Hooks".to_owned());

    let items = items_from_results(&[search_result])?;

    assert_eq!(items[0].subtitle(), Some(""));
    Ok(())
}

#[test]
fn breadcrumb_is_truncated_to_seventy_five_characters() -> Result<()> {
    let mut search_result = result("lvl2");
    search_result.hierarchy.lvl0 = "A".repeat(50);
    search_result.hierarchy.lvl1 = Some("B".repeat(50));
    search_result.hierarchy.lvl2 = Some("Selected".to_owned());

    let items = items_from_results(&[search_result])?;
    let subtitle = items[0].subtitle().expect("subtitle must be present");

    assert_eq!(
        (subtitle.chars().count(), subtitle.ends_with("...")),
        (75, true)
    );
    Ok(())
}

#[test]
fn truncation_preserves_unicode_character_boundaries() {
    let value = "🦀".repeat(80);
    let truncated = truncate(&value, 75);

    assert_eq!(
        (truncated.chars().count(), truncated.ends_with("...")),
        (75, true)
    );
}

#[test]
fn item_preserves_url_fields() -> Result<()> {
    let items = items_from_results(&[result("lvl1")])?;
    let item = &items[0];

    assert_eq!(
        (item.arg(), item.quick_look_url(), item.valid()),
        (
            Some("https://react.dev/reference/react/hooks"),
            Some("https://react.dev/reference/react/hooks"),
            true,
        )
    );
    Ok(())
}

#[test]
fn item_rejects_missing_selected_hierarchy_level() {
    let error = items_from_results(&[result("lvl2")])
        .expect_err("missing hierarchy level must be rejected");

    assert_eq!(
        error.to_string(),
        "Algolia result hooks is missing hierarchy level 2"
    );
}

#[test]
fn item_rejects_invalid_result_type() {
    let error =
        items_from_results(&[result("heading")]).expect_err("invalid result type must be rejected");

    assert_eq!(error.to_string(), "invalid Algolia result type: heading");
}

#[test]
fn item_rejects_out_of_range_result_type() {
    let error =
        items_from_results(&[result("lvl7")]).expect_err("out-of-range type must be rejected");

    assert_eq!(error.to_string(), "invalid Algolia result type: lvl7");
}

#[test]
fn google_fallback_encodes_query_and_is_selectable() -> Result<()> {
    let item = google_fallback_item("state hooks")?;

    assert_eq!(
        (item.arg(), item.valid()),
        (
            Some("https://www.google.com/search?q=React+state+hooks"),
            true
        )
    );
    Ok(())
}

#[test]
fn placeholder_is_not_selectable() {
    assert!(!placeholder_item().valid());
}
