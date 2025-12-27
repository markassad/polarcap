use polarcap_core::parse_pcap;
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
    pcap_file: String,
    size_hint: usize,
    n_rows: usize,
    data: Option<DataFrame>,
    current_offset: usize,
}

impl PCapSource {
    /// Returns the schema for pcap packet data
    fn pcap_schema() -> PySchema {
        let schema = Schema::from_iter(vec![
            // Packet metadata from pcap file
            Field::new(
                "timestamp".into(),
                DataType::Datetime(TimeUnit::Microseconds, None),
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

        Ok(Self {
            pcap_file: pcap_file.to_string(),
            size_hint,
            n_rows,
            data: None,
            current_offset: 0,
        })
    }

    fn next(&mut self) -> PyResult<Option<PyDataFrame>> {
        // Load data on first call
        if self.data.is_none() {
            match parse_pcap(&self.pcap_file) {
                Ok(df) => {
                    self.data = Some(df);
                }
                Err(e) => {
                    return Err(exceptions::PyIOError::new_err(format!(
                        "Failed to parse pcap file: {}",
                        e
                    )));
                }
            }
        }

        // Return data in chunks based on size_hint
        if let Some(ref df) = self.data {
            if self.current_offset >= df.height() {
                return Ok(None);
            }

            let end = std::cmp::min(self.current_offset + self.size_hint, df.height());
            let end = std::cmp::min(end, self.current_offset + self.n_rows);

            let slice = df.slice(self.current_offset as i64, end - self.current_offset);
            self.current_offset = end;

            Ok(Some(PyDataFrame(slice)))
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
