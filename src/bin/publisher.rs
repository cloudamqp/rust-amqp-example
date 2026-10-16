use rust_amqp_example::{connect, publish};

#[tokio::main]
async fn main() -> Result<(), amqp_client_rs::Error> {
    let client = connect().await?;
    publish(&client, "hello", "Hello CloudAMQP!").await
}
