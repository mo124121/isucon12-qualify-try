use sqlx::mysql::{MySqlDatabaseError, MySqlPool};

pub async fn create_index_if_not_exists(pool: &MySqlPool, query: &str) -> Result<(), sqlx::Error> {
    match sqlx::query(query).execute(pool).await {
        Ok(_) => Ok(()),
        Err(err) => {
            // MySQLのエラーコード1061または1060を確認
            if let Some(db_err) = err.as_database_error() {
                let mysql_err_code = db_err.downcast_ref::<MySqlDatabaseError>().number();
                if mysql_err_code == 1061 || mysql_err_code == 1060 {
                    println!("Detected already existing index/column, but it's ok");
                    return Ok(());
                }
            }
            // 他のエラーはそのまま返す
            return Err(err);
        }
    }
}
pub async fn drop_unique_index_if_exists(
    pool: &MySqlPool,
    table: &String,
    index: &String,
) -> sqlx::Result<()> {
    // インデックスの存在を確認するクエリ
    let index_exists: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM information_schema.statistics 
        WHERE table_name = ? 
          AND table_schema = DATABASE() 
          AND index_name = ?;
        "#,
    )
    .bind(table)
    .bind(index)
    .fetch_one(pool)
    .await?;

    // インデックスが存在する場合、削除する
    if index_exists.0 > 0 {
        let query_str = format!("ALTER TABLE {} DROP INDEX {};", table, index);
        // 動的に構築されたクエリを実行
        sqlx::query(&query_str).execute(pool).await?;
        println!("Index {} was dropped successfully.", index);
    } else {
        println!("Index {} does not exist.", index);
    }

    Ok(())
}
