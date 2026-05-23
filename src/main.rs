use cope::config::read_json_from;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExampleConfig {
    connection_string: String,
}

fn main() {
    let app_config: ExampleConfig = read_json_from("app.json").unwrap();
    println!("my connection string: {}", app_config.connection_string);
}
