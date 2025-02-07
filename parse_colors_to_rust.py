f = open("./colors.yaml")
contents = f.read()

contents = contents.split("\n")

flagnames:str = []
namealias:str = []
flagcolors:str = []

i:int = 0
while i < len(contents):
    if i+1 < len(contents) and "color" in contents[i+1]:
        flagnames.append(contents[i].strip(":"))
        tempcolors:str = []
        while "-" in contents[i+2]:
            tempcolors.append(contents[i+2].strip(" -\""))
            i+=1
        flagcolors.append(tempcolors)

        if "alias" in contents[i+2]:
            tempalias:str = []
            while "-" in contents[i+3]:
                tempalias.append(contents[i+3].strip(" -"))
                i+=1
            namealias.append(tempalias)
        else:
            namealias.append([])

    i+=1

def hex2rgb(h:str):
    return (int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16))

def spaces(i:int):
    for j in range(i):
        print(" ", end="")

"""
for i in range(len(flagnames)):
    print(flagnames[i], end=" ")
    for h in namealias[i]:
        print(f"({h})", end=" ")
    for h in flagcolors[i]:
        print(hex2rgb(h), end= " ")
    print()
"""

#create static flag color arrays
for i in range(len(flagnames)):
    print(f"pub static {flagnames[i].upper()}: &[(u8, u8, u8)] = &[", end="")
    for h in range(len(flagcolors[i])):
        if h != 0:
            spaces(len(f"pub static {flagnames[i].upper()}: &[(u8, u8, u8)] = &["))
        print(f"{hex2rgb(flagcolors[i][h])},")

    spaces(len(f"pub static {flagnames[i].upper()}: &[(u8, u8, u8)] = &["))
    print("];\n")

print("pub static NONE:   &[(u8, u8, u8)] = &[(  0,   0,   0)];\n")

#create color getter function
print("pub fn get_flag(flag_name: &str) -> &'static [(u8, u8, u8)] {")
for i in range(len(flagnames)):
    if i == 0:
        print(f"    if flag_name == \"{flagnames[i]}\"", end="")
        for h in namealias[i]:
            print(f" || flag_name == \"{h}\"", end="")
        print("\n    {", f"return {flagnames[i].upper()};", "}")
        continue

    print(f"    else if flag_name == \"{flagnames[i]}\"", end="")
    for h in namealias[i]:
            print(f" || flag_name == \"{h}\"", end="")
    print("\n    {", f"return {flagnames[i].upper()};", "}")
    
print("    else\n    { return NONE; }")
print("}")
