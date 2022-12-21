
#[cfg(test)]
mod test {
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    async fn abc() -> Result<(), sqlx::Error> {
        // Create a connection pool
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:root@127.0.0.1:3306/test").await?;
    
        // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
        let row: (i64,) = sqlx::query_as("SELECT ?")
            .bind(150_i64)
            .fetch_one(&pool).await?;
    
        assert_eq!(row.0, 150);

        Ok(())
    }
}