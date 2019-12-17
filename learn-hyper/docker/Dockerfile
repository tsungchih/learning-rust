FROM rust:alpine AS cargo-build

RUN mkdir /app
WORKDIR /app

COPY src /app/src
COPY Cargo.toml /app

RUN cargo build --release

#----------------------------------------------------------------
# The Second Stage Image Build
#----------------------------------------------------------------
FROM alpine:latest

#RUN apk update \
#    && apk add --update-cache ca-certificates libressl-dev \
#    && rm -rf /var/cache/apk/*

RUN addgroup -g 1000 appuser \
    && adduser -D -s /bin/sh -u 1000 -G appuser appuser \
    && mkdir -p /home/appuser/app/bin

WORKDIR /home/appuser/app/bin
COPY --from=cargo-build /app/target/release/app .
RUN chown -R appuser:appuser /home/appuser/app
USER appuser
CMD ["./app"]