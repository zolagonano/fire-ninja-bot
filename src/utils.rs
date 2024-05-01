pub async fn fetch_sources(sources: &[&str]) -> String {
    let mut content = String::new();
    for source in sources {
        if let Ok(response) = reqwest::get(*source).await {
            if let Ok(text) = response.text().await {
                content.push_str(&text);
            }
        }
    }

    content
}
