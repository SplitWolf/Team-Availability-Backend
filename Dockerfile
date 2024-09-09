FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner app/recipe.json recipe.json
# Build Deps
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
# Build App
ARG SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin team-availablity-coordinator

FROM alpine as runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/dist2/ /usr/local/bin/dist/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/team-availablity-coordinator /usr/local/bin/team-availablity-coordinator

EXPOSE 3000
ENTRYPOINT [ "/usr/local/bin/team-availablity-coordinator" ]