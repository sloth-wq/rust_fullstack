use crate::http::resolver::health::query::HealthQuery;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(HealthQuery);

pub type MainSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish()
}
