#!/bin/python3.9
import sys
from typing import List

def arg_count(sig: str):
    start_index = sig.index("(") + 1
    end_index = sig.index(")")
    arg_list = sig[start_index:end_index]
    return arg_list.count(": ")

def get_name(sig: str):
    start_index = sig.index("fn ") + 3
    end_index = sig.index("(")
    return sig[start_index:end_index]

if len(sys.argv) != 3:
    print("./import_to_call.py <infile> <outfile>")
    sys.exit(2)

in_file = open(sys.argv[1], 'r')
out_file = open(sys.argv[2], 'w')

lines = in_file.readlines()
in_file.close()
filtered_lines: List[str] = []
for line in lines:
    line = line.strip()
    if not line.startswith("fn"):
        continue
    filtered_lines.append(line)

for line in filtered_lines:
    param_count = arg_count(line)
    func_name = get_name(line)
    func_call = "pub fn" + line.removeprefix("fn")
    func_call = func_call.removesuffix(";") + " {"
    returns_vec = func_call.endswith("-> cpp::simd::Vector2 {") or func_call.endswith("-> cpp::simd::Vector3 {") or func_call.endswith("-> cpp::simd::Vector4 {")
    func_call = func_call.replace("-> cpp::simd::Vector", "-> phx::Vec").replace("*const ", "&").replace("*mut", "&mut").replace("&mut lua_State", "*mut lua_State")
    func_call += "\n    unsafe {\n        impl_::" + func_name + "("
    current_count = 0
    while current_count < param_count:
        func_call += "arg" + str((current_count + 1))
        if current_count + 1 != param_count:
            func_call += ", "
        current_count += 1
    func_call += ")"
    if returns_vec:
        func_call += ".into()"
    func_call += "\n    }\n}"
    out_file.write(func_call + "\n\n")