FROM rust:1.71

WORKDIR ../app

RUN apt-get update

RUN cargo install cargo-watch

CMD ["tail", "-f", "/dev/null"]