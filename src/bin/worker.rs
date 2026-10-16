use rust_amqp_example::{connect, consume};

#[tokio::main]
async fn main() -> Result<(), amqp_client_rs::Error> {
    let client = connect().await?;
    println!(" [*] Waiting for messages. To exit press CTRL+C");
    tokio::select! {
        result = consume(&client, "hello", |message| println!(" [x] Received '{message}'")) => result,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}
