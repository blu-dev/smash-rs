#!/bin/python3.9
from io import TextIOWrapper
import sys

def usage():
    print('const_to_typed.py <filepath> <-e | -b> <prefix> <repr type> <out file> [-p name]')
    sys.exit(2)

def snake_to_pascal(string: str):
    lower = string.lower()
    pascal = lower[0].capitalize()
    was_prev_underscore = False
    for ch in lower[1:]:
        if ch == '_':
            was_prev_underscore = True
            continue
        if was_prev_underscore:
            pascal += ch.capitalize()
        else:
            pascal += ch
        was_prev_underscore = False
    return pascal

if len(sys.argv) != 6:
    usage()

def make_enum(type_name: str, repr_type: str, const_values, file: TextIOWrapper):
    file.write("#[repr(" + repr_type + ")]\n")
    file.write("pub enum " + type_name + " {\n")
    for name in const_values:
        file.write("    " + name[0] + " = 0x" + name[1] + ",\n")
    file.write("}\n")

def make_bitflags(type_name: str, repr_type: str, const_values, file: TextIOWrapper):
    file.write("bitflags! {\n")
    file.write("    #[repr(C)]\n")
    file.write("    #[allow(non_upper_case_globals)]\n")
    file.write("    struct " + type_name + ": " + repr_type + " {\n")
    for name in const_values:
        file.write("        const " + name[0] + " = 0x" + name[1] + ";\n")
    file.write("    }\n}\n")

repr_type = sys.argv[4]
if repr_type != "i8" and repr_type != "u8" and repr_type != "i16" and repr_type != "u16" and repr_type != "i32" and repr_type != "u32" and repr_type != "i64" and repr_type != "u64":
    usage()

file = open(sys.argv[1], 'r')
lines = file.readlines()

const_values = []

for line in lines:
    split = line.split(":")
    const_name = split[2]
    if const_name.startswith(sys.argv[3]):
        trimmed_const_name = const_name.strip().removeprefix(sys.argv[3] + "_")
        if trimmed_const_name != "TERM":
            const_values.append([snake_to_pascal(trimmed_const_name), split[1]])

type_name = snake_to_pascal(sys.argv[3].strip().removesuffix("_").removeprefix("_"))
file.close()

file = open(sys.argv[5], 'w')
if sys.argv[2] == '-e':
    make_enum(type_name, repr_type, const_values, file)
elif sys.argv[2] == '-b':
    make_bitflags(type_name, repr_type, const_values, file)
else:
    usage()
file.close()