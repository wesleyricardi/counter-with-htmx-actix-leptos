FROM rust:1.71

WORKDIR ../app

RUN apt-get update
RUN rustup component add rustfmt
RUN cargo install cargo-watch

CMD ["tail", "-f", "/dev/null"]