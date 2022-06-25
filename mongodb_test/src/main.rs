/**
 * link: https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
 */
use bson::{doc, oid::ObjectId, Bson, Document};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
// use std::env;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = "mongodb://localhost:27017";
    //   env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    let users: Collection<Document> = client.database("test").collection("users");

    // create a document
    let new_doc = doc! {
     "name": "Bruce",
     "age": 50
    };
    println!("{}", new_doc);

    // insert a document
    let insert_result = users.insert_one(new_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    // find a document
    let user = users
        .find_one(
            doc! {
                "name": "Bruce"
            },
            None,
        )
        .await?
        .expect("Not Found");

    println!("{}", user);

    // update a document
    let update_result = users
        .update_one(
            doc! {
               "_id": &user.get("_id")
            },
            doc! {
               "$set": { "age": 18 }
            },
            None,
        )
        .await?;
    println!("Updated {} document", update_result.modified_count);

    // delete a document
    let delete_result = users
        .delete_one(
            doc! {
               "name": "Bruce"
            },
            None,
        )
        .await?;
    println!("Deleted {} documents", delete_result.deleted_count);

    // access document field without serde
    if let Ok(name) = user.get_str("name") {
        println!("{}", name);
    } else {
        println!("no name field!");
    }

    // map bson to struct with serde
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        name: String,
        age: i32,
    }

    let new_user = User {
        id: None,
        name: "Adam".to_string(),
        age: 18,
    };

    // Convert new_user to a Bson instance:
    let serialized_user = bson::to_bson(&new_user)?;
    let document = serialized_user.as_document().unwrap();

    // insert a document
    let insert_result = users.insert_one(document, None).await?;
    println!("New document ID: {}", insert_result.inserted_id);

    // Read the document from the users collection:
    let loaded_user = users
        .find_one(
            Some(doc! { "_id":  insert_result.inserted_id.clone() }),
            None,
        )
        .await?
        .expect("Document not found");

    // Deserialize the document into a User instance
    let loaded_user_struct: User = bson::from_bson(Bson::Document(loaded_user))?;
    println!("User loaded from collection: {:?}", loaded_user_struct);

    Ok(())
}
