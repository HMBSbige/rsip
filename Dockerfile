FROM --platform=$BUILDPLATFORM rust AS builder

ARG TARGETPLATFORM

WORKDIR /app

COPY . .

RUN set -ex; \
    case "$TARGETPLATFORM" in \
    "linux/amd64") target='x86_64-unknown-linux-musl' ;; \
    "linux/arm64") target='aarch64-unknown-linux-musl' ;; \
    *) echo >&2 "error: unsupported $TARGETPLATFORM architecture"; exit 1 ;; \
    esac; \
    rustup target add $target; \
    cargo build --release --target $target; \
    mv target/$target/release/rsip /usr/local/bin/rsip; \
    chmod +x /usr/local/bin/rsip

RUN set -ex \
    && echo "Testing Docker image..." \
    && uname -a \
    && cat /etc/os-release \
    && file /usr/local/bin/rsip

FROM busybox:uclibc

COPY --from=builder /usr/local/bin/rsip /usr/local/bin/

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=1s --start-period=3s --retries=2 \
    CMD sh -c 'port=${RSIP_PORT_NUMBER:-80}; wget -q --spider 127.0.0.1:${port}/health >/dev/null 2>&1'

ENTRYPOINT ["/usr/local/bin/rsip"]
