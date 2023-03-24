extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate serde;
extern crate serde_json;

mod data;

#[tokio::main]
async fn main() {
    let program_arguments = std::env::args().collect::<Vec<String>>();
    if 3 != program_arguments.len() {
        eprintln!("Usage: {} <search-query> <authentication-token>", &program_arguments[0]);
        return;
    }

    let search_query = &program_arguments[1];
    let authentication_token = &program_arguments[2];

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );

    let client = reqwest::Client::new();
    let response = match client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", authentication_token))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application.json")
        .send()
        .await {
            Ok(response) => response,
            Err(error) => {
                eprintln!("Error occurred while sending a request: {}.", error);
                return;
            }
        };

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.text().await {
                Ok(body) => println!("response:\n{:?}", body),
                Err(error) => {
                    eprintln!("Error occurred while trying to take the response body as a string: {}.", error);
                    return;
                }
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Unauthorized request, please provide a valid token!");
        }
        status => {
            println!("The request failed with status {}.", status);
        }
    }
}
