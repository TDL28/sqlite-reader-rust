use sqlite;
use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};

fn main() {
    let mut help: bool = false;
    let mut db_path: String = String::new();
    let mut table: Option<String> = None;
    let mut columns: Option<String> = None;
    let mut list: bool = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("SQLite Reader");
        parser.refer(&mut help).add_option(
            &["-h", "--help"],
            StoreTrue,
            "Show this help message and exit.",
        );
        parser.refer(&mut db_path).add_option(
            &["-d", "--db", "--database"],
            Store,
            "Path to the SQLite database file.",
        );
        parser.refer(&mut table).add_option(
            &["-t", "--table"],
            StoreOption,
            "Name of the table to read from.",
        );
        parser.refer(&mut columns).add_option(
            &["-c", "--columns"],
            StoreOption,
            "Comma-separated list of columns to read.",
        );
        parser.refer(&mut list).add_option(
            &["-l", "--list"],
            StoreTrue,
            "If no table is provided, list all tables in the database. If table is provided, list all columns in the table.",
        );
        
        let _args = parser.parse_args();
    }

    if help {
        std::process::exit(0);
    }
    if db_path.is_empty() {
        println!("Error: Database path is required.\n");
        print_usage();
        std::process::exit(1);
    }

    let db = sqlite::open(db_path).unwrap_or_else(|_| {
        println!("Error: Could not open database.\n");
        print_usage();
        std::process::exit(1);
    });

    // check if user wants to list tables or columns
    if list {
        
        // if user provided a table name, list columns in that table
        if !table.is_none() {
            let columns = get_columns(&db, &table.clone().unwrap());
            println!("Columns in '{}'", table.unwrap());
            println!("---------------------");
            for column in columns {
                println!("{}", column);
            }
            std::process::exit(0);

        }

        // if no table name is provided, list all tables in the database
        let mut tables = get_tables(&db);
        tables.sort();
        println!("Tables");
        println!("---------------------");
        for table in tables {
            println!("{}", table);
        }
        std::process::exit(0);
    
    }

    // if not listing, make sure a table name is provided
    if table.is_none() {
        println!("Error: Table name is required when not listing.\n");
        print_usage();
        std::process::exit(1);
    }
    
    // if columns are provided, split them into a vector
    let cols: Vec<String> = columns.clone().unwrap_or_else(|| String::from("*")).split(',').map(|s| s.trim().to_string()).collect();
        

    let rows = get_table_content(&db, &table.unwrap(), &cols);
    for row in rows {
        println!("{}", row.join(","));
    }           
}


fn print_usage() {
    println!("SQLite Reader");
    println!("Usage: sqlite_reader [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -d, --db, --database  Path to the SQLite database file.");
    println!("  -h, --help            Show this help message and exit.");
    println!("  -t, --table           Name of the table to read from.");
    println!("  -c, --columns         Comma-separated list of columns to read.");
    println!("  -l, --list            List all tables or columns in the specified table.");
}


fn get_tables(db: &sqlite::Connection) -> Vec<String> {
    let mut tables = Vec::new();
    let mut statement = db.prepare("SELECT name FROM sqlite_master WHERE type='table'").unwrap();
    while let sqlite::State::Row = statement.next().unwrap() {
        let table_name: String = statement.read(0).unwrap();
        tables.push(table_name);
    }
    tables
}

fn get_columns(db: &sqlite::Connection, table_name: &str) -> Vec<String> {
    let mut columns = Vec::new();
    let mut statement = db.prepare(&format!("PRAGMA table_info({})", table_name)).unwrap();
    while let sqlite::State::Row = statement.next().unwrap() {
        let column_name: String = statement.read(1).unwrap();
        columns.push(column_name);
    }
    columns
}

fn get_table_content(db: &sqlite::Connection, table_name: &String, columns: &Vec<String>) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let column_names = columns.join(", ");
    let mut statement = db.prepare(&format!("SELECT {} FROM {}", column_names, table_name)).unwrap();
    
    while let Ok(sqlite::State::Row) = statement.next() {
        let mut row = Vec::new();
        for i in 0..statement.column_count() {
            let value: String = statement.read::<String,_>(i).unwrap_or_else(|_| String::from("NULL"));
            row.push(value.replace(",", "\\,"));
        }
        rows.push(row);
    }
    
    rows
}
