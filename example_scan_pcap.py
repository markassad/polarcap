"""
Simple example demonstrating how to use polarcap to read and analyze pcap files.
"""

import polarcap
import polars as pl
import sys


def main():
    # Path to the example pcap file
    if len(sys.argv) > 1:
        pcap_file = sys.argv[1]
    else:
        pcap_file = "tests/example.pcap"

    print(f"Reading pcap file: {pcap_file}\n")

    # Scan the pcap file using polarcap
    # Returns a Polars LazyFrame for efficient processing
    lf = polarcap.scan_pcap(pcap_file, size=1000)

    # Collect and display basic information
    df = lf.collect()

    print(f"Total packets: {len(df)}")
    print("\nDataFrame schema:")
    print(df.schema)
    print(df)

    # Example: Filter and analyze specific packet types
    # (This depends on what fields are available in your pcap parser)
    print(f"\nDataFrame shape: {df.shape}")
    print(f"Columns: {df.columns}")


if __name__ == "__main__":
    main()
