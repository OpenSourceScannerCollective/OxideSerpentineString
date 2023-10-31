import string_extract

data = ""
with open("./test/test.js") as f:
    data = f.read()

string_extract.parse(data, "JavaScript")