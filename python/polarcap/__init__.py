"""
polarcap - A Polars interface to pcap (Packet Capture) data.

This package provides a high-performance interface for reading and analyzing
pcap files using Polars DataFrames. The core parsing and extraction logic is
implemented in Rust for maximum performance, while the Python API provides
a familiar and intuitive interface.

Example:
    >>> import polarcap
    >>> # Future usage example:
    >>> # df = polarcap.read_pcap("capture.pcap")
    >>> # print(df.head())
"""

from polarcap._polarcap import __version__, add_numbers

__all__ = ["__version__", "add_numbers"]
