use clap::Parser;
use neocities::Neocities;
use neocities::NeocitiesError;

#[derive(Parser, Debug)]
struct Args {

    #[clap(long="username", short='u', default_value="", required=true)]
    username: String,

    #[clap(long="password", short='p', default_value="", required=true)]
    password: String,

    #[clap(long="filename", short='f', default_value="", required=true)]
    filename: String,
}

fn get_neo_client(username: String, password: String) -> Neocities {
    let client = Neocities::login(username, password);
    return client;
}

async fn upload_file(client: Neocities, filename: String) {
    let result = client.upload(filename.clone(), filename.clone())
        .await;
    let response = NeocitiesError::ApiErr(result);
    println!("{}", response);
}

fn main() {
    let args = Args::parse();
    let client = get_neo_client(args.username, args.password);
    let result = upload_file(client, args.filename);
}
