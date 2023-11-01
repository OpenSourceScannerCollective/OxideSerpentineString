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

    print(colored(" # TEST PARSER: ", "white", "on_red") +
          colored(lang.upper(), "blue", "on_red") +
          colored(" # ", "white", "on_red"))

    with open(filepath) as f:
        data = f.read()

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
        print(colored("\t\t[" + key + "]", "green") + colored(" => ", "dark_grey") + colored("{" + value + "}",
                                                                                             "light_green"))


def test_regex():
    print(colored(" # TEST REGEX # ", "white", "on_red"))
    test_str = """
        // comment with key -----BEGIN RSA PRIVATE KEY-----
        
        var mystr = "variable with key -----BEGIN EC PRIVATE KEY----- "
    """
    matches = string_extract.do_regex(test_str);
    print_matches(matches)


def lang_tests():
    test_langs = [
        "JavaScript",
        "Python",
        "Json",
        "Toml"
    ]

    for lang in test_langs:
        test_parser(lang)


# begin
test_regex()
lang_tests()
