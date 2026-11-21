# Install target machine

rustup target add wasm32-unknown-unknown

# Install trunk

cargo install --locked trunk

# Start server

cd <module_name>
trunk serve
