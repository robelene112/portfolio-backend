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

async fn root() -> Html<String> { 
    let html = tokio::fs::read_to_string("static/index.html").await.unwrap();
    Html(html)
}

async fn desktop_styles() -> impl IntoResponse  {
    let css = tokio::fs::read_to_string("static/styles.css").await.unwrap();
    ([(header::CONTENT_TYPE, "text/css")], css)
}

async fn mobile_styles() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("static/mobile.css").await.unwrap();
    ([(header::CONTENT_TYPE, "text/css")], css)
}

async fn main_js() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("static/main.js").await.unwrap();
    ([(header::CONTENT_TYPE, "text/javascript")], js)
}

async fn reset_styles() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("static/reset.css").await.unwrap();
    ([(header::CONTENT_TYPE, "text/css")], css)
}
