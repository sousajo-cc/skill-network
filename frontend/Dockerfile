FROM rustlang/rust:nightly as builder
ADD . ./
RUN apt-get update && \
    apt-get install -y curl ca-certificates pkg-config libssl-dev git unzip --no-install-recommends && \
    curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
    apt-get update && \
    apt-get install -y nodejs yarn --no-install-recommends && \
    cargo install wasm-pack && \
    cargo install cargo-make && \
    rustup target add wasm32-unknown-unknown && \
    yarn install --pure-lockfile && \
    yarn add webpack && \
    yarn add serve && \
    yarn add webpack-cli && \
    yarn add webpack-dev-server

EXPOSE 8080
CMD ["yarn", "start"]
