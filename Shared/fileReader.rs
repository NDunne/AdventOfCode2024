use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Trait for file processing
pub trait FileReader {
    /// Defines the operation to be performed on each row
    fn process_row(&mut self, row: &str);

    /// Reads the file line by line and applies the operation
    fn read_file(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let row = line?;
            self.process_row(&row);
        }
        Ok(())
    }
}
