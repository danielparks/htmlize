# Docker image for running `cargo bench --features iai iai`. Use docker.sh to
# build and run.

FROM ubuntu:latest

RUN apt-get update \
  && apt-get install -y locales curl valgrind build-essential \
  && rm -rf /var/lib/apt/lists/* \
	&& localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8
ENV LANG=en_US.utf8

# Install rustup, rust, and cargo-binstall
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --default-toolchain stable \
  && curl -sSfL https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  | tar -C /root/.cargo/bin/ -xzf -

ENV PATH=/root/.cargo/bin:$PATH

COPY . /work
WORKDIR /work
RUN cargo build --benches --profile bench --all-features
