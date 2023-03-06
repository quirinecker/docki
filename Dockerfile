FROM rust:slim

WORKDIR /opt/rust

RUN apt update \
    && apt-get -y upgrade \
    && apt-get -y install libssl-dev pkg-config 
