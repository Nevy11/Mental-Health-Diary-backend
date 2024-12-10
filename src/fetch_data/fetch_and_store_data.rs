pub async fn fetch_and_store() {
    let url = "https://www.googleapis.com/customsearch/v1?key=YOUR_API_KEY&cx=YOUR_CSE_ID&q=Rust";
    let client = reqwest::Client::new();
    let _res = client.get(url).send().await.unwrap();
}
