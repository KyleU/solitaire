FROM alpine:3.10
#RUN apk add libseccomp
COPY target/x86_64-unknown-linux-musl/release/solitaire .
EXPOSE 5050/tcp
ENTRYPOINT ["/solitaire"]
