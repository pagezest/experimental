mod memory;
use memory::get_process_memory;
use sqlite::{ Connection, State };

fn main() {
    let mem_before = get_process_memory();

    let connection = Connection::open(":memory:").unwrap();
    let mem_after_conn = get_process_memory();

    let query = "CREATE TABLE users (name TEXT, age INTEGER);";
    connection.execute(query).unwrap();
    let mem_after_table_creation = get_process_memory();

    let query = "INSERT INTO users VALUES (?, ?);";
    let mut statement = connection.prepare(query).unwrap();

    for _ in 0..5 {
        statement.bind((1, "Alice")).unwrap();
        statement.bind((2, 42)).unwrap();
        statement.next().unwrap();
        statement.reset().unwrap();
    }
    let mem_after_insertions = get_process_memory();

    let query = "SELECT * FROM users";
    let mut statement = connection.prepare(query).unwrap();

    let mut count = 0;
    while let Ok(State::Row) = statement.next() {
        let _: String = statement.read(0).unwrap();
        let _: i64 = statement.read(1).unwrap();
        count += 1;
    }
    assert_eq!(count, 5);
    let mem_after_query = get_process_memory();

    println!(
        "Memory usage for SQLite crate:\n\
         Before: {} kB\n\
         After opening DB: {} kB\n\
         After table creation: {} kB\n\
         After insertions: {} kB\n\
         After query execution: {} kB",
        mem_before,
        mem_after_conn,
        mem_after_table_creation,
        mem_after_insertions,
        mem_after_query
    );
}
