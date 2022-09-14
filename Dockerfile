# See https://hub.docker.com/_/rust/

# This version of the container is distroless. It includes no package manager.
# If you need to extend the functionality, use the container defined in
# the Dockerfile-distro file.

FROM rust:latest as builder
WORKDIR /usr/src/newdoc
COPY . .
RUN cargo install --path .

FROM registry.access.redhat.com/ubi9-micro:latest
COPY --from=builder /usr/local/cargo/bin/newdoc /usr/local/bin/newdoc
# When running this container interactively, use `-v .:/mnt/newdoc:Z`
# to mount the current directory in the host to the container working dir.
VOLUME ["/mnt/newdoc"]
WORKDIR "/mnt/newdoc"
CMD ["newdoc"]
