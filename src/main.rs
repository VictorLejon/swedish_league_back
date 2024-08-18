use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};

use warp::Filter;

mod routes;

fn combined_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp.Rejection> + Clone{
    routes::test::test_route()
        //.or(endpoints::another::another_route())
        //.or(endpoints::another::another_route())
        //.or(endpoints::another::another_route())
}

#[tokio::main]
async fn main() /*-> mongodb::error::Result<()> */ {
    let mongodb_uri =  dotenv::var("MONGODB_URI").expect("MONGODB_URI is not set");

    // Create a new client and connect to the server
    //let client = Client::with_uri_str(mongodb_uri).await?;

    // Get a handle on the movies collection
    //let database = client.database("sample_mflix");
    //let my_coll: Collection<Document> = database.collection("movies");

    // Find a movie based on the title value
    //let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;

    // Print the document
    //println!("Found a movie:\n{:#?}", my_movie);

    // DEFINE ENDPOINTS
    let routes = combined_routes.with(warp::log("endpoints"));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
