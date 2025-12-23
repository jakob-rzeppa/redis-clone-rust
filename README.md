# Redis Clone Rust

A simple redis like key-value store based on https://tokio.rs/tokio/tutorial.

---

The intentions behind the project are:

- learning the rust basics (first rust project)
- creating a database for https://github.com/jakob-rzeppa/http-server-c, to later implement multiple instances of the http-server and load balancing (probably also written in rust)

## Features

- [x] Connection over tcp, so that https://github.com/jakob-rzeppa/http-server-c can use the database
- [x] The cache itself (GET, SET, INSERT, REMOVE)
- [ ] Persistent data storage (AOF (Append Only File): log changes -> logs can be replayed: see https://redis.io/docs/latest/operate/oss_and_stack/management/persistence/)
- [ ] (isn't really a feature) application tests

## Planning

### Architecture

- A connection spawns a tokio task
- Read / write locked individual entries, so multiple values can be read / written to at the same time.
- The full cache shall only be locked if an entry is being removed or inserted

### Requests

- u8 version
- u8 command (GET, SET, INSERT, REMOVE)
- u16 content length
- content of specified length
- EOT (End of Transmission) ASCII control character 00000100 (4)

#### GET content

- u32 id

#### SET content

- u32 id
- data

#### INSERT content

- data

#### REMOVE content

- u32 id

### Response

the same metadata as the request

- u8 version
- u8 command
- u16 status code
- u16 content length
- content of specified length
- EOT (End of Transmission) ASCII control character 00000100 (4)

#### Insert content

- u32 id

#### Get content

- content
