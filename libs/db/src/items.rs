use crate::{DateTime, DbError, DbTran};

pub async fn get_or_insert(
    tran: &DbTran<'_>,
    item_id: i64,
    posted_by: i64,
    posted_at: DateTime,
) -> Result<i64, DbError> {
    let id = tran
        .query_one(
            r#"
            WITH "inserted" AS (
                INSERT INTO "items" ("item_id", "posted_by", "posted_at") VALUES ($1, $2, $3)
                ON CONFLICT ("item_id") DO NOTHING
                RETURNING "id"
            )
            SELECT "id" FROM "items" WHERE "item_id" = $1
            UNION
            SELECT "id" FROM "inserted"
            "#,
            &[&item_id, &posted_by, &posted_at],
        )
        .await?
        .try_get("id")
        .map_err(DbError::generic)?;
    Ok(id)
}
