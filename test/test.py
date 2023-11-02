import string_extract
from termcolor import colored, cprint


def test_parser(lang):

    if lang.lower() == "javascript":
        filepath = "./test/language/javascript/test.js"
    elif lang.lower() == "python":
        filepath = "./test/language/python/test.py"
    elif lang.lower() == "json":
        filepath = "./test/language/json/test.json"
    elif lang.lower() == "toml":
        filepath = "./test/language/toml/test.toml"
    else:
        print(colored("Invalid parser language: <" + lang + ">", "red", "on_black"))
        return

    with open(filepath) as f:
        data = f.read()

    print(colored(" # TEST PARSER: ", "white", "on_red") +
          colored(lang.upper(), "blue", "on_red") +
          colored(" # ", "white", "on_red"))

    results = string_extract.parse(data, lang)

    for res in results:
        print(colored("kind: ", "yellow"), end="")
        if res.kind == "string":
            print(colored("<" + res.kind + ">", "cyan"))
        else:
            print(colored("<" + res.kind + ">", "magenta"))
        print(colored("\tvalue:", "dark_grey"), colored(res.value, "blue"))
        print(colored("\traw:", "dark_grey"), colored(res.raw, "red"))
        print(colored("\tloc:", "dark_grey"), colored(res.line.start, "light_grey"))
        if len(res.matches) > 0:
            print_matches(res.matches)


def print_matches(matches):
    print(colored("\tmatches (", "dark_grey") + colored(str(len(matches)), "light_grey") + colored(
        ")" + (":" if len(matches) > 0 else ""),
        "dark_grey"))
    for key, value in matches.items():
        print(colored("\t\t[" + key + "]", "green") + colored(" => ", "dark_grey"), value)


def test_regex(lang):

    if lang.lower() == "javascript":
        filepath = "./test/patterns/javascript/test.js"
    else:
        print(colored("Invalid parser language: <" + lang + ">", "red", "on_black"))
        return

    with open(filepath) as f:
        data = f.read()

    print(colored(" # TEST REGEX: ", "white", "on_red") +
          colored(lang.upper(), "blue", "on_red") +
          colored(" # ", "white", "on_red"))

    with open(filepath) as f:
        data = f.read()

    print_matches(string_extract.do_regex(data))


def lang_tests():
    test_langs = [
        "JavaScript",
        "Python",
        "Json",
        "Toml"
    ]

    for lang in test_langs:
        test_parser(lang)
        test_regex(lang)


# begin
lang_tests()
