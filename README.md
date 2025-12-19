# Redis Clone Rust

A simple redis like key-value store based on https://tokio.rs/tokio/tutorial.

---

The intentions behind the project are:

- learning the rust basics (first rust project)
- creating a database for https://github.com/jakob-rzeppa/http-server-c, to later implement multiple instances of the http-server and load balancing (probably also written in rust)

## Features

- [ ] Connection over tcp, so that https://github.com/jakob-rzeppa/http-server-c can use the database
- [ ] The cache itself
- [ ] Persistent data storage (AOF: logs changes -> logs can be replayed: see https://redis.io/docs/latest/operate/oss_and_stack/management/persistence/)
- [ ] (isn't really a feature) application tests

## Planning

### Architecture

- A connection spawns a tokio task
- One writer / multiple readers -> the value that shall be updated can't be read during the operation, but other values are able to be read while a different value is updated

### Requests

- u8 version
- u8 command (GET, SET, INSERT, REMOVE)
- u16 content length
- content of specified length

#### GET

- u32 id

#### SET

- u32 id
- content

#### INSERT

- content

#### REMOVE

- u32 id

### Response

the same metadata as the request

- u8 version
- u8 command
- u16 status code
- u16 content length
- content of specified length

#### Insert

- u32 id

#### Get

- content
