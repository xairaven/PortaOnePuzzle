// Represents the core puzzle solving engine
pub struct PuzzleSolver<'a> {
    pieces: Vec<&'a str>,
}

impl<'a> PuzzleSolver<'a> {
    // Initialize a new solver with the given pieces
    pub fn new(pieces: Vec<&'a str>) -> Self {
        Self { pieces }
    }

    // Execute the search for the longest possible sequence
    pub fn solve(&self) -> String {
        let n = self.pieces.len();
        if n == 0 {
            return String::new();
        }

        let mut adj = vec![vec![]; n];

        // Build the adjacency list for the directed graph
        for (i, row) in adj.iter_mut().enumerate().take(n) {
            for j in 0..n {
                if i != j {
                    let a = self.pieces[i];
                    let b = self.pieces[j];
                    // Check if the last two chars match the first two chars
                    if a.len() >= 2 && b.len() >= 2 && a[a.len() - 2..] == b[..2] {
                        row.push(j);
                    }
                }
            }
        }

        // Initialize the internal state for the search
        let mut search_state = SearchState {
            pieces: &self.pieces,
            adj: &adj,
            visited: vec![false; n],
            max_chain: String::new(),
            max_len: 0,
        };

        // Run DFS from each piece as a starting point
        for i in 0..n {
            search_state.visited[i] = true;

            search_state.dfs(i, self.pieces[i].to_string(), 1);

            // Reset for the next starting node
            search_state.visited[i] = false;
        }

        search_state.max_chain
    }
}

// Internal structure to hold the state during the DFS traversal
struct SearchState<'a> {
    pieces: &'a [&'a str],
    adj: &'a [Vec<usize>],
    visited: Vec<bool>,
    max_chain: String,
    max_len: usize,
}

impl<'a> SearchState<'a> {
    // Recursive depth-first search to find the longest chain
    fn dfs(&mut self, current_node: usize, current_chain: String, current_len: usize) {
        // Update the maximum chain found so far
        if current_len > self.max_len {
            self.max_len = current_len;
            self.max_chain = current_chain.clone();
        }

        // Explore all valid neighbors
        for &next_node in &self.adj[current_node] {
            if !self.visited[next_node] {
                self.visited[next_node] = true;

                let next_piece = self.pieces[next_node];
                // Append the new piece excluding the first two duplicated digits
                let new_chain = format!("{}{}", current_chain, &next_piece[2..]);

                self.dfs(next_node, new_chain, current_len + 1);

                // Backtrack to allow other combinations
                self.visited[next_node] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let numbers = vec!["608017", "248460", "962282", "994725", "177092"];
        let solver = PuzzleSolver::new(numbers);

        let actual = solver.solve();
        let expected = String::from("24846080177092");

        assert_eq!(expected, actual);
    }
}
