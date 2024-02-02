use axum::{
    routing::get,
    response::Html,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root));
        // .route("/styles", get(styles));

    // run our app with hyper, listening globally on port 3000 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let index_html = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(index_html)
}

/*
async fn styles () {

}
*/
