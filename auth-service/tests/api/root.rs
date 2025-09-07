use crate::helpers::TestApp;
use test_context::test_context;

#[test_context(TestApp)]
async fn root_returns_auth_ui(app: &mut TestApp) {
    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
