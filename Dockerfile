FROM rust:1.82-slim as builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
        curl \
        tar \
        && rm -rf /var/lib/apt/lists/*

# Since the Binaryen in Debian is old, we need to install it from source
ENV BINARYEN_VERSION=120_b
ENV BINARYEN_URL=https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz

RUN curl -L -o binaryen.tar.gz ${BINARYEN_URL} && \
    tar -xzf binaryen.tar.gz && \
    mv binaryen-version_${BINARYEN_VERSION} /opt/binaryen && \
    ln -s /opt/binaryen/bin/wasm-opt /usr/local/bin/wasm-opt && \
    rm binaryen.tar.gz

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY index.html ./
COPY index.scss ./
COPY src ./src
COPY images ./images

RUN trunk build --release --dist=dist

FROM nginx:alpine

RUN rm -rf /usr/share/nginx/html/*

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
