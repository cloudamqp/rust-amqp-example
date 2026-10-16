use amqp_client_rs::{
    BasicProperties, Client, Error, QueueDeclareOpts, RecoveryConfig, SubscribeOpts,
};

/// Connects to CLOUDAMQP_URL, and recovers the connection, queues and consumers if it's lost.
pub async fn connect() -> Result<Client, Error> {
    let url =
        std::env::var("CLOUDAMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost".into());
    Client::builder(&url)?
        .recovery(RecoveryConfig::default())
        .build()
        .await
}

fn durable() -> QueueDeclareOpts {
    QueueDeclareOpts {
        durable: true,
        ..Default::default()
    }
}

/// Publishes a persistent message and returns once the broker has confirmed it.
pub async fn publish(client: &Client, queue: &str, message: &str) -> Result<(), Error> {
    client.queue(queue, durable()).await?;
    let props = BasicProperties::builder()
        .content_type("text/plain")
        .persistent()
        .build()?;
    client
        .publish_confirmed_with_props("", queue, &message.as_bytes().to_vec(), props)
        .await?;
    println!(" [x] Sent '{message}'");
    Ok(())
}

/// Calls `handler` for each message, with a prefetch of 10, and acks the message after it returns.
pub async fn consume(
    client: &Client,
    queue: &str,
    mut handler: impl FnMut(String),
) -> Result<(), Error> {
    let queue = client.queue(queue, durable()).await?;
    let mut consumer = queue
        .subscribe::<Vec<u8>>(SubscribeOpts {
            prefetch: 10,
            ..Default::default()
        })
        .await?;
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(d) => {
                handler(String::from_utf8_lossy(&d.body).into_owned());
                d.ack()?;
            }
            // Not decodable: dead-letter it instead of redelivering it forever
            Err(f) => f.reject(false)?,
        }
    }
    Ok(())
}
