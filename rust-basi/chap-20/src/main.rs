fn main() {
    let db_conn = DbConnection {
        host: "localhost".to_string(),
        port: 5432,
    };

    println!("Connected to database at {}:{}", db_conn.host, db_conn.port);
}

struct DbConnection {
    host: String,
    port: u16,
}

// Implementing Drop trait to handle cleanup when DbConnection goes out of scope
impl Drop for DbConnection {
    fn drop(&mut self) {
        println!("Closing database connection to {}:{}", self.host, self.port);
        // Here you would add code to actually close the connection
    }
}
