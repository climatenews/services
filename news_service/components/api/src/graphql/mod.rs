use crate::graphql::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::PgPool;
use std::fs;

pub mod errors;
pub mod query;

pub type ClimateActionSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn init_graphql_schema(db_pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool)
        .finish();

    // Export graphql schema
    fs::write("../../schema.graphql", schema.sdl()).expect("Unable to write schema");

    schema
}
