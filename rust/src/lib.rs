// use anyhow::Result;
// use serde_json::json;

// #[tokio::test]
// async fn quick_dev() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     hc.do_get("/hello").await?.print().await?;

//     let req_create_user = hc.do_post(
//         "/create-user",
//         json!({
//             "username": "c",
//             "password": "a",
//             "first_name": "a",
//             "last_name": "a",
//             "email": "e@e.com",
//         })
//     );

//     req_create_user.await?.print().await;

//     let req_create_message = hc.do_post(
//         "/create-message",
//         json!({
//             "user_id": "a",
//             "message": "test message",
//             "conversation_id": "test_id"
//         })
//     );

//     req_create_message.await?.print().await;

//     Ok(())
// }