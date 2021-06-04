# Skill Network

Frontend templated by [seed-quickstart-webpack](https://github.com/seed-rs/seed-quickstart-webpack).

## Setup

### Dependencies

Install docker

### Build and Run

```
sudo docker-compose up
```

After a while, the application will be available on localhost:8080

## Backend dev setup

### Dependencies

Install usual rust stuff (cargo, rustc nightly).
Install libsqlite3-dev.

```
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
diesel migration run
```

### Build and Run without docker

On the backend directory: ``cargo +nightly run``.

## Frontend dev setup

### Dependencies

Install usual rust stuff (cargo, rustc nightly).
Install Yarn.

```
cargo install cargo-make
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
cd frontend
yarn install --pure-lockfile
yarn global add webpack
yarn global add serve
yarn global add webpack-cli
yarn global add webpack-dev-server
```

### Build and Run without docker

On the frontend directory: ``yarn start``.

The application should be running on ``http://localhost:8080/``.

