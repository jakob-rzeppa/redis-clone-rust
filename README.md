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

### Requests v1

The connection should be a tcp stream in big-endian format

- first byte metadata (version of the request and command (GET, SET, INSERT, REMOVE))

#### GET

- 2-5 u32 id

#### SET

- 2-5 u32 id
- 6-7 u16 length
- rest data

#### INSERT

- 2-3 u16 length
- rest data

#### REMOVE

- 2-5 u32 id

### Response v1

> the response version is the same as the request one, so no need for metadata

#### Insert

u32 id

#### Get

- u16 length
- rest data
