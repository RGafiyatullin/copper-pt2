use crate::{DbError, DbTran};

pub async fn get_or_insert(tran: &DbTran<'_>, nickname: &str) -> Result<i64, DbError> {
    let user_id = tran
        .query_one(
            r#"
            WITH "inserted" AS (
                INSERT INTO "users" ("nickname") VALUES ($1)
                ON CONFLICT ("nickname") DO NOTHING
                RETURNING "id"
            )
            SELECT "id" FROM "users" WHERE "nickname" = $1
            UNION
            SELECT "id" FROM "inserted"
            "#,
            &[&nickname],
        )
        .await?
        .try_get("id")
        .map_err(DbError::generic)?;
    Ok(user_id)
}
