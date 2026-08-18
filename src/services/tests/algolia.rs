use serde_json::json;

use super::*;

fn config() -> AlgoliaSearchConfig {
    AlgoliaSearchConfig {
        application_id: "app".to_owned(),
        api_key: "key".to_owned(),
        index_name: "react".to_owned(),
    }
}

#[test]
fn endpoint_uses_single_index_search_route() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://127.0.0.1:8080/api/")?)?;

    assert_eq!(
        client.endpoint()?.as_str(),
        "http://127.0.0.1:8080/api/1/indexes/react/query"
    );
    Ok(())
}

#[test]
fn client_uses_platform_verifier_and_search_timeouts() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://localhost/")?)?;
    let timeouts = client.agent.config().timeouts();

    assert!(matches!(
        client.agent.config().tls_config().root_certs(),
        ureq::tls::RootCerts::PlatformVerifier
    ));
    assert_eq!(timeouts.connect, Some(CONNECT_TIMEOUT));
    assert_eq!(timeouts.global, Some(SEARCH_TIMEOUT));
    Ok(())
}

#[test]
fn response_limit_is_two_mebibytes() {
    assert_eq!(MAX_RESPONSE_BYTES, 2_097_152);
}

#[test]
fn request_body_preserves_react_search_contract() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://localhost/")?)?;
    let body: serde_json::Value = serde_json::from_str(&client.request_body("state hooks")?)?;

    assert_eq!(
        body,
        json!({
            "query": "state hooks",
            "attributesToRetrieve": [
                "hierarchy.lvl0", "hierarchy.lvl1", "hierarchy.lvl2",
                "hierarchy.lvl3", "hierarchy.lvl4", "hierarchy.lvl5",
                "hierarchy.lvl6", "content", "type", "url"
            ],
            "attributesToSnippet": [
                "hierarchy.lvl1:10", "hierarchy.lvl2:10", "hierarchy.lvl3:10",
                "hierarchy.lvl4:10", "hierarchy.lvl5:10", "hierarchy.lvl6:10",
                "content:10"
            ],
            "snippetEllipsisText": "...",
            "page": 0,
            "hitsPerPage": 20
        })
    );
    Ok(())
}

#[test]
fn search_response_deserializes_hierarchy() -> Result<()> {
    let response: SearchResponse = serde_json::from_value(json!({
        "hits": [{
            "objectID": "hooks",
            "type": "lvl2",
            "url": "https://react.dev/reference/react/hooks#state-hooks",
            "hierarchy": {
                "lvl0": "React APIs",
                "lvl1": "Built-in React Hooks",
                "lvl2": "State Hooks",
                "lvl3": null,
                "lvl4": null,
                "lvl5": null,
                "lvl6": null
            },
            "content": null
        }]
    }))?;

    assert_eq!(response.hits[0].hierarchy.level(2), Some("State Hooks"));
    Ok(())
}

#[test]
fn search_response_deserializes_mixed_content_and_hierarchy_results() -> Result<()> {
    let response: SearchResponse = serde_json::from_value(json!({
        "hits": [
            {
                "objectID": "hooks",
                "type": "lvl1",
                "url": "https://react.dev/reference/react/hooks",
                "hierarchy": {
                    "lvl0": "React APIs",
                    "lvl1": "Built-in React Hooks",
                    "lvl2": null,
                    "lvl3": null,
                    "lvl4": null,
                    "lvl5": null,
                    "lvl6": null
                },
                "content": null
            },
            {
                "objectID": "use-content",
                "type": "content",
                "url": "https://react.dev/reference/react/use#conditional-use",
                "hierarchy": {
                    "lvl0": "React APIs",
                    "lvl1": "use",
                    "lvl2": "Usage (Promises)",
                    "lvl3": "Pitfall",
                    "lvl4": null,
                    "lvl5": null,
                    "lvl6": null
                },
                "content": "Unlike other hooks, use can be called inside conditions."
            }
        ]
    }))?;

    assert_eq!(
        (
            response.hits[0].object_id.as_str(),
            response.hits[1].object_id.as_str(),
            response.hits[1].hierarchy_level()?
        ),
        ("hooks", "use-content", 0)
    );
    Ok(())
}
