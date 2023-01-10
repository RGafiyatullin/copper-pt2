use crate::error::DbError;
use crate::{DateTime, DbTran};

pub async fn insert_snapshot(tran: &DbTran<'_>, taken_at: DateTime) -> Result<i64, DbError> {
    let snapshot_id: i64 = tran
        .query_one(
            r#"
            INSERT INTO "snapshots" ("taken_at") VALUES ($1) RETURNING "id"
            "#,
            &[&taken_at],
        )
        .await?
        .try_get("id")
        .map_err(DbError::generic)?;

    Ok(snapshot_id)
}

pub async fn insert_top_rank(
    tran: &DbTran<'_>,
    snapshot_id: i64,
    item_id: i64,
    rank: i32,
) -> Result<i64, DbError> {
    let top_rank_id = tran
        .query_one(
            r#"
                INSERT INTO "top_ranks" ("snapshot_id", "rank", "item_id")
                VALUES ($1, $2, $3)
                RETURNING "id"
            "#,
            &[&snapshot_id, &rank, &item_id],
        )
        .await?
        .try_get("id")
        .map_err(DbError::generic)?;

    Ok(top_rank_id)
}
