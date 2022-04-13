import sys
from dataclasses import dataclass


@dataclass
class ConstEntry:
    value: int
    name: str

    def __init__(self, line: str):
        split = line.strip().split(":")
        self.value = int(split[1], 16)
        self.name = split[2]

@dataclass
class WorkId:
    ty: str
    kind: str
    offset: int
    name: str

    def __init__(self, entry: ConstEntry):
        ty = entry.value >> 0x1C
        kind = (entry.value >> 0x18) & 0xF

        if kind == 0xE:
            if ty == 0x1:
                self.ty = "Int"
            else:
                self.ty = "None"
        
        if kind == 0xE:
            self.kind = "Transition"
        else:
            self.kind = "None"
        
        self.offset = entry.value & 0xFFFFFF

        self.name = entry.name
        
        
@dataclass
class StatusWorkId:
    internal: WorkId
    object_class: str
    object_name: str
    status_kind: str
    const_name: str

    def get_namespace(self) -> str:
        namespace = self.object_class.lower()
        if self.object_name != None:
            namespace += "::" + self.object_name.lower()
        if self.status_kind != None:
            namespace += "::" + self.status_kind.lower()
        namespace += "::" + self.const_name
        return namespace

    def __init__(self, internal: WorkId):
        self.internal = internal
        start, remainder = internal.name.split("_", maxsplit=1)
        
        if start == "FIGHTER":
            self.object_class = "fighter"
        elif start == "WEAPON":
            self.object_class = "weapon"
        elif start == "ITEM":
            self.object_class = "item"
        else:
            self.object_class = None
            return
        
        if remainder.startswith("STATUS"):
            self.object_name = None
        else:
            if "_STATUS_" not in remainder:
                self.object_class = None
                return
            else:
                self.object_name, remainder = remainder.split("_STATUS_", maxsplit=1)
        
        if remainder.startswith("WORK"):
            self.status_kind = None
            self.const_name = remainder.removeprefix("WORK_")
        elif "_WORK_" not in remainder:
            if "_FLOAT_" in remainder:
                self.status_kind, self.const_name = remainder.split("_FLOAT_", maxsplit=1)
            elif "_INT_" in remainder:
                self.status_kind, self.const_name = remainder.split("_INT_", maxsplit=1)
            elif "_FLAG_" in remainder:
                self.status_kind, self.const_name = remainder.split("_FLAG_", maxsplit=1)
            else:
                self.object_class = None
        else:
            self.object_class = None
        
                
def namespaceify_constant(const: str):
    if const.startswith("FIGHTER") or const.startswith("FT"):
        const = const.removeprefix("FIGHTER_").removeprefix("FT_")
        namespace = "fighter::"
    elif const.startswith("WEAPON") or const.startswith("WN"):
        const = const.removeprefix("WEAPON_").removeprefix("WN_")
        namespace = "weapon::"
    elif const.startswith("ITEM"):
        const = const.removeprefix("ITEM_")
        namespace = "item::"
    elif const.startswith("ENEMY"):
        const = const.removeprefix("ENEMY_")
        namespace = "ENEMY::"
    else:
        return None

    words: list[str] = []
    while True:
        current, const = const.split("_", maxsplit=1)
        if current == "INSTANCE":
            break
        else:
            words.append(current.lower())
    for word in words:
        namespace += word + "_"
    namespace = namespace.removesuffix("_") + "::instance::"
    
    found_work = False
    words: list[str] = []
    while True:
        if not "_" in const:
            break
        current, const = const.split("_", maxsplit=1)
        if current == "WORK":
            found_work = True
        if current == "FLAG" or current == "INT" or current == "FLOAT":
            break
        elif found_work and current != "WORK" and current != "ID":
            const = current + "_" + const
            break
        elif not found_work:
            words.append(current.lower())

    for word in words:
        namespace += word + "_"
    namespace = namespace.removesuffix("_") + "::"

    namespace += const
    return namespace.replace("::::", "::")      

def namespaceify(snake_case: str):
    words = snake_case.split("_")
    output = ""
    for word in words:
        output += word.lower() + "::"
    return output.removesuffix("::")

def get_type_and_name(const_name: str):
    split = const_name.split("_", maxsplit= 1)
    return [split[0].capitalize(), split[1]]

def split_work_id(const_name: str):
    if "_WORK_ID_" in const_name:
        split = const_name.split("_WORK_ID_")
        ty, name = get_type_and_name(split[1])
        namespace = namespaceify(split[0])
        return [namespace, ty, name]
    else:
        split = const_name.split("_WORK_")
        ty, name = get_type_and_name(split[1])
        namespace = namespaceify(split[0])
        return [namespace, ty, name]

def find_group_namespace(const_name: str):
    return namespaceify(const_name.split("_TRANSITION_GROUP_")[0])

def find_transition_namespace(const_name: str):
    return namespaceify(const_name.split("_TRANSITION_TERM_ID_")[0])

def find_work_namespace(const_name: str):
    split = const_name.split("_WORK_ID_")
    namespace = namespaceify(split[0])
    return namespace

def find_namespace(const_name: str):
    if "TRANSITION_GROUP" in const_name:
        return find_group_namespace(const_name)
    elif "TRANSITION_TERM" in const_name:
        return find_transition_namespace(const_name)
    elif "WORK_ID" in const_name:
        return find_work_namespace(const_name)
    else:
        return ""

path = sys.argv[1]

lines = open(path).readlines()

for line in lines:
    entry = ConstEntry(line)
    work_id = WorkId(entry)
    if work_id.kind == "Transition" and work_id.ty == "Int":
        print(hex(entry.value), entry.name)

# successes: list[StatusWorkId] = []

# for line in lines:
#     entry = ConstEntry(line)
#     work_id = WorkId(entry)
#     v = namespaceify_constant(work_id.name)
#     if v == None:
#         print("Failed:", work_id.name)
#     else:
#         print(hex(entry.value), v)
    # status_id = StatusWorkId(work_id)
    # if status_id.object_class == None:
    #     print("Failed:", work_id.name)
    # else:
    #     successes.append(status_id)

# print("Successes:")
# for success in successes:
#     print(success.get_namespace())
# print(find_namespace("FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START"));  