mod test_env;

use {crate::test_env::*, serde_json::json};

#[tokio::test]
async fn test_gas_usage_get_all_children_ids() -> anyhow::Result<()> {
    // Test Flow:
    // 1. Add posts
    // 2. Get all children ids of a post and check gas usage.
    let contract = init_contracts().await?;

    let deposit_amount = near_units::parse_near!("0.1");

    // Add Posts
    let add_idea_post = contract
        .call("add_post")
        .args_json(json!({
            "parent_id": null,
            "labels": [],
            "body": {
                "name": "This is a test idea.",
                "description": "This is a test description.",
                "post_type": "Idea",
                "idea_version": "V1"
            }
        }))
        .deposit(deposit_amount)
        .transact()
        .await?;

    assert!(add_idea_post.is_success());

    let add_solution_v2_post = contract
        .call("add_post")
        .args_json(json!({
            "parent_id": 0,
            "labels": [],
            "body": {
                "name": "Solution Test",
                "description": "This is a test solution post.",
                "post_type": "Solution",
                "requested_sponsor": "neardevgov.near",
                "requested_sponsorship_amount": "1000",
                "requested_sponsorship_token": "NEAR",
                "solution_version": "V2"
            }
        }))
        .deposit(deposit_amount)
        .max_gas()
        .transact()
        .await?;

    assert!(add_solution_v2_post.is_success());

    let add_comment_post = contract
        .call("add_post")
        .args_json(json!({
            "parent_id": 0,
            "labels": [],
            "body": {
                "description": "This is test Comment.",
                "comment_version": "V2",
                "post_type": "Comment"
            }
        }))
        .deposit(deposit_amount)
        .max_gas()
        .transact()
        .await?;

    assert!(add_comment_post.is_success());

    // Add first level comments to post 0
    for _ in 0..10 {
        // Add more posts with parent_id 0
        let add_comment_post = contract
            .call("add_post")
            .args_json(json!({
                "parent_id": 0,
                "labels": [],
                "body": {
                    "description": "This is test Comment.",
                    "comment_version": "V2",
                    "post_type": "Comment"
                }
            }))
            .deposit(deposit_amount)
            .max_gas()
            .transact()
            .await?;

        assert!(add_comment_post.is_success());
    }

    // Add comments to the comments
    for i in 0..10 {
        // Add more posts with parent_id 0
        let add_comment_post = contract
            .call("add_post")
            .args_json(json!({
                "parent_id": i + 2,
                "labels": [],
                "body": {
                    "description": "This is test Comment.",
                    "comment_version": "V2",
                    "post_type": "Comment"
                }
            }))
            .deposit(deposit_amount)
            .max_gas()
            .transact()
            .await?;
        assert!(add_comment_post.is_success());
    }
    // Add a comment manually because it seems the for loop doesn't add any gas
    let add_comment_post = contract
        .call("add_post")
        .args_json(json!({
            "parent_id": 0,
            "labels": [],
            "body": {
                "description": "This is test Comment.",
                "comment_version": "V2",
                "post_type": "Comment"
            }
        }))
        .deposit(deposit_amount)
        .max_gas()
        .transact()
        .await?;

    assert!(add_comment_post.is_success());

    let res = contract
        .call("get_all_children_ids")
        .args_json(json!({
          "post_id": 0,
        }))
        .deposit(deposit_amount)
        .max_gas()
        .transact()
        .await?;

    println!("Burnt gas (all): {}", res.total_gas_burnt);

    assert_eq!(format!("Burnt gas (all):"), format!("Burnt gas (all): {}", res.total_gas_burnt));

    Ok(())
}
