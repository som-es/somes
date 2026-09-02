FROM rust:1.87

RUN apt-get update && apt-get install -y \
    libiconv-hook-dev \
    libtool \
    libxml2-dev \
    libxslt-dev \
    libclang-dev \
    libssl-dev \
    pkg-config \
    xmlsec1 \
    libxml2-dev libxmlsec1-dev libxmlsec1-openssl \
    postgresql-client \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

CMD ["./target/release/somes-api"]
