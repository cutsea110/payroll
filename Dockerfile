FROM rust:latest AS builder

WORKDIR /work/
COPY . /work/
RUN cargo update
RUN cargo build --release

FROM debian:sid-slim

COPY --from=builder /work/target/release/payroll-app /usr/bin/payroll-app
CMD ["payroll-app", "-h"]
