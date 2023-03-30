pub mod handler;
pub mod resolver;
pub mod schema;

#[cfg(test)]
mod test_utils {
    use crate::http::handler::graphql_handler;
    use crate::http::schema::build_schema;
    use actix_web::test;
    use actix_web::web::Data;
    use actix_web::{guard, web, App};
    use serde_json::json;

    // 検証以上のことをしているので、メソッド名を適切にするか、適切な粒度で分割したい.
    pub async fn assert_query(query: &str, expected: serde_json::Value) {
        let schema = build_schema();
        let app = test::init_service(
            App::new()
                .app_data(Data::new(schema.clone()))
                .service(web::resource("/").guard(guard::Post()).to(graphql_handler)),
        )
        .await;

        let resp = test::TestRequest::post()
            .uri("/")
            .set_json(async_graphql::Request::new(query))
            .send_request(&app)
            .await;

        let resp_bytes = test::read_body(resp).await;
        let resp_str = std::str::from_utf8(&resp_bytes).unwrap();

        let resp: serde_json::Value = serde_json::from_str(resp_str).unwrap();

        assert_eq!(resp["data"], json!(expected));
    }
}
