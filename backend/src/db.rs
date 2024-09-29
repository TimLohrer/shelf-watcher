use rocket_db_pools::{mongodb::{Client, Database}, Connection, Database as RocketDB}; 

#[derive(RocketDB)] 
#[database("shelfwatcher-db")] 
pub struct ShelfWatcherDatabase(Client);

pub fn get_main_db(connection: &Connection<ShelfWatcherDatabase>) -> Database {
    connection.database("shelfwatcher_data")
}

pub fn get_logs_db(connection: &Connection<ShelfWatcherDatabase>) -> Database {
    connection.database("shelfwatcher_logs")
}