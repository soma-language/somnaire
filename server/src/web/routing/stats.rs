use axum::{debug_handler, extract::State};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, IntoSimpleExpr, QuerySelect,
};
use sea_query::{Expr, Func};

use crate::{
    schema::packages,
    web::api::{ApiResponse, ok},
};

#[derive(serde::Serialize, FromQueryResult)]
pub struct PackagesStats {
    packages: i64,
    versions: i64,
    downloads: i64,
    publishers: i64,
}

#[debug_handler]
pub async fn handler(State(db): State<DatabaseConnection>) -> ApiResponse<PackagesStats> {
    let stats = packages::Entity::find()
        .select_only()
        .column_as(packages::Column::Id.count(), "packages")
        .column_as(packages::Column::Version.count(), "versions") // todo
        .column_as(
            Func::coalesce([
                Func::sum(packages::Column::Downloads.into_expr()).into(),
                Expr::value(0),
            ])
            .into_simple_expr(),
            "downloads",
        )
        .column_as(
            Func::count_distinct(packages::Column::Author.into_expr()).into_simple_expr(),
            "publishers",
        )
        .into_model::<PackagesStats>()
        .one(&db)
        .await
        .inspect_err(|e| tracing::error!("Failed to fetch package stats: {}", e))?
        .unwrap_or(PackagesStats {
            packages: 0,
            versions: 0,
            downloads: 0,
            publishers: 0,
        });

    ok(stats)
}
