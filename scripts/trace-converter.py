#!/usr/bin/env python3
import struct
import sys

def read_uint64_from_binary(filename):
    index = 0

    try:
        with open(filename, 'rb') as file:
            while True:
                # Read 8 bytes at a time
                bytes_read = file.read(8)
                
                # Break if we've reached the end of file or don't have enough bytes
                if not bytes_read or len(bytes_read) < 8:
                    break
                
                # Unpack as uint64 (little-endian) and convert to hex
                value = struct.unpack('<Q', bytes_read)[0]
                bytes = ""
                disassembly = ""
                registers = ""
                memory = ""
                comments = ""

                print(f"{index:02X},{value:016X},{bytes},{disassembly},{registers},{memory},{comments}")

                index += 1
                
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found")
    except Exception as e:
        print(f"Error: {str(e)}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: ./script.py <binary_file>")
        sys.exit(1)
    
    print("Index,Address,Bytes,Disassembly,Registers,Memory,Comments")
    read_uint64_from_binary(sys.argv[1])