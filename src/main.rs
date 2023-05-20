use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    username: Option<String>
}

struct Database {
    pool: Pool,
}

impl Database {
    fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = Pool::new(url)?;
        Ok(Self { pool })
    }

    fn get_connection(&mut self) -> Result<PooledConn, Box<dyn std::error::Error>> {
        self.pool.get_conn().map_err(Into::into)
    }

    fn get_users(&mut self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;
        let selected_users = conn.query_map(
            "SELECT id, username FROM users",
            |(id, username)| User { id, username },
        )?;
        Ok(selected_users)
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:root@localhost:3306/login_fastapi";

    /* let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    
    let selected_users = conn
        .query_map(
            "SELECT id, username FROM users",
            |(id, username)| {
                User { id, username }
            },
        )?;

    println!("{:?}", selected_users); */

    let mut database = Database::new(url)?;

    let selected_users = database.get_users()?;
    println!("{:?}", selected_users);

    Ok(())
}
