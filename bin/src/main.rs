


#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Hello, world!");

    let endpoint = "https://github.com";
    let client = rs_crypto_sdk::ApiClient::from(endpoint);
    println!("{:#?}", client.clone());

    let response = client.get().await.ok().unwrap();
    println!("{:#?}", response);
    Ok(())
}
