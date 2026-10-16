# Use CloudAMQP from Rust

A minimal example of publishing and consuming messages with
[amqp-client.rs](https://github.com/cloudamqp/amqp-client.rs), against
[CloudAMQP](https://www.cloudamqp.com) or any AMQP 0-9-1 broker such as
[LavinMQ](https://lavinmq.com) or RabbitMQ.

All code is in [`src/lib.rs`](src/lib.rs):

- `publish` publishes a persistent message and returns once the broker has confirmed it.
- `consume` consumes with a prefetch of 10, acking each message after the handler returns.
- `connect` connects using the `CLOUDAMQP_URL` environment variable, and
  recovers the connection, queues and consumers if it's lost.

## Usage

Tool versions are in `mise.toml`; install them with [mise](https://mise.jdx.dev),
or use stable Rust.

```sh
export CLOUDAMQP_URL="amqps://user:password@host/vhost"
cargo run --bin worker      # in one terminal
cargo run --bin publisher   # in another
```

Without `CLOUDAMQP_URL` it connects to `amqp://guest:guest@localhost`.

## Tests

`cargo test` publishes and consumes a message through a real broker on
localhost. CI runs it against LavinMQ.
