# rustbridge.io
[![Build Status](https://travis-ci.org/rustbridge/rustbridge.io.svg?branch=master)](https://travis-ci.org/rustbridge/rustbridge.io)

Clone to rust of https://rustbridge.github.io/

# Rustbridge
Free and guided workshops in Rust – a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

Our events are aimed at people who are underrepresented in technology and offer the opportunity to learn within a friendly, safe and sometimes fun environment. We do this by providing rewarding learning experiences, building helpful tools as well as materials and gathering a group of awesome humans. RustBridge is a project by the Rust Language Community Team.

# Requirements 
* Rust development environment setup: [Rust](https://www.rust-lang.org/en-US/)
* PostgreSQL database driver: [Diesel](http://diesel.rs/guides/getting-started/)
* PostgreSQL database setup: [PostgreSQL](https://www.postgresql.org/)

*Note*: some of the dependencies used in this project use nightly builds. You will need to use nightly to run the project. To install nightly run: `rustup install nightly` to use nightly run: `rustup run nightly rustc`, and to set default to nightly: `rustup default nightly`

# Usage
In a terminal:
1. git clone git@github.com:rustbridge/rustbridge.io.git
2. cd rustbridge.io
3. cargo run

Navigate with a web browser to http://localhost:8000

### Database setup
To set up the local PostgreSQL database, run the command:
 `echo DATABASE_URL=postgres://[username]:[password]@localhost/rustbridge`. 

*Note* on windows you'll need to set the `DATABASE_URL` environment variable by running (as administrator):
`setx /m DATABASE_URL postgres://[username]:[password]@localhost/rustbridge`

Unless otherwise specified, the username should be postgres, and the password is whatever you set during the postgres installation. 

Once connection is established, run: 
```
$ diesel setup
> Creating database: rustbridge
```
Then run the [migration](https://en.wikipedia.org/wiki/Schema_migration) script. 
```
$ diesel migration run
> Running migration [current migration]
```
Which will apply all current migrations to your database.

To add organizers to the database, make sure all migrations are applied, and follow the instructions on the [Rustbridge-Cli](https://github.com/rustbridge/rustbridge-cli).

# License 
Rustbridge is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), 

See LICENSE-APACHE and LICENSE-MIT for details.
