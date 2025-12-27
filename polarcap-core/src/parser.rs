/// Parser module for pcap files.
///
/// This module contains the core parsing logic for pcap files,
/// converting packet data into Polars-compatible structures.
use pcap_parser::*;
use polars::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Parse a pcap file and return a Polars DataFrame
pub fn parse_pcap<P: AsRef<Path>>(path: P) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut timestamps = Vec::new();
    let mut packet_numbers = Vec::new();
    let mut captured_lengths = Vec::new();
    let mut original_lengths = Vec::new();
    let mut data_packets = Vec::new();

    let mut packet_num: u64 = 0;

    // Parse pcap header
    let (mut rem, _header) = match parse_pcap_header(&buffer) {
        Ok(result) => result,
        Err(e) => return Err(format!("Failed to parse pcap header: {:?}", e).into()),
    };

    // Parse packets
    loop {
        match parse_pcap_frame(rem) {
            Ok((remaining, frame)) => {
                // Convert timestamp to microseconds
                let ts_micros = (frame.ts_sec as i64) * 1_000_000 + (frame.ts_usec as i64);

                timestamps.push(ts_micros);
                packet_numbers.push(packet_num);
                captured_lengths.push(frame.caplen);
                original_lengths.push(frame.origlen);
                data_packets.push(frame.data.to_vec());

                packet_num += 1;
                rem = remaining;
            }
            Err(nom::Err::Incomplete(_)) | Err(nom::Err::Error(_)) => break,
            Err(nom::Err::Failure(e)) => {
                return Err(format!("Parse error: {:?}", e).into());
            }
        }
    }

    let df = df!(
        "timestamp" => timestamps,
        "packet_number" => packet_numbers,
        "captured_length" => captured_lengths,
        "original_length" => original_lengths,
        "data" => data_packets,
    )?;

    // Convert timestamp column to Datetime type
    let df = df
        .lazy()
        .with_column(col("timestamp").cast(DataType::Datetime(TimeUnit::Microseconds, None)))
        .collect()?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_pcap_module() {
        // Basic test to ensure module compiles
    }
}
