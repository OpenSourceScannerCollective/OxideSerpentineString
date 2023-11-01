import string_extract

data = ""
with open("./test/test.js") as f:
    data = f.read()

results = string_extract.parse(data, "JavaScript")

for res in results:
    print("kind:", res.kind, "\n\tmatches:", len(res.matches) , "\n\tvalue:" , res.value, "\n\traw:", res.raw)
    for key, value in res.matches.items():
        print("\t\t> (", key, ") =>", value)