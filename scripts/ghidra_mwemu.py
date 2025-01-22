'''
    View in ghidra the trace of emulation by changin color of emulated instructions, and save a call log with ghidra function names.

    1. store mwemu trace with verbose
    cargo run --release -- -6 -vv -f file.bin > log 2>&1

    2. put this script here and launch it from hidra with the same binary opened.
    /Users/jesus/ghidra/Ghidra/Features/Jython/ghidra_scripts
'''


import os
import re
from java.awt import Color
from ghidra.program.model.address import Address
from ghidra.program.model.address import AddressSet

def parse_emulation_log(file_path):
    EMULATION_MARKER = " ----- emulation -----"
    ADDRESS_PATTERN = r"^([0-9]+)\s+(0x[0-9a-fA-F]+)"
    CALL_PATTERN = r"call\s+([0-9a-fA-F]+)h"

    address_regex = re.compile(ADDRESS_PATTERN)
    call_regex = re.compile(CALL_PATTERN)

    addresses = []
    call_output_path = os.path.splitext(file_path)[0] + ".call"

    with open(call_output_path, "w") as call_file:
        with open(file_path, "r") as log_file:
            lines = log_file.readlines()
            marker_found = False
            for line in lines:
                if EMULATION_MARKER in line:
                    marker_found = True
                    continue

                if marker_found:
                    address_match = address_regex.search(line)
                    if address_match:
                        address = address_match.group(2)
                        addresses.append(address)

                    call_match = call_regex.search(line)
                    if call_match:
                        call_address_hex = call_match.group(1)
                        call_address_str = "0x" + call_address_hex
                        function_name = get_function_name(call_address_str)
                        if function_name:
                            call_file.write("Call to function: {} at address {}\n".format(function_name, call_address_str))
                        else:
                            call_file.write("No function name found for address: {}\n".format(call_address_str))

    print("Call data saved to {}".format(call_output_path))
    return addresses

def get_function_name(address_str):
    current_program = getCurrentProgram()
    symbol_table = current_program.getSymbolTable()
    address = toAddr(address_str)
    symbol = symbol_table.getPrimarySymbol(address)

    if symbol:
        return symbol.getName()

    function_manager = current_program.getFunctionManager()
    function = function_manager.getFunctionAt(address)
    if function:
        return function.getName()

    return None

def highlight_addresses(addresses):
    current_program = getCurrentProgram()
    listing = current_program.getListing()

    min_address = current_program.getMinAddress()
    max_address = current_program.getMaxAddress()

    address_set = AddressSet()
    valid_addresses = 0

    for address_str in addresses:
        address = toAddr(address_str)
        if min_address <= address <= max_address:
            address_set.add(address)
            valid_addresses += 1

    setBackgroundColor(address_set, Color(169, 169, 169))
    print("{} addresses were highlighted!".format(valid_addresses))

log_file_path = askFile("Select Emulation Log File", "Open").getAbsolutePath()

instruction_addresses = parse_emulation_log(log_file_path)

highlight_addresses(instruction_addresses)

print("Script completed successfully!")
