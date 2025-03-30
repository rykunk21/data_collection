use data_collection::db;
use data_collection::recipes::*;

#[tokio::main]
async fn main() {
    let db = db::conn().await.expect("Failed to connect to DB: ");

    let document = match get_document("https://www.aheadofthyme.com/40-best-salad-recipes/").await {
        Ok(doc) => doc,
        Err(_) => panic!("Cannot get doc!"),
    };

    let recipes = get_recipes(&document).await;

    for rec in recipes {
        let mut id = rec.url.clone();
        id = id
            .trim_start_matches("https://www.aheadofthyme.com/")
            .trim_end_matches("/")
            .to_string();

        println!("WROTE: {}", id);

        let _: Option<Recipe> = match db.create(("recipes", id)).content(rec).await {
            Ok(res) => {
                println!("Sucess");
                res
            }
            Err(e) => {
                println!("Failure: {}", e);
                None
            }
        };
    }
}
