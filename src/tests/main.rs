use std::cell::Cell;

use alfred_workflow_rs::FileCache;

use super::*;

fn settings() -> WorkflowSettings {
    WorkflowSettings {
        use_alfred_cache: false,
        use_file_cache: false,
        cache_ttl: Some(86_400),
        file_cache_max_entries: Some(1_280),
    }
}

#[test]
fn plist_defaults_map_to_runtime_settings() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let info_path = directory.path().join("info.plist");
    std::fs::write(&info_path, include_str!("../../info.plist"))?;

    let actual = read_workflow_settings(
        &Workflow::new(),
        info_path,
        directory.path().join("missing-prefs.plist"),
    )?;

    assert_eq!(
        actual,
        WorkflowSettings {
            use_alfred_cache: true,
            use_file_cache: false,
            cache_ttl: Some(86_400),
            file_cache_max_entries: Some(1_280),
        }
    );
    Ok(())
}

#[test]
fn missing_settings_use_safe_defaults() {
    assert_eq!(
        workflow_settings_from_defaults(&BTreeMap::new()),
        WorkflowSettings {
            use_alfred_cache: false,
            use_file_cache: false,
            cache_ttl: None,
            file_cache_max_entries: None,
        }
    );
}

#[test]
fn automatic_cache_wins_when_both_modes_are_enabled() {
    let mut workflow = Workflow::new();
    let mut settings = settings();
    settings.use_alfred_cache = true;
    settings.use_file_cache = true;

    configure_cache(&mut workflow, "hooks", &settings);

    assert_eq!(
        (workflow.use_automatic_cache(), workflow.cache_key()),
        (true, None)
    );
}

#[test]
fn file_cache_key_is_the_normalized_query() {
    let mut workflow = Workflow::new();
    let mut settings = settings();
    settings.use_file_cache = true;

    configure_cache(&mut workflow, "state hooks", &settings);

    assert_eq!(workflow.cache_key(), Some("state hooks"));
}

#[test]
fn empty_query_shows_placeholder_without_searching() -> Result<()> {
    let search_calls = Cell::new(0);
    let cli = Cli::default();
    let mut workflow = Workflow::new();

    populate_workflow_with(&mut workflow, &cli, &settings(), |_| {
        search_calls.set(search_calls.get() + 1);
        Ok(Vec::new())
    })?;

    assert_eq!(
        (search_calls.get(), workflow.get_items()?.items()[0].title()),
        (0, "Search the React docs...")
    );
    Ok(())
}

#[test]
fn file_cache_hit_bypasses_algolia() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut cached = Workflow::with_file_cache(FileCache::with_path(directory.path()));
    cached.set_cache_key(Some("hooks"));
    cached.add_item(Item::new("cached result"))?;

    let mut workflow = Workflow::with_file_cache(FileCache::with_path(directory.path()));
    let mut settings = settings();
    settings.use_file_cache = true;
    let search_calls = Cell::new(0);
    let cli = Cli {
        query: "hooks".to_owned(),
        ..Cli::default()
    };

    populate_workflow_with(&mut workflow, &cli, &settings, |_| {
        search_calls.set(search_calls.get() + 1);
        Ok(Vec::new())
    })?;

    assert_eq!(
        (search_calls.get(), workflow.get_items()?.items()[0].title()),
        (0, "cached result")
    );
    Ok(())
}

#[test]
fn no_results_render_google_fallback() -> Result<()> {
    let mut workflow = Workflow::new();
    let cli = Cli {
        query: "missing topic".to_owned(),
        ..Cli::default()
    };

    populate_workflow_with(&mut workflow, &cli, &settings(), |_| Ok(Vec::new()))?;

    assert_eq!(
        workflow.get_items()?.items()[0].arg(),
        Some("https://www.google.com/search?q=React+missing+topic")
    );
    Ok(())
}

#[test]
fn update_item_is_rendered_without_entering_file_cache() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut workflow = Workflow::with_file_cache(FileCache::with_path(directory.path()));
    workflow.set_cache_key(Some("hooks"));
    workflow.add_item(Item::new("search result"))?;
    let options = update_render_options_with(&Cli::default(), || Ok(true));

    let rendered: serde_json::Value =
        serde_json::from_str(&workflow.to_json_string_with(options)?)?;
    let cached = workflow.get_items()?;

    assert_eq!(
        (
            rendered["items"].as_array().map(Vec::len),
            cached.len(),
            cached.items()[0].title()
        ),
        (Some(2), 1, "search result")
    );
    Ok(())
}

#[test]
fn updater_check_failure_does_not_change_rendered_results() -> Result<()> {
    let mut workflow = Workflow::new();
    workflow.add_item(Item::new("search result"))?;
    let options = update_render_options_with(&Cli::default(), || Err(anyhow::anyhow!("offline")));

    let rendered: serde_json::Value =
        serde_json::from_str(&workflow.to_json_string_with(options)?)?;

    assert_eq!(rendered["items"].as_array().map(Vec::len), Some(1));
    Ok(())
}
