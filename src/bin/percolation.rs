use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_union::QuickUnion;

#[derive(Debug)]
/// Represents a percolation grid
struct Percolation {
    /// The grid rows
    rows: usize,
    /// The grid columns
    columns: usize,
    /// A boolean vector that keeps track whether a particular site is open or not
    sites: Vec<bool>,
    /// The union find data structure that can quickly answer queries whether this grid percolates
    /// or not
    percolation_uf: QuickUnion,
}

impl Percolation {
    /// Creates the percolation structure
    ///
    /// # Arguments
    ///
    /// * `rows`: The number of rows in the grid
    /// * `columns`: The number of columns in the grid
    fn new(rows: usize, columns: usize) -> Self {
        if rows == 0 || columns == 0 {
            panic!("Dimensions should be greater that zero");
        }

        Self {
            rows,
            columns,
            sites: vec![false; rows * columns],
            percolation_uf: QuickUnion::new(rows * columns + 2),
        }
    }

    /// Read a percolation grid from a file.
    ///
    /// The file should have as the first row the size of the grid (only square grids are supported
    /// for the time being). The following lines should contain the row and the column of the site
    /// to open, as white space
    fn from_file(file: File) -> Percolation {
        // Open the input file
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Read the size of the grid
        let size = match lines.next() {
            Some(v) => match v.unwrap().parse::<usize>() {
                Ok(v) => v,
                Err(e) => panic!("Cannot parse first line as an integer: {:?}", e),
            },
            None => panic!("File is empty"),
        };

        // Create the supporting percolation data structure
        let mut percolation = Percolation::new(size, size);
        for line in lines {
            // Split lines as whitespace separated integers
            let dimension: Vec<usize> = line.unwrap().split_whitespace().map(
                |s| {
                    match s.parse::<usize>() {
                        Ok(v) => v,
                        Err(e) => panic!("Cannot parse component as an integer: {:?}", e),
                    }
                }
            ).collect();
            // Check if we got at least 2
            if dimension.len() < 2 {
                panic!("Cannot parse line {}: need at least two integers")
            }
            // Open site
            percolation.open(dimension[0], dimension[1])
        }

        percolation
    }

    /// Open a site
    ///
    /// * `row`: The index of the row to open (zero based)
    /// * `column`: The index of the column to open (zero based)
    fn open(&mut self, row: usize, column: usize) {
        // Mark the site as open
        let index = self.index(row, column);
        self.sites[index] = true;

        // Connect the site to adjacent sites
        if row > 1 && self.is_open(row - 1, column) {
            self.percolation_uf.union(index, self.index(row - 1, column));
        }
        if row < self.rows - 1 && self.is_open(row + 1, column) {
            self.percolation_uf.union(index, self.index(row + 1, column));
        }
        if column > 1 && self.is_open(row, column - 1) {
            self.percolation_uf.union(index, self.index(row, column - 1));
        }
        if column < self.columns - 1 && self.is_open(row, column + 1) {
            self.percolation_uf.union(index, self.index(row, column + 1));
        }

        // Connect with the 'dummy' top site
        if row == 0 {
            self.percolation_uf.union(0, index);
        }
        // Connect with the 'dummy' bottom site
        if row == self.rows - 1 {
            self.percolation_uf.union(self.rows * self.columns + 1, index);
        }
    }

    /// Check if a site is open
    ///
    /// * `row`: The index of the row to check (zero based)
    /// * `column`: The index of the column to check (zero based)
    fn is_open(&self, row: usize, column: usize) -> bool {
        let index = self.index(row, column);

        self.sites[index]
    }

    /// Transforms a grid row to an index that the `sites`
    ///
    /// * `row`: The index of the row (zero based)
    /// * `column`: The index of the column (zero based)
    fn index(&self, row: usize, column: usize) -> usize {
        if row >= self.rows || column > self.columns {
            panic!("The cell to open is out of bounds: {} {}", row, column);
        }

        row * self.rows + column
    }

    /// Returns true if the grid percolates, false otherwise
    fn percolates(&self) -> bool {
        self.percolation_uf.connected(0, self.columns * self.rows + 1)
    }
}

/// The main entry point of the program
fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file not provided")
    }

    // Open the input file
    let file = match File::open(&args[1]) {
        Ok(v) => v,
        Err(e) => panic!("Cannot open input file: {:?}", e),
    };
    let percolation = Percolation::from_file(file);

    println!("Percolates: {:?}", percolation.percolates());
}
