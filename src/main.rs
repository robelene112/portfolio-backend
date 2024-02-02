use axum::{
    routing::get,
    response::{Html, IntoResponse},
    Router,
    http::header
};

#[tokio::main]
async fn main() {
    // build our application 
    let app = Router::new()
        .route("/", get(root))
        .route("/reset", get(reset_styles))
        .route("/styles", get(desktop_styles))
        .route("/mobile", get(mobile_styles))
        .route("/main", get(main_js));

    // run our app with hyper, listening globally on port 3000 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> { 
    Html(include_str!("../static/index.html"))
}

async fn desktop_styles() -> impl IntoResponse  {
    ([(header::CONTENT_TYPE, "text/css")], include_str!("../static/styles.css"))
}

async fn mobile_styles() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], include_str!("../static/mobile.css"))
}

async fn main_js() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], include_str!("../static/main.js"))
}

async fn reset_styles() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], include_str!("../static/reset.css"))
}
