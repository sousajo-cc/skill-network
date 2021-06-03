FROM node:12
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        ssh \
        libssl-dev \
        pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/rust
ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:/rust/bin:$PATH

RUN echo -n "(curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly --no-modify-path) && rustup default nightly" > /install-rust.sh && \
    chmod 755 /install-rust.sh

RUN /install-rust.sh

RUN cargo install wasm-pack
RUN cargo install cargo-make
RUN rustup target add wasm32-unknown-unknown

ADD . ./

RUN yarn install --pure-lockfile
RUN yarn add webpack
RUN yarn add serve
RUN yarn add webpack-cli
RUN yarn add webpack-dev-server

EXPOSE 8080

CMD ["yarn", "start"]
