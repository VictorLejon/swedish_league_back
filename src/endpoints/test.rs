use warp::{Filter, Reply, http::StatusCode};

pub async fn test_handler(
    data: String
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        format!("Request data: {}", data),
        StatusCode::OK,
    )) 
}

pub fn test_route(

) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("test" / String)
        .and(warp::get())
        .and_then(test_handler)
}