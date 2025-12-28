/// Parser module for pcap files.
///
/// This module contains the core parsing logic for pcap files,
/// converting packet data into Polars-compatible structures.
use pcap::{Capture, Precision};
use polars::prelude::*;
use std::path::Path;

/// Streaming pcap parser that reads packets incrementally
pub struct PCapParser {
    capture: Capture<pcap::Offline>,
    packet_counter: u64,
}

impl PCapParser {
    /// Open a pcap file for streaming
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let capture = Capture::from_file_with_precision(path, Precision::Nano)?;
        Ok(Self {
            capture,
            packet_counter: 0,
        })
    }

    /// Read the next batch of packets and return as a DataFrame
    pub fn read_batch(
        &mut self,
        batch_size: usize,
    ) -> Result<Option<DataFrame>, Box<dyn std::error::Error>> {
        // Use Polars builders for more efficient Series construction
        let mut timestamp_builder =
            PrimitiveChunkedBuilder::<Int64Type>::new("timestamp".into(), batch_size);
        let mut packet_number_builder =
            PrimitiveChunkedBuilder::<UInt64Type>::new("packet_number".into(), batch_size);
        let mut captured_length_builder =
            PrimitiveChunkedBuilder::<UInt32Type>::new("captured_length".into(), batch_size);
        let mut original_length_builder =
            PrimitiveChunkedBuilder::<UInt32Type>::new("original_length".into(), batch_size);
        let mut data_builder = BinaryChunkedBuilder::new("data".into(), batch_size);

        let mut count = 0;

        // Read up to batch_size packets
        while count < batch_size {
            match self.capture.next_packet() {
                Ok(packet) => {
                    // Convert timestamp to nanoseconds
                    let ts_nanos =
                        packet.header.ts.tv_sec * 1_000_000_000 + packet.header.ts.tv_usec;

                    timestamp_builder.append_value(ts_nanos);
                    packet_number_builder.append_value(self.packet_counter);
                    captured_length_builder.append_value(packet.header.caplen);
                    original_length_builder.append_value(packet.header.len);
                    data_builder.append_value(packet.data);

                    self.packet_counter += 1;
                    count += 1;
                }
                Err(_) => {
                    // End of file or error
                    break;
                }
            }
        }

        // If no packets were read, return None
        if count == 0 {
            return Ok(None);
        }

        // Finish building the series
        let timestamp_series = timestamp_builder.finish().into_series();
        let packet_number_series = packet_number_builder.finish().into_series();
        let captured_length_series = captured_length_builder.finish().into_series();
        let original_length_series = original_length_builder.finish().into_series();
        let data_series = data_builder.finish().into_series();

        // Convert timestamp to Datetime type with nanosecond precision
        let timestamp_series =
            timestamp_series.cast(&DataType::Datetime(TimeUnit::Nanoseconds, None))?;

        // Create DataFrame from series
        let df = DataFrame::new(vec![
            timestamp_series.into(),
            packet_number_series.into(),
            captured_length_series.into(),
            original_length_series.into(),
            data_series.into(),
        ])?;

        Ok(Some(df))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_pcap_module() {
        // Basic test to ensure module compiles
    }
}
