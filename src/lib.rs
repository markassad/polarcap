use polarcap_core::PCapParser;
use polars::prelude::*;
use pyo3::{exceptions, prelude::*};
use pyo3_polars::{PyDataFrame, PyExpr, PySchema};

/// A Polars interface to pcap (Packet Capture) data.
///
/// This module provides high-performance pcap file parsing using Rust,
/// with data exposed as Polars DataFrames for analysis in Python.
///
#[pyfunction]
fn add_numbers(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pyclass]
pub struct PCapSource {
    parser: Option<PCapParser>,
    size_hint: usize,
    n_rows: usize,
    total_read: usize,
}

// SAFETY: PCapSource is only accessed from Python, which has the GIL
// ensuring single-threaded access to the parser
unsafe impl Send for PCapSource {}
unsafe impl Sync for PCapSource {}

impl PCapSource {
    /// Returns the schema for pcap packet data
    fn pcap_schema() -> PySchema {
        let schema = Schema::from_iter(vec![
            // Packet metadata from pcap file
            Field::new(
                "timestamp".into(),
                DataType::Datetime(TimeUnit::Nanoseconds, None),
            ),
            Field::new("packet_number".into(), DataType::UInt64),
            Field::new("captured_length".into(), DataType::UInt32),
            Field::new("original_length".into(), DataType::UInt32),
            Field::new("data".into(), DataType::Binary),
        ]);
        PySchema(Arc::new(schema))
    }
}

#[pymethods]
impl PCapSource {
    #[new]
    #[pyo3(signature = (pcap_file, size_hint, n_rows))]
    fn new_source(
        pcap_file: &str,
        size_hint: Option<usize>,
        n_rows: Option<usize>,
    ) -> PyResult<Self> {
        let n_rows = n_rows.unwrap_or(usize::MAX);
        let size_hint = size_hint.unwrap_or(10_000);

        // Open the pcap file immediately
        let parser = match PCapParser::open(pcap_file) {
            Ok(p) => Some(p),
            Err(e) => {
                return Err(exceptions::PyIOError::new_err(format!(
                    "Failed to open pcap file: {}",
                    e
                )));
            }
        };

        Ok(Self {
            parser,
            size_hint,
            n_rows,
            total_read: 0,
        })
    }

    fn next(&mut self) -> PyResult<Option<PyDataFrame>> {
        // Check if we've already read the maximum number of rows
        if self.total_read >= self.n_rows {
            return Ok(None);
        }

        // Calculate how many rows to read in this batch
        let remaining = self.n_rows - self.total_read;
        let batch_size = std::cmp::min(self.size_hint, remaining);

        // Read the next batch from the parser
        if let Some(ref mut parser) = self.parser {
            match parser.read_batch(batch_size) {
                Ok(Some(df)) => {
                    let rows_read = df.height();
                    self.total_read += rows_read;

                    // If we've read more than n_rows, slice the DataFrame
                    if self.total_read > self.n_rows {
                        let excess = self.total_read - self.n_rows;
                        let keep = rows_read - excess;
                        self.total_read = self.n_rows;
                        Ok(Some(PyDataFrame(df.slice(0, keep))))
                    } else {
                        Ok(Some(PyDataFrame(df)))
                    }
                }
                Ok(None) => Ok(None),
                Err(e) => Err(exceptions::PyIOError::new_err(format!(
                    "Failed to read pcap batch: {}",
                    e
                ))),
            }
        } else {
            Ok(None)
        }
    }

    fn schema(&self) -> PySchema {
        Self::pcap_schema()
    }

    fn try_set_predicate(&mut self, _predicate: PyExpr) -> PyResult<()> {
        Ok(())
    }
}

#[pymodule]
fn _polarcap(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;

    m.add_class::<PCapSource>().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_workspace_hint() {
        // This test serves as a reminder to run workspace tests
        println!();
        println!("💡 TIP: Run 'cargo test --workspace' to test all crates in this workspace");
        println!("   - polarcap (this crate): Python bindings");
        println!("   - polarcap-core: Core Rust functionality");
        println!();

        // Basic test to ensure crate compiles
        assert_eq!(env!("CARGO_PKG_NAME"), "polarcap");
    }
}
