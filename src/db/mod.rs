use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;
use surrealdb::{
    engine::remote::ws::{Ws, Client},
    Connection, Surreal,
};
use tokio;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Example {
    pub data1: u32,
    pub data2: u32,
    pub data3: u32,
}

impl Example {
    pub fn src(&self) -> String {
        format!("d1: {}, d2: {}, d3: {}", self.data1, self.data2, self.data3)
    }
}

pub async fn conn() -> Result<Surreal<Client>, surrealdb::Error> {
    let db = Surreal::new::<Ws>("127.0.0.1:8080").await?;
    db.use_ns("test").use_db("test").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn write_example() {
        let db = conn().await.expect("Failed to connect to db:");

        let ex = Example {
            data1: 1,
            data2: 2,
            data3: 3,
        };

        let ex: Option<Example> = db
            .create(("ex", "example_id"))
            .content(ex)
            .await
            .expect("Failed to create and insert example in the db");

        assert_eq!(ex, Some(Example{data1: 1, data2:2 ,data3: 3}));

    }

    #[tokio::test]
    async fn read_example() {

        let db = conn().await.expect("Failed to connect to db:");

        let ex: Vec<Example> = db.select("ex").await.expect("Failed to retrieve ex");
        
        assert_eq!(ex, vec![Example{data1: 1, data2:2 ,data3: 3}]);


    }
}
