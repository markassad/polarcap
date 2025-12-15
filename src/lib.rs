use pyo3::prelude::*;

mod parser;

/// A Polars interface to pcap (Packet Capture) data.
///
/// This module provides high-performance pcap file parsing using Rust,
/// with data exposed as Polars DataFrames for analysis in Python.
#[pymodule]
fn _polarcap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_init() {
        // Basic test to ensure module compiles
        assert_eq!(env!("CARGO_PKG_NAME"), "polarcap");
    }
}
