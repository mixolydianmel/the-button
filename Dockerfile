FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/the-button
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/the-button/target/release/backend /usr/local/bin/backend
COPY --from=build /usr/src/the-button/frontend/dist /usr/local/bin/frontend/dist

EXPOSE 8080

WORKDIR /usr/local/bin
CMD ["backend"]
