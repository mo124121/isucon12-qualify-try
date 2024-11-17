FROM rust:1.82-bullseye

WORKDIR /tmp
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y wget gcc g++ make sqlite3 default-mysql-client

RUN useradd --uid=1001 --create-home isucon
USER isucon

RUN mkdir -p /home/isucon/webapp/rust
WORKDIR /home/isucon/webapp/rust
COPY --chown=isucon:isucon ./ /home/isucon/webapp/rust/

CMD [ "cargo", "run", "--release" ]
