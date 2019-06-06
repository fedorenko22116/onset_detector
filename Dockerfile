FROM rust:1.35

WORKDIR /src

COPY ./ ./

RUN cargo install --path . --all-features

ENV CURRENT_USER user

RUN adduser --disabled-password ${CURRENT_USER} && \
    usermod -a -G www-data ${CURRENT_USER}

USER ${CURRENT_USER}

WORKDIR /www/home/${CURRENT_USER}/storage

CMD ["audio_daemon", "0.0.0.0:8095"]

EXPOSE 8095
