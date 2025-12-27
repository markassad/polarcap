"""
Simple example demonstrating how to use polarcap to read and analyze pcap files.
"""

import polarcap
import polars as pl


def main():
    # Path to the example pcap file
    pcap_file = "tests/example.pcap"

    print(f"Reading pcap file: {pcap_file}\n")

    # Scan the pcap file using polarcap
    # Returns a Polars LazyFrame for efficient processing
    lf = polarcap.scan_pcap(pcap_file)

    # Collect and display basic information
    df = lf.collect()

    print(f"Total packets: {len(df)}")
    print("\nDataFrame schema:")
    print(df.schema)

    print("\nFirst 10 packets:")
    print(df.head(10))

    # Example: Filter and analyze specific packet types
    # (This depends on what fields are available in your pcap parser)
    print(f"\nDataFrame shape: {df.shape}")
    print(f"Columns: {df.columns}")


if __name__ == "__main__":
    main()
