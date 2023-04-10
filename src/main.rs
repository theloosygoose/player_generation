use rusqlite::{Connection, Result};
mod players;

fn main() {

    fn create_table(conn: &Connection){
        match conn.execute(
            "CREATE TABLE PLAYER (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                age             INTEGER NOT NULL,
                offense_rating  INTEGER NOT NULL, 
                defense_rating  INTEGER NOT NULL
            );",
            (), //empty list of params.
        ) {
            Ok(created) => println!("{} Table Person was created", created),
            Err(err) => println!("Table Creation Failed: {}", err),
        };
    }

    fn save_game() -> Result<()> {
        let path = "./players.db";
        let db = Connection::open(path)?;

        create_table(&db);

        println!("{}", db.is_autocommit());
        Ok(())
    }

    let mut all_players = players::People::new();

    all_players.add_random_player();

    println!("ALL PLAYERS: {:?}", all_players);

    match save_game() {
        Ok(saved) => println!("{:?} Saved Game", saved),
        Err(err) => println!("Error in Saving Game {}", err),
    }


}
