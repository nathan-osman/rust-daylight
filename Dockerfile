FROM rust:latest

# Set the directory for the source code
WORKDIR /usr/src/daylight

# Copy the source code to the container
COPY . .

# Build the executable
RUN cargo build --release


FROM scratch

# Set the directory for the executable
WORKDIR /usr/local/bin

# Copy the executable to the container
COPY --from=0 /usr/src/daylight/target/release/daylight .

# Run the executable
ENTRYPOINT ["daylight"]
