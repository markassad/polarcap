use pyo3::prelude::*;

/// A Polars interface to pcap (Packet Capture) data.
///
/// This module provides high-performance pcap file parsing using Rust,
/// with data exposed as Polars DataFrames for analysis in Python.
#[pymodule]
fn _polarcap(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_workspace_hint() {
        // This test serves as a reminder to run workspace tests
        println!("");
        println!("💡 TIP: Run 'cargo test --workspace' to test all crates in this workspace");
        println!("   - polarcap (this crate): Python bindings");
        println!("   - polarcap-core: Core Rust functionality");
        println!("");

        // Basic test to ensure crate compiles
        assert_eq!(env!("CARGO_PKG_NAME"), "polarcap");
    }
}
