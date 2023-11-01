import string_extract

data = ""
with open("./test/test.js") as f:
    data = f.read()

results = string_extract.parse(data, "JavaScript")

for res in results:
    print("kind:", res.kind, "\n\tvalue:" , res.value, "\n\traw:", res.raw)
    if len(res.matches) > 0:
        print("\tmatches (" + str(len(res.matches)) + "):")
        for key, value in res.matches.items():
            print("\t\t[" + key + "] => (" + value + ")")