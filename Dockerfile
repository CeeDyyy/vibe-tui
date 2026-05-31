# 1. Start with the official Rust environment
FROM rust:latest

# 2. Download and install ttyd, just like we did manually
RUN curl -Lo /usr/local/bin/ttyd https://github.com/tsl0922/ttyd/releases/download/1.7.3/ttyd.x86_64 \
    && chmod +x /usr/local/bin/ttyd

# 3. Create a working folder inside the container
WORKDIR /app

# 4. Copy all your Windows project files into the container
COPY . .

# 5. Compile the Rust app! (Using --release makes it heavily optimized and fast)
RUN cargo build --release

# 6. Expose the web server port
EXPOSE 7681

# 7. The single command that runs when the container starts up
# Note: Because we used --release, the file is now in /release/ instead of /debug/
CMD ["ttyd", "-p", "7681", "./target/release/app"]