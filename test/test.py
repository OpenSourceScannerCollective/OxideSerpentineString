import string_extract
from termcolor import colored, cprint

data = ""
with open("./test/test.js") as f:
    data = f.read()

results = string_extract.parse(data, "JavaScript")

for res in results:
    print(colored("kind: ", "dark_grey"), end="")
    if res.kind == "string":
        print(colored("< " + res.kind + " >", "cyan"))
    else:
        print(colored("< " + res.kind + " >", "magenta"))
    print(colored("\tvalue:", "dark_grey"), colored(res.value,"blue"))
    print(colored("\traw:", "dark_grey"), colored(res.raw, "red"))
    if len(res.matches) > 0:
        print(colored("\tmatches (", "dark_grey") + colored(str(len(res.matches)), "light_grey") + colored("):", "dark_grey"))
        for key, value in res.matches.items():
            print(colored("\t\t[" + key + "]", "green") + colored(" => ", "dark_grey") + colored( "{" + value + "}", "yellow"))