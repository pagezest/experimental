mod memory;
use memory::get_process_memory;
use rusqlite::{ Connection, Result };

#[derive(Debug)]
struct Person {
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let mem_before = get_process_memory();

    let conn = Connection::open_in_memory()?;
    let mem_after_conn = get_process_memory();

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        ()
    )?;
    let mem_after_table_creation = get_process_memory();

    let me = Person {
        name: "Steven".to_string(),
        data: None,
    };

    for _ in 0..5 {
        conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data))?;
    }
    let mem_after_insertions = get_process_memory();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    let persons: Vec<_> = person_iter.collect::<Result<_>>()?;
    assert_eq!(persons.len(), 5);

    let mem_after_query = get_process_memory();

    println!(
        "Memory usage for Rusqlite:\n\
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

    Ok(())
}
