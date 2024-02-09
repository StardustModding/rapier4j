import os
import json

with open("./rapier3d.json", "r") as f:
    data = f.read()

data = json.loads(data)
index = data["index"]
paths = data["paths"]
keys = list(index.keys())

def is_static(decl):
    inps = decl["inputs"]
    has_self = inps[0][0] == "self" if len(inps) > 0 else False

    return not has_self

def is_mut(decl):
    inps = decl["inputs"]

    has_self = inps[0][0] == "self" if len(inps) > 0 else False
    has_borrowed_self = "borrowed_ref" in inps[0][1] if has_self and len(inps) > 1 else False
    has_mut_self = inps[0][1]["borrowed_ref"]["mutable"] if has_borrowed_self else False

    return has_mut_self

def handle_type(obj):
    # can be resolved_path or generic
    out = None

    if "resolved_path" in obj:
        rp = obj["resolved_path"]
        out = rp["name"]
        vals = []

        for arg in rp["args"]["angle_bracketed"]["args"]:
            if "type" in arg and "generic" in arg["type"]:
                vals.append(arg["type"]["generic"])

        vals = "<" + ", ".join(vals) + ">" if len(vals) > 0 else ""
        out += vals
    elif "generic" in obj:
        out = obj["generic"]
    elif "borrowed_ref" in obj:
        mut = obj["borrowed_ref"]["mutable"]
        mut = "&mut " if mut else "&"
        ty = obj["borrowed_ref"]["type"]
        out = mut + handle_type(ty)
    elif "dyn_trait" in obj:
        traits = []

        for trait in obj["dyn_trait"]["traits"]:
            traits.append(handle_type(trait))
        
        out = "dyn " + " + ".join(traits)
    elif "trait" in obj:
        trait = obj["trait"]
        out = trait["name"]
        vals = []

        for arg in trait["args"]["angle_bracketed"]["args"]:
            if "type" in arg and "generic" in arg["type"]:
                vals.append(arg["type"]["generic"])

        vals = "<" + ", ".join(vals) + ">" if len(vals) > 0 else ""
        out += vals
    elif "qualified_path" in obj:
        qp = obj["qualified_path"]
        out = handle_type(qp["self_type"]) + "::"
        out = qp["name"]
        vals = []

        for arg in qp["args"]["angle_bracketed"]["args"]:
            if "type" in arg and "generic" in arg["type"]:
                vals.append(arg["type"]["generic"])

        vals = "<" + ", ".join(vals) + ">" if len(vals) > 0 else ""
        out += vals
    elif "primitive" in obj:
        out = obj["primitive"]
    
    return out

funcs = {}

for key in keys:
    item = index[key]
    crate = item["crate_id"]

    if crate != 0:
        continue

    path_id = item["id"].split("-")[-1]
    
    if not path_id in paths:
        continue

    path = paths[path_id]
    kind = path["kind"]
    crate = path["crate_id"]

    if crate != 0:
        continue

    if kind != "struct":
        continue

    if not "struct" in item["inner"]:
        continue

    name = "::".join(path["path"])
    struct = path["path"][-1]
    data1 = json.dumps(path, indent=4)
    data = json.dumps(item, indent=4)

    # print(f"{name} = {data1}\n-> {data}")

    for impl in item["inner"]["struct"]["impls"]:
        val = index[impl]
        items = val["inner"]["impl"]["items"]
        
        for item in items:
            val = index[item]

            if not "function" in val["inner"]:
                continue

            data = val["inner"]["function"]["decl"]
            name = val["name"]
            out = ""
            inp = []
            start_idx = 0 if is_static(data) else 1

            for item in data["inputs"][start_idx:]:
                arg_name = item[0]
                arg_type = handle_type(item[1])

                inp.append(f"{arg_name}: {arg_type}")

            static = "static " if is_static(data) else ""
            mut = "mut " if is_mut(data) else ""
            prefix = f"{static}{mut}"
            inp = ", ".join(inp)

            if "output" in data and data["output"] != None:
                tmp = handle_type(data["output"])

                if tmp != None:
                    out = " -> " + tmp

            data = f"{prefix}fn {name}({inp}){out};" \
                .replace("$crate::", "crate::") \
                .replace("crate::", "rapier3d::")

            if not struct in funcs:
                funcs[struct] = []

            funcs[struct].append(data)

for i, (k, v) in enumerate(funcs.items()):
    code = f"class {k} {{\n"

    for item in v:
        code += f"    {item}\n"
    
    code += "};"

    print(code)
