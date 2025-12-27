"""
polarcap - A Polars interface to pcap (Packet Capture) data.

This package provides a high-performance interface for reading and analyzing
pcap files using Polars DataFrames. The core parsing and extraction logic is
implemented in Rust for maximum performance, while the Python API provides
a familiar and intuitive interface.

Example:
    >>> import polarcap
    >>> # Future usage example:
    >>> # df = polarcap.scan_pcap("capture.pcap")
    >>> # print(df.head())
"""

from typing import Iterator

import polars as pl
from polars.io.plugins import register_io_source

from polarcap._polarcap import PCapSource, __version__, add_numbers

__all__ = ["__version__", "add_numbers"]


def scan_pcap(pcap_file: str, size: int = 1000) -> pl.LazyFrame:
    def source_generator(
        with_columns: list[str] | None,
        predicate: pl.Expr | None,
        n_rows: int | None,
        batch_size: int | None,
    ) -> Iterator[pl.DataFrame]:
        new_size = size
        if n_rows is not None and n_rows < size:
            new_size = n_rows
        src = PCapSource(pcap_file, batch_size, new_size)

        predicate_set = True
        if predicate is not None:
            try:
                src.try_set_predicate(predicate)
            except pl.exceptions.ComputeError:
                predicate_set = False

        while (out := src.next()) is not None:
            # If the source could not apply the predicate
            # (because it wasn't able to deserialize it), we do it here.
            if not predicate_set and predicate is not None:
                out = out.filter(predicate)

            yield out

    src = PCapSource(pcap_file, 0, 0)

    return register_io_source(io_source=source_generator, schema=src.schema())
