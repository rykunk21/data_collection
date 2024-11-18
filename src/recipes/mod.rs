use regex::Regex;
use reqwest::Client;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::panic;

use crate::utils::u32Ext;


pub async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url)
        .send()
        .await?;

    let body = res.text().await?;
    Ok(body)
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Nutrient {
    unit: String,
    label: String,
    quantity: f64,
    daily: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[allow(non_snake_case)]
struct Macros {
    PROCNT: Nutrient,
    FAT: Nutrient,
    CHOCDF: Nutrient,
    ENERC_KCAL: Nutrient,
    SUGAR: Nutrient,
    FIBTG: Nutrient,
    CA: Nutrient,
    FE: Nutrient,
    MG: Nutrient,
    P: Nutrient,
    K: Nutrient,
    NA: Nutrient,
    ZN: Nutrient,
    VITA_RAE: Nutrient,
    TOCPHA: Nutrient,
    VITD: Nutrient,
    VITC: Nutrient,
    THIA: Nutrient,
    RIBF: Nutrient,
    NIA: Nutrient,
    VITB6A: Nutrient,
    FOL: Nutrient,
    VITB12: Nutrient,
    VITK1: Nutrient,
    CHOLE: Nutrient,
    FATRN: Nutrient,
    FASAT: Nutrient,
    FAMS: Nutrient,
    FAPU: Nutrient,
}

impl Macros {
    pub fn normalize_by_servings(&mut self, servings: u64) -> () {
        let nutrients = vec![
            &mut self.PROCNT,
            &mut self.FAT,
            &mut self.CHOCDF,
            &mut self.ENERC_KCAL,
            &mut self.SUGAR,
            &mut self.FIBTG,
            &mut self.CA,
            &mut self.FE,
            &mut self.MG,
            &mut self.P,
            &mut self.K,
            &mut self.NA,
            &mut self.ZN,
            &mut self.VITA_RAE,
            &mut self.TOCPHA,
            &mut self.VITD,
            &mut self.VITC,
            &mut self.THIA,
            &mut self.RIBF,
            &mut self.NIA,
            &mut self.VITB6A,
            &mut self.FOL,
            &mut self.VITB12,
            &mut self.VITK1,
            &mut self.CHOLE,
            &mut self.FATRN,
            &mut self.FASAT,
            &mut self.FAMS,
            &mut self.FAPU,
        ];

        for nutrient in nutrients {
            // Normalize the quantity and daily value by dividing by the number of servings
            nutrient.quantity /= servings as f64;
            nutrient.daily /= servings as f64;
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Unit {
    TABLESPOON,
    TEASPOON,
    CUP,
    LB,
    CONTAINER,
}

impl Unit {
    pub fn from(str: &str) -> Result<Self, Box<dyn Error>> {
        match str.to_lowercase().as_str() {
            "tablespoon" | "tablespoons" => Ok(Unit::TABLESPOON),
            "teaspoon" | "teaspoons" => Ok(Unit::TEASPOON),
            "cup" | "cups" => Ok(Unit::CUP),
            "lb" | "lbs" | "pound" | "pounds" => Ok(Unit::LB),
            "container" | "containers" => Ok(Unit::CONTAINER),
            _ => Err("Error building Unit enum!")?,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instruction {
    pub section: Option<String>,
    pub steps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ingredient {
    name: String,
    quantity: f32,
    units: Option<Unit>,
    prepped: Option<String>,
}

/// Represents a recipe with detailed information including image, URL, cuisine type,
/// preparation method, time estimates, ingredients, and more.
///
/// The `Recipe` struct holds a complete set of information typically associated with a recipe,
/// including the ingredients, instructions, and video links (if available). It also supports optional
/// fields like the description, notes, and macros (nutritional information), making it flexible for a
/// variety of recipe formats.
///
/// # Fields
///
/// - `img`: A string containing the URL of the recipe's image.
/// - `url`: A string containing the URL to the recipe's page.
/// - `cuisine`: A string indicating the cuisine type of the recipe (e.g., Italian, Mexican).
/// - `category`: A string indicating the category of the recipe (e.g., dessert, main course).
/// - `method`: A string describing the method of preparation (e.g., baking, frying).
/// - `total_time`: Total time in minutes for making the recipe.
/// - `prep_time`: Preparation time in minutes (time spent on getting ingredients ready).
/// - `cook_time`: Cooking time in minutes.
/// - `name`: The name of the recipe.
/// - `description`: An optional string providing a description of the recipe.
/// - `instructions`: A vector of `Instruction` objects detailing the step-by-step process to make the recipe.
/// - `ingredients`: A vector of `Ingredient` objects containing all the ingredients required for the recipe.
/// - `video`: An optional string containing the URL to a video tutorial for the recipe.
/// - `notes`: An optional string containing additional notes for the recipe (e.g., tips or variations).
/// - `servings`: The number of servings the recipe yields.
/// - `equiptment`: A vector of strings listing the equipment needed for the recipe.
/// - `macros`: An optional `Macros` object containing nutritional information (e.g., calories, protein).
///
/// # Example
///
/// ```rust
/// let recipe = Recipe::new("https://example.com/image.jpg", "https://example.com/recipe-page")
///     .expect("Failed to create recipe");
/// ```
///
/// # Errors
///
/// The `new` function may return an error if there are issues parsing the recipe data.
/// This could occur if any of the fields cannot be properly extracted or parsed during
/// initialization.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Recipe {
    img: String,
    pub url: String,
    cuisine: String,
    category: String,
    method: String,
    total_time: u32,
    prep_time: u32,
    cook_time: u32,
    pub name: String,
    description: Option<String>,
    pub instructions: Vec<Instruction>,
    ingredients: Vec<Ingredient>,
    video: Option<String>,
    notes: Option<String>,
    servings: u64,
    equiptment: Vec<String>,
    macros: Option<Macros>,
}

impl Recipe {
    /// Loads a new recipe from the database
    pub fn from_id(id: u64) -> Self {

        Recipe {
            url: "someurl".to_string(),
            ..Default::default()
        }
    }

    /// Creates a new `Recipe` instance from an image URL and a recipe URL.
    ///
    /// This method initializes the `Recipe` struct with the provided image and URL.
    /// It also attempts to parse additional recipe data using the `parse_recipe` method.
    ///
    /// # Arguments
    ///
    /// - `img`: A string slice containing the URL to the image of the recipe.
    /// - `url`: A string slice containing the URL to the recipe's page.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Box<dyn Error>>`: Returns a `Recipe` if successfully created, or an error if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let recipe = Recipe::new("https://example.com/image.jpg", "https://example.com/recipe-page")
    ///     .expect("Failed to create recipe");
    /// ```
    ///
    /// # Errors
    ///
    /// If parsing the recipe fails (e.g., missing data, invalid format), this function returns an error.
    pub async fn new(img: &str, url: &str) -> Result<Self, Box<dyn Error>> {
        let mut r = Recipe {
            img: img.into(),
            url: url.into(),
            ..Default::default()
        };
        match r.parse_recipe().await {
            Ok(()) => return Ok(r),
            Err(e) => return Err(e),
        }
    }

    /// Parses the recipe out of the recipe's home page
    ///
    /// Constructing a recipe instance queries the url and extracts the
    /// relevant data into the recipe strcut. Parsing this information is a
    /// lot of work, which this function handles
    async fn parse_recipe(&mut self) -> Result<(), Box<dyn Error>> {
        // The document represents the page as whole, starts enabling `find` capabilities
        let document =
            get_document(&self.url).await?;

        let mut id = document
            .find(Class("tasty-recipes-jump-link"))
            .next()
            .and_then(|id| id.attr("href").map(|href| href.to_string()))
            .ok_or("ID not found in document")?; // Converts the Option to Result and propagates error using ?

        // Process the id (assuming you want to remove the '#' and '-jump-target' from the href)
        id = id
            .trim_start_matches('#')
            .trim_end_matches("-jump-target")
            .to_string();

        if let Some(recipe) = document.find(Attr("id", id.as_str())).next() {
            let header = recipe.find(Name("header")).next().unwrap();

            self.parse_header(&header)?;

            let body = recipe
                .find(Class("tasty-recipes-entry-content"))
                .next()
                .unwrap();

            self.description = body
                .find(Class("tasty-recipes-description-body"))
                .next()
                .and_then(|d| Some(d.text().trim().into()));

            if let Some(ul_containter) = body.find(Class("tasty-recipes-ingredients")).next() {
                for ul in ul_containter.find(Name("ul")) {
                    self.parse_ingredients(&ul)?;
                }
            }

            if let Some(instructions_block) = body.find(Class("tasty-recipes-instructions")).next()
            {
                self.parse_instructions(&instructions_block)?;
            }

            if let Some(frame_url) = body
                .find(Name("iframe"))
                .next()
                .and_then(|fr| fr.attr("src").map(String::from))
            {
                self.video = Some(frame_url);
            }

            self.notes = match body
                .find(Class("tasty-recipes-notes"))
                .next()
                .and_then(|n| Some(n.text().trim().to_string()))
            {
                Some(n) => Some(Self::clean_notes(&n)),
                None => None,
            };

            if let Some(details) = body
                .find(Class("tasty-recipes-other-details"))
                .next()
                .and_then(|d| d.find(Name("ul")).next())
            {
                for li in details.find(Name("li")) {
                    match li.attr("class") {
                        Some(class) => match class {
                            "prep-time" => {
                                let prep_time_str = li
                                    .find(Class("tasty-recipes-prep-time"))
                                    .next()
                                    .unwrap()
                                    .text();
                                self.prep_time = match u32::from_time_str(&prep_time_str) {
                                    Ok(t) => t,
                                    Err(e) => panic!("{}: {}", self.url, e),
                                };
                            }
                            "cook-time" => {
                                let cook_time_str = li
                                    .find(Class("tasty-recipes-cook-time"))
                                    .next()
                                    .unwrap()
                                    .text();
                                self.cook_time = match u32::from_time_str(&cook_time_str) {
                                    Ok(t) => t,
                                    Err(e) => panic!("{}: {}", self.url, e),
                                };
                            }
                            "cuisine" => {
                                self.cuisine = li
                                    .find(Class("tasty-recipes-cuisine"))
                                    .next()
                                    .unwrap()
                                    .text();
                            }
                            "category" => {
                                self.category = li
                                    .find(Class("tasty-recipes-category"))
                                    .next()
                                    .unwrap()
                                    .text();
                            }
                            "method" => {
                                self.method = li
                                    .find(Class("tasty-recipes-method"))
                                    .next()
                                    .unwrap()
                                    .text();
                            }

                            _ => {}
                        },
                        None => {}
                    }
                }
            }

            if let Some(nutrition_url) = body
                .find(Name("iframe").and(Attr("title", "nutritional information")))
                .next()
                .and_then(|nut| nut.attr("data-l-src"))
            {
                self.get_macros(format!("https:{}", nutrition_url).as_str()).await?;
            } else {
                self.macros = None
            }

            Ok(())
        } else {
            Err("ID not found in document (after jump link was found)")?
        }
    }

    /// Parses the recipe's name and total time from the header node.
    ///
    /// Extracts the name from an `<h2>` tag and total time from a specific class,
    /// updating the `Recipe` struct's fields. Panics if the total time is invalid.
    ///
    /// # Arguments
    ///
    /// - `header`: The HTML node containing the recipe's header information.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if parsing is successful, or an error if parsing fails.    
    fn parse_header(&mut self, header: &Node) -> Result<(), Box<dyn Error>> {
        self.name = header.find(Name("h2")).next().unwrap().text();
        let time_str = header
            .find(Class("tasty-recipes-total-time"))
            .next()
            .unwrap()
            .text();

        self.total_time = match u32::from_time_str(&time_str) {
            Ok(t) => t,
            Err(e) => panic!("{}: {}", self.url, e),
        };

        Ok(())
    }

    /// Parses the ingredient list from a given HTML node, extracting ingredient names, quantities, units, and preparation details.
    /// The ingredients are then added to the recipe's list of ingredients.
    ///
    /// # Arguments
    /// - `list`: The HTML node containing the ingredient list (`<ul>` or `<li>` elements).
    ///
    /// # Returns
    /// A `Result` that indicates whether the parsing succeeded (`Ok(())`) or failed (`Err`).    
    fn parse_ingredients(&mut self, list: &Node) -> Result<(), Box<dyn Error>> {
        let mut ingredients = Vec::new();

        for ingredient in list.find(Name("li")) {
            let name = match ingredient.find(Name("strong")).next() {
                Some(n) => n.text(),
                None => match ingredient.find(Name("b")).next() {
                    Some(n) => n.text(),
                    None => Err(format!(
                        "Error building ingredients for: {}. No ingredient name found:{} ",
                        self.url,
                        ingredient.text()
                    ))?,
                },
            };

            if let Some(span) = ingredient.find(Name("span")).nth(1) {
                let quantity = match span.attr("data-amount") {
                    Some(q) => q.parse::<f32>().unwrap(),
                    None => span
                        .find(Name("span"))
                        .next()
                        .ok_or("Could not parse inner span")? 
                        .attr("data-amount")
                        .ok_or("Could not parse inner span")?
                        .parse::<f32>()?
                     
                };

                let units = span.attr("data-unit").and_then(|u| match Unit::from(u) {
                    Ok(u) => Some(u),
                    Err(_) => None, // just return none if the parsing fails
                });

                let prepped = ingredient
                    .find(Name("em"))
                    .next()
                    .and_then(|p| Some(p.text()));

                ingredients.push(Ingredient {
                    name,
                    quantity,
                    units,
                    prepped,
                });
            } else {
                // no units (things like parsley, optional for seriving: https://www.aheadofthyme.com/minestrone-soup/)
                let prepped = ingredient
                    .find(Name("em"))
                    .next()
                    .and_then(|p| Some(p.text()));

                ingredients.push(Ingredient {
                    name,
                    quantity: 0.0,
                    units: None,
                    prepped,
                });
            }
        }

        self.add_ingredients(ingredients);

        Ok(())
    }

    /// Adds ingredients to the recipe, either updating the quantity of existing ingredients or adding new ones.
    ///
    /// # Arguments
    /// - `new_ingredients`: A vector of ingredients to be added or updated in the recipe.
    ///
    /// # Returns
    /// This function does not return any value.
    fn add_ingredients(&mut self, new_ingredients: Vec<Ingredient>) {
        for new_ingredient in new_ingredients {
            // Check if the ingredient already exists in the vector
            if let Some(existing_ingredient) = self
                .ingredients
                .iter_mut()
                .find(|ingredient| ingredient.name == new_ingredient.name)
            {
                // If it exists, increase the quantity
                existing_ingredient.quantity += new_ingredient.quantity;
            } else {
                // If it doesn't exist, add it to the vector
                self.ingredients.push(new_ingredient);
            }
        }
    }

    /// Parses the instructions from the provided HTML node and stores them in the recipe.
    /// It handles both single and multiple ordered lists for steps and organizes them under section headers.
    ///
    /// # Arguments
    /// - `list`: The HTML node containing the instructions to be parsed.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn Error>>`: Returns `Ok(())` on success, or an error if parsing fails.
    fn parse_instructions(&mut self, list: &Node) -> Result<(), Box<dyn Error>> {
        // "https://www.aheadofthyme.com/easy-meat-lasagna/" for some reason not grabbing all instructions, but other similar examples are
        let h4_blocks: Vec<_> = list.find(Name("h4")).collect();

        let mut ol_blocks: Vec<_> = list
            .find(Name("div"))
            .nth(1)
            .expect("Could not find nth child in parse instructions")
            .children()
            .filter(|child| child.name() == Some("ol"))
            .collect();

        if ol_blocks.len() == 1 {
            // we have 1 ol block
            let ol_block = ol_blocks
                .pop()
                .expect("Ol blocks failed to pop the element");

            self.instructions = vec![Instruction {
                section: None,
                steps: ol_block.find(Name("li")).map(|step| step.text()).collect(),
            }];
        } else {
            let mut instructions = Vec::new();

            for (h4, ol) in h4_blocks.iter().zip(ol_blocks.iter()) {
                let instruction = Instruction {
                    section: Some(String::from(h4.text().trim_end_matches(":"))),
                    steps: ol.find(Name("li")).map(|step| step.text()).collect(),
                };
                instructions.push(instruction);
            }

            self.instructions = instructions;
        }

        Ok(())
    }

    /// Fetches and parses nutritional information from a given URL.
    /// Extracts the recipe's macros and servings from a JavaScript variable and normalizes the macros by servings.
    ///
    /// # Arguments
    /// - `url`: The URL where the nutritional data can be found.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn Error>>`: Returns `Ok(())` on success, or an error if parsing fails.
    async fn get_macros(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        let document = get_document(url).await?;

        if let Some(data) = document.find(Name("script")).next() {
            let re = Regex::new(r"var preloaded = \{'recipe': (.*)\}")?;

            if let Some(cap) = re.captures(&data.text()) {
                let json_str = &cap[1];
                let json_value: Value = serde_json::from_str(json_str)?;

                if let (Some(macros), Some(servings)) =
                    (json_value.get("nutrients"), json_value.get("servings"))
                {
                    self.servings = servings.as_u64().expect("Failed to parse servings to u64");
                    self.macros = serde_json::from_value(macros.clone())?;
                    if let Some(macros) = self.macros.as_mut() {
                        macros.normalize_by_servings(self.servings);
                    } else {
                        self.macros = None
                    }
                } else {
                    self.macros = None;
                }
            } else {
                Err(format!("Regex pattern failed from: {}", data.text()))?
            }
        } else {
            Err(format!("Could not find script tag from: {}", url))?
        }

        Ok(())
    }

    /// Cleans up the input string by normalizing spaces, newlines, and tabs.
    /// Replaces non-breaking spaces with regular spaces, consolidates multiple newlines or tabs,
    /// and trims leading/trailing spaces.
    ///
    /// # Arguments
    /// - `input`: The string to be cleaned.
    ///
    /// # Returns
    /// - A cleaned-up version of the input string.    
    fn clean_notes(input: &str) -> String {
        // Remove all non-breaking spaces (\u{a0}) and replace with regular spaces
        let input = input.replace("\u{a0}", " ");

        // Normalize newlines and tabs: replace multiple newlines or tabs with a single newline or space
        let re = Regex::new(r"[\n\t]+").unwrap();
        let input = re.replace_all(&input, " ").to_string();

        // Trim leading and trailing spaces
        input.trim().to_string()
    }
}

/// Retrieves a list of `Recipe` objects by scraping a given HTML document.
///
/// This function searches the provided `Document` for recipe entries contained within a
/// `div` element with the class `"entry-content"`. Each recipe entry is identified by a
/// `figure` tag, which contains a link (`a`) to the recipe's URL and an image (`img`)
/// representing the recipe's image source. Both the URL and image source must be present
/// for a valid recipe to be added to the result list.
///
/// The function creates a new `Recipe` instance for each valid entry and collects them
/// into a `Vec<Recipe>`. If the `Recipe::new` constructor fails, an error message is
/// printed, but the process continues with the next entry.
///
/// # Arguments
///
/// * `document` - A reference to the `Document` to scrape the recipe information from.
///
/// # Returns
///
/// * A `Vec<Recipe>` containing the parsed recipe objects. If no valid recipes are found,
///   an empty vector is returned.
///
/// # Example
///
/// ```rust
/// let document = scraper::Html::parse_document("<html>...</html>");
/// let recipes = get_recipes(&document);
/// for recipe in recipes {
///     println!("{}", recipe);
/// }
/// ```
///
/// # Errors
///
/// Any errors encountered while creating a `Recipe` instance (e.g., missing image or URL)
/// are logged to the console with the corresponding URL, but they do not interrupt the
/// scraping process.
///
/// # Panics
///
/// This function will not panic under normal circumstances.
pub async fn get_recipes(document: &Document) -> Vec<Recipe> {
    let mut out: Vec<Recipe> = Vec::new();

    if let Some(entry_content) = document
        .find(Name("div").and(Class("entry-content")))
        .next()
    {
        for figure in entry_content.find(Name("figure")) {
            let url = figure
                .find(Name("a"))
                .next()
                .and_then(|a| a.attr("href").map(|href| href.to_string()));

            let img = figure
                .find(Name("img"))
                .next()
                .and_then(|img| img.attr("data-lazy-src").map(|src| src.to_string()));

            // Only push if both `url` and `img` are available
            if let (Some(url), Some(img)) = (url, img) {
                match Recipe::new(&img, &url).await {
                    Ok(r) => {
                        out.push(r);
                    }
                    Err(e) => {
                        println!("Url: {} Threw the following: {}", url, e)
                    }
                }
            }
        }
    }

    out
}

/// Retrieves an HTML document from a specified URL.
///
/// This function performs a blocking HTTP GET request to the provided `url` and attempts
/// to fetch the content of the page. If the request is successful, the function converts
/// the response text (HTML) into a `Document` object that can be further processed for scraping.
///
/// If the request or the text conversion fails, an error message is returned as a `Result::Err`
/// with a descriptive error message. On success, it returns a `Result::Ok` containing the `Document`
/// object.
///
/// # Arguments
///
/// * `url` - The URL from which the HTML document is to be fetched. This is a string slice (`&str`).
///
/// # Returns
///
/// * `Result<Document, Box<dyn Error>>` - Returns a `Document` if the request and text reading are successful,
///   or a boxed error if either operation fails.
///
/// # Example
///
/// ```rust
/// let url = "https://example.com";
/// match get_document(url) {
///     Ok(doc) => println!("Document retrieved successfully"),
///     Err(e) => println!("Error retrieving document: {}", e),
/// }
/// ```
///
/// # Errors
///
/// This function may return an error if:
/// - The HTTP request fails (e.g., network issues, invalid URL).
/// - The response text cannot be read (e.g., the server returns an unexpected response).
/// - If any of these issues occur, an error message will be returned detailing the problem.
///
/// # Panics
///
/// This function will not panic under normal circumstances, as it uses error handling to report issues.
pub async fn get_document(url: &str) -> Result<Document, Box<dyn Error>> {
    let response = fetch_data(url).await?;
    
    // Convert the HTML string into a Document
    Ok(Document::from(response.as_str()))
}


pub async fn get_recipe_test(id: u8) -> Recipe {
    
    let img = "";

    let url = "https://www.aheadofthyme.com/easy-meat-lasagna/";

    Recipe::new(img, url).await.expect("Failed to get recipe")


}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_get_recipes() -> Result<(), Box<dyn std::error::Error>> {
        let document = get_document("https://www.aheadofthyme.com/30-best-shrimp-recipes/").await?;
            

        // Assuming `get_recipe_urls` is a function that takes a `Document` and returns URLs
        let urls = get_recipes(&document).await;

        println!("{:#?}", &urls[..5]);
        // Continue with your logic, parsing `response`, etc.
        Ok(())
    }

    #[tokio::test]
    async fn test_parse_recipe() {
        let img = "";

        let url = "https://www.aheadofthyme.com/easy-meat-lasagna/";

        let r = Recipe::new(img, url).await.expect("Failed to get recipe");

        println!("{:#?}", r);
    }

    #[tokio::test]
    async fn test_get_macros() {
        let img = "";

        let url = "https://nutrifox.com/embed/label/121461";

        let mut r = Recipe {
            ..Default::default()
        };
        r.get_macros(&url).await.expect("Failed to get macros");

        println!("{:#?}", r);
    }
}
