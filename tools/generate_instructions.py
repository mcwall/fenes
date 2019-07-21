import sys
import os
import re
from os.path import basename

addr_mode_format = 'Some( AddressMode::{addr_mode} )'
addr_mode_empty = 'None'
output_format = '0x{opcode} => Instruction {{ op: Op::{instruction}, address_mode: {addr_mode} }},'



def get_output(opcode, instruction, addr_mode):
    if addr_mode:
        formatted_addr_mode = addr_mode_format.format(addr_mode=addr_mode)
    else:
        formatted_addr_mode = addr_mode_empty
    
    return output_format.format(opcode=opcode, instruction=instruction, addr_mode=formatted_addr_mode)

# Parse a txt file with opcodes and addr modes, and output the opcodes as a generated structure
def generate_ops(fn_in, fn_out):
    file_in = open(fn_in, 'r')
    out_lines = []
    line_num = 0

    for line in file_in:
        line_num += 1
        s = re.sub("[^a-zA-Z0-9-]", "", line.strip())
        if (len(s) < 3):
            continue
        
        output = get_output(s[:2], s[2:5], s[5:])
        out_lines.append(output)

    with open(fn_out, 'w') as file_out:
        for item in out_lines:
            file_out.write("%s\n" % item)


filename_in = sys.argv[1]
if len(sys.argv) > 2:
    filename_out = sys.argv[2]
else:
    filename_out = os.path.splitext(filename_in)[0] + ".gen"

generate_ops(filename_in, filename_out)
