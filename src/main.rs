use data_collection::recipes::*;
use data_collection::db;

#[tokio::main]
async fn main() {
    
    
    let db = db::conn().await.expect("Failed to connect to DB: ");
    
    let document = match get_document("https://www.aheadofthyme.com/40-best-salad-recipes/").await {
        Ok(doc) => doc,
        Err(e) => panic!("Cannot get doc!")
    };


    // Assuming `get_recipe_urls` is a function that takes a `Document` and returns URLs
    let recipes = get_recipes(&document).await;   

    for rec in recipes {

        let mut id = rec.url.clone();
        id = id.trim_start_matches("https://www.aheadofthyme.com/").trim_end_matches("/").to_string();
        
        println!("WROTE: {}", id);

        let _: Option<Recipe> = match db
            .create(("recipes", id))
            .content(rec)
            .await {
                Ok(Res) => { println!("Sucess"); Res}
                Err(e) => {println!("Failure: {}", e); None}
            };

    }


}

