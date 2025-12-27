"""
Generate test pcap files with realistic network packets using scapy.
"""

from scapy.all import IP, TCP, UDP, ICMP, ARP, Ether, wrpcap
from scapy.layers.dns import DNS, DNSQR
from scapy.layers.http import HTTP, HTTPRequest
import random


def generate_varied_packets(count=100):
    """Generate a variety of realistic network packets."""
    packets = []

    for i in range(count):
        # Vary source and destination IPs
        src_ip = f"192.168.1.{(i % 250) + 1}"
        dst_ip = f"10.0.0.{(i % 250) + 1}"

        # Create different types of packets
        if i % 10 == 0:
            # ARP packet
            pkt = Ether(dst="ff:ff:ff:ff:ff:ff") / ARP(
                op=1,  # ARP request
                psrc=src_ip,
                pdst=dst_ip
            )
        elif i % 8 == 0:
            # ICMP packet (ping)
            pkt = Ether() / IP(src=src_ip, dst=dst_ip) / ICMP(
                type=8,  # Echo request
                id=i,
                seq=i
            )
        elif i % 5 == 0:
            # DNS query
            pkt = Ether() / IP(src=src_ip, dst="8.8.8.8") / UDP(
                sport=random.randint(50000, 60000),
                dport=53
            ) / DNS(rd=1, qd=DNSQR(qname=f"example{i}.com"))
        elif i % 3 == 0:
            # UDP packet
            pkt = Ether() / IP(src=src_ip, dst=dst_ip) / UDP(
                sport=random.randint(1024, 65535),
                dport=random.choice([53, 123, 161, 500])
            ) / (b"UDP payload " + str(i).encode())
        else:
            # TCP packet with varying flags
            flags = ["S", "A", "SA", "F", "R", "P"][i % 6]
            pkt = Ether() / IP(src=src_ip, dst=dst_ip) / TCP(
                sport=random.randint(1024, 65535),
                dport=random.choice([80, 443, 22, 3389, 8080]),
                flags=flags,
                seq=i * 1000,
                ack=i * 500 if "A" in flags else 0
            )

            # Add payload for some TCP packets
            if i % 4 == 0:
                pkt = pkt / (b"HTTP/1.1 200 OK\r\n" if i % 2 else b"GET / HTTP/1.1\r\n")

        # Set timestamp
        pkt.time = 1640000000 + i
        packets.append(pkt)

    return packets


if __name__ == "__main__":
    import os

    # Ensure we're in the tests directory
    test_dir = os.path.dirname(os.path.abspath(__file__))
    pcap_file = os.path.join(test_dir, "example.pcap")

    print("Generating 100 varied network packets...")
    packets = generate_varied_packets(100)

    print(f"Writing packets to {pcap_file}...")
    wrpcap(pcap_file, packets)

    print(f"Successfully created {pcap_file}")
    print(f"  Total packets: {len(packets)}")
    print(f"  Packet types: TCP, UDP, ICMP, ARP, DNS")
