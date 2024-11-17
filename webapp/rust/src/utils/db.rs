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
