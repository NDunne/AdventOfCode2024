use std::fs::File;
use std::io::{BufRead, BufReader};

/// Trait for file processing
pub trait FileReader {
    /// Defines the operation to be performed on each row
    fn process_row(&mut self, row: &str) -> anyhow::Result<()>;

    /// Reads the file line by line and applies the operation
    fn process_file(&mut self, file_path: &str) -> anyhow::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.process_row(&line?)?;
        }
        Ok(())
    }
}

pub trait Solution {
    fn run(self) -> anyhow::Result<(impl ToString, impl ToString)>;
}
