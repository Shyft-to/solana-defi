use postgres::{Client, NoTls};

use postgres::Error as PostgresError;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use serde_json::{Value};

// DATABASE_URL
const DB_URL: &str = "postgres://postgres:monapaul5@localhost:5432/postgres";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// Updated Model: Transaction struct with new fields
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Transaction {
    pub id: Option<i32>,
    pub slot: i64,
    pub block_time: i64,
    pub signature: String,
    pub event: String
}

use tokio::task;

pub async fn set_database(transaction: &Transaction) -> Result<(), PostgresError> {
    // Clone the necessary data to avoid lifetime issues
    let signature = transaction.signature.clone();
    let event = transaction.event.clone().to_string().replace("\n", "").replace("\"", "").replace("\r", "");
    let slot = transaction.slot.clone();
    let block_time = transaction.block_time.clone();

    task::spawn_blocking(move || {
        let mut client = Client::connect(DB_URL, NoTls)?;
        // Try creating the transactions table if it doesn't exist
        let result = client.batch_execute(
            "CREATE TABLE IF NOT EXISTS pumpIndexer (
                id SERIAL PRIMARY KEY,
                slot BIGINT NOT NULL,
                blocktime BIGINT NOT NULL,
                signature VARCHAR NOT NULL,
                event VARCHAR NOT NULL
            )"
        );

        if let Err(e) = result {
            eprintln!("Error creating table: {}", e);
            return Err(e);
        }

        // Insert the transaction data
        let result = client.execute(
            "INSERT INTO pumpIndexer (slot,blocktime,signature, event) VALUES ($1, $2,$3,$4)",
            &[&slot,&block_time,&signature, &event]
        );
        if let Err(e) = result {
            eprintln!("Error inserting pumpIndexer: {}", e);
            return Err(e);
        }

        Ok(())
    }).await.unwrap() // await the blocking task
}

pub fn handle_get_request(request: &str) -> (String, String) {
    let signature = get_id(request).to_string();
    eprintln!("Extracted signature from request: {}", signature); // Debug print

    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            eprintln!("Querying for signature: {}", signature); // Debug print before query

            match client.query("SELECT * FROM pumpIndexer WHERE signature = $1", &[&signature]) {
                Ok(rows) => {
                    if rows.is_empty() {
                        eprintln!("No records found for signature: {}", signature);
                        return (NOT_FOUND.to_string(), format!("No record found for signature: {}", signature));
                    }

                    let transactions: Vec<Transaction> = rows.iter().map(|row| Transaction {
                        id: row.get::<_, Option<i32>>(0), // Handling nullable id
                        slot: row.get::<_, i64>(1), 
                        block_time: row.get::<_, i64>(2),
                        signature: row.get::<_, String>(3),
                        event: row.get::<_, String>(4),
                    }).collect();

                    eprintln!("Found transactions: {:?}", transactions);
                    (OK_RESPONSE.to_string(), serde_json::to_string(&transactions).unwrap())
                }
                Err(e) => {
                    eprintln!("Database query error: {:?}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "Database query failed".to_string())
                }
            }
        }
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            (INTERNAL_SERVER_ERROR.to_string(), "Database connection failed".to_string())
        }
    }
}

// Function to handle GET request for all transactions
pub fn handle_get_all_request() -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut transactions = Vec::new();

            for row in client.query("SELECT * FROM pumpIndexer", &[]).unwrap() {
                transactions.push(Transaction {
                    id: row.get(0),
                    slot: row.get(1),
                    block_time: row.get(2),
                    signature: row.get(3),
                    event: row.get(4)
                   // tx: serde_json::from_str(&row.get::<_, String>(2)).unwrap(), // Deserialize JSON string into `serde_json::Value`
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&transactions).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

// Function to drop the transactions table
pub fn drop_table() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Drop the transactions table entirely
    client.execute("DROP TABLE IF EXISTS pumpIndexer", &[])?;

    Ok(())
}

// Helper function to extract signature from the request URL
fn get_id(request: &str) -> &str {
     request.split('/')
        .nth(2)  // Get the correct segment (adjust index as needed)
        .unwrap_or("")  // Ensure we don’t panic
        .split_whitespace().next().unwrap_or("")  // Remove any trailing HTTP version or headers
        .trim()
           // Debug print
}
