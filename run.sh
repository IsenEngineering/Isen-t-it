#!/bin/bash

# Check if the first argument is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 {client|server} [wasm|native] [dev|release]"
  exit 1
fi

# Argument 1: Determine the binary to run
if [ "$1" = "client" ]; then
  BIN="client"
elif [ "$1" = "server" ]; then
  BIN="server"
else
  echo "Invalid first argument: $1. Must be 'client' or 'server'."
  exit 1
fi

# Argument 2: Determine the target (default: native)
TARGET=${2:-native}
if [ "$TARGET" = "wasm" ]; then
    if [ "$BIN" = "server" ]; then
        echo "Error: 'server' cannot be used with 'wasm' target."
        exit 1
    fi
  TARGET_FLAG="--target wasm32-unknown-unknown"
elif [ "$TARGET" = "native" ]; then
  TARGET_FLAG=""
else
  echo "Invalid second argument: $2. Must be 'wasm' or 'native'."
  exit 1
fi

# Argument 3: Determine the build mode (default: dev)
MODE=${3:-dev}
if [ "$MODE" = "release" ]; then
  MODE_FLAG="--release"
elif [ "$MODE" = "dev" ]; then
  MODE_FLAG=""
else
  echo "Invalid third argument: $3. Must be 'dev' or 'release'."
  exit 1
fi

# Execute the constructed command
COMMAND="cargo run --bin $BIN $TARGET_FLAG $MODE_FLAG"
echo "Executing: $COMMAND"
$COMMAND
