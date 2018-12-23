FROM rustlang/rust:nightly

# Set the directory for the source code
WORKDIR /usr/src/daylight

# Copy the source code to the container
COPY . .

# Build the executable
RUN cargo build --release


FROM debian:stretch

# Set the directory for the executable
WORKDIR /opt/daylight

# Copy the static files to the container
COPY static ./static

# Copy the executable to the container
COPY --from=0 /usr/src/daylight/target/release/daylight .

# Run the executable
ENTRYPOINT ["/opt/daylight/daylight"]
