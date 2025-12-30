use axum::extract::Path;
use axum::response::Html;
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

pub fn file_to_string(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents: String = "".to_string();
    let _ = file.read_to_string(&mut contents);
    return contents;
}

#[tokio::main]
async fn main() {
    let key_contents: String = file_to_string("./assets/key.txt");
    let mut key: HashMap<String, String> = HashMap::new();
    for line in key_contents.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        println!("{}", parts.len());
        if parts.len() == 2 {
            key.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    let app = Router::new().route("/{code}", get(move |code| redirect(code, key)));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3981")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn redirect(Path(code): Path<String>, key: HashMap<String, String>) -> Html<String> {
    let resp = key.get(&code);
    if resp.is_none() {
        return Html(
            "<meta http-equiv=\"refresh\" content=\"0; url=https://quinnpatwardhan.com \" />"
                .to_string(),
        );
    } else {
        return Html(format!(
            "<meta http-equiv=\"refresh\" content=\"0; url=https://{} \" />",
            resp.unwrap()
        ));
    }
}
