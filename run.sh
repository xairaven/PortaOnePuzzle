#!/bin/bash

cargo build --release

EXECUTABLE="./target/release/PortaOnePuzzle"
DATA_FILE="./dev-resources/source.txt"
$EXECUTABLE --file "$DATA_FILE"