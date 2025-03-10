import reversejp

props = reversejp.find_properties(139.7670, 35.6812)

for prop in props:
    print(prop.code, prop.name, prop.en_name)
