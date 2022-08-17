FROM jo3mccain/rusty as builder-base

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --workspace && \
    cargo test --all-features --color always --quiet --release --workspace


FROM photon as latest

COPY --from=builder-base /project/target/release/crypto-automata /crypto-automata

EXPOSE 9876/tcp
EXPOSE 9876/udp

CMD = ["./crypto-automata"]