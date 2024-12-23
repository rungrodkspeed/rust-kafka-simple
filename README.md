# Simple Kafka with Rust

A lightweight and easy-to-understand implementation of a Kafka producer and consumer built using Rust. This project demonstrates how to interact with Apache Kafka in a simple, efficient way, leveraging the performance and safety features of Rust.

## Features

- **Producer**: Send messages to Kafka topics.
- **Consumer**: Read messages from Kafka topics.
- Lightweight and minimal dependencies.
- Clean and readable Rust code for beginners and advanced developers alike.

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.
- [Apache Kafka](https://kafka.apache.org/) set up and running.

## Usage

1. Clone this repository:
   ```bash
   git clone https://github.com/your-repo/simple-kafka-rust.git
   cd simple-kafka-rust
    ```

2. Start your Kafka server:
    ```bash
    docker compose up
    ```

3. Run the producer:
    ```bash
    cargo run --bin producer
    ```

4. Run the consumer:
    ```bash
    cargo run --bin consumer
    ```

## Demo

![Demo](media/demo.gif)


## References

- [Using Kafka with Rust](https://www.arroyo.dev/blog/using-kafka-with-rust)
- [การใช้งาน Kafka ใน Golang: From Zero to Hero (Maybe)](https://medium.com/@rayato159/%E0%B8%81%E0%B8%B2%E0%B8%A3%E0%B9%83%E0%B8%8A%E0%B9%89%E0%B8%87%E0%B8%B2%E0%B8%99-kafka-%E0%B9%83%E0%B8%99-golang-%E0%B8%88%E0%B8%B2%E0%B8%81-zero-to-hero-maybe-a9538720ca75)