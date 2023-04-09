use sqlx::{migrate::MigrateDatabase, Sqlite};

mod players;

const DB_URL: &str = "sqlite://sqlite.db";


#[tokio::main]
async fn main() {

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db Success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let mut league_players = players::People::new();

    league_players.add_random_player();

    league_players.increase_ratings();

    league_players.say_hello(1);
}
