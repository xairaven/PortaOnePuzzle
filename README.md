# PortaOne Puzzle Solver

> [!WARNING]
> This repository contains a solution to an internship test assignment. 
> Please do not copy or submit this code as your own. It is published solely 
> for portfolio and demonstration purposes.

## Task Description
The goal is to solve a digital puzzle by finding the longest continuous sequence 
from a given list of number fragments. The fragments can be connected if the last 
two digits of the first fragment exactly match the first two digits of the second 
fragment. Each fragment can be used only once.

For example, having the following fragments: `608017`, `248460`, `962282`, `994725`, `177092`.
By analyzing the edges, we can build the chain: `248460` & `608017` & `177092` -> `2484(60)80(17)7092`.
The resulting longest sequence is `24846080177092`.

## Solution Approach
The problem is modeled as finding the longest path in a directed graph. Every text 
fragment represents a node. A directed edge is created from node A to node B if the suffix 
(last 2 digits) of A matches the prefix (first 2 digits) of B.

To find the longest sequence, the program uses a Depth-First Search (DFS) algorithm with 
backtracking. It iterates through each fragment as a potential starting point, recursively 
explores all valid connections, and continuously keeps track of the maximum sequence length 
achieved. The internal state during traversal is managed efficiently using memory references 
to avoid unnecessary allocations.

## Technology Stack
The project is written entirely in Rust to ensure high performance and memory safety.
Dependencies include `clap` for command-line argument parsing and `thiserror` for convenient 
error handling.

## How to Run

The easiest way to run the program without installing any tools is to use the pre-compiled binaries. 
Go to the "Releases" page in this repository and download the archive for your operating system 
(Windows, macOS, or Linux). Extract the executable and run it via the terminal, passing the text 
file with your fragments as an argument:

```bash
./PortaOnePuzzle --file path/to/source.txt
```

If you have Rust and Cargo installed, you can compile and run the project directly 
from the source code. Clone the repository and execute the following command in 
the project root:

```bash
cargo run --release -- --file path/to/source.txt
```

Alternatively, a bash script is provided for UNIX-like systems. You can simply execute `./run.sh` 
which will build the release version and run it against the default `dev-resources/source.txt` file.
