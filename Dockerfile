FROM rust:1.35

WORKDIR /src

COPY ./ ./

RUN cargo install --path . --all-features

WORKDIR /var/storage

CMD ["audio_daemon", "0.0.0.0:8095"]

EXPOSE 8095
