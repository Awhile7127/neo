// Library for command-line argument parsing
use clap::Parser;

// Libraries for handling Neocities API requests and subsequent errors
use neocities::Neocities;

// Library for handling asynchronous functions
use tokio;

// Library for handling reading file content
use std::fs;

// Build command-line arguments
#[derive(Parser, Debug)]
struct Args {

    #[clap(long="key", short='k', default_value="", required=true)]
    key: String,

    #[clap(long="filename", short='f', default_value="", required=true)]
    filename: String,
}

// Fetch authenticated Neocities client
fn get_neo_client(key: String) -> Neocities {
    let client = Neocities::new(key);
    return client;
}

fn read_file(file: String) -> String {
    let content = fs::read_to_string(file).unwrap();
    return content;
}

// Handle uploading files via the API
async fn upload_file(client: Neocities, filename: String, content: String) -> String {

    // Upload file via the API
    // client.upload takes to args: file_path (String) and file (T)

    // Removing `.unwrap()` returns a Result<T, E> of
    // Result<String, NeocitiesError>
    let result = client.upload(filename.clone(), content)
        .await;

    // Catch errors
    let response = match result {
        Ok(res) => res,
        Err(error) => error.to_string(),
    };

    return response;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = get_neo_client(args.key);
    let content = read_file(args.filename.clone());
    let result = upload_file(client, args.filename.clone(), content).await;
    println!("{:?}", result);
}
