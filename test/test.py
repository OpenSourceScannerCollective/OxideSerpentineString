import string_extract

data = ""
with open("./test/test.js") as f:
    data = f.read()

results = string_extract.parse(data, "JavaScript")

for res in results:
    print("kind:", res.kind, "matches:", len(res.matches) , "value:" , res.value)
    for key, value in res.matches.items():
        print("\t>", key, "=>", value)