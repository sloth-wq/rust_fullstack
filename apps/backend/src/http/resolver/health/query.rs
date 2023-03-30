use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
struct Ping {
    status: i64,
    message: String,
}

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    async fn ping(&self) -> Ping {
        Ping {
            status: 200,
            message: "OK".to_string(),
        }
    }
}

// TODO: 統合テストは tests に移動したい.
#[cfg(test)]
mod tests {
    use crate::http::test_utils::assert_query;
    use serde_json::json;

    #[actix_web::test]
    async fn ok() {
        let query = r#"{ ping { status, message } }"#;

        let expected = json!({
            "ping": {
                "status": 200,
                "message": "OK"
            }
        });

        assert_query(query, expected).await;
    }
}
