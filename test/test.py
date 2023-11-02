import oxide_serpentine_string
from termcolor import colored, cprint


def test_parser(lang, verbose):

    if lang.lower() == "javascript":
        filepath = "./test/language/javascript/test.js"
    elif lang.lower() == "python":
        filepath = "./test/language/python/test.py"
    elif lang.lower() == "json":
        filepath = "./test/language/json/test.json"
    elif lang.lower() == "toml":
        filepath = "./test/language/toml/test.toml"
    else:
        if verbose:
            print(colored("Invalid parser language: <" + lang + ">", "red", "on_black"))
        return

    with open(filepath) as f:
        data = f.read()

    if verbose:
        print(colored(" # TEST PARSER: ", "white", "on_red") +
              colored(lang.upper(), "blue", "on_red") +
              colored(" # ", "white", "on_red"))

    results = oxide_serpentine_string.parse(data, lang)

    if verbose:
        for res in results:
            print(colored("kind: ", "yellow"), end="")
            if res.kind == "string":
                print(colored("<" + res.kind + ">", "cyan"))
            else:
                print(colored("<" + res.kind + ">", "magenta"))

            res_value = res.value if len(res.value) < 100 else res.value[:97] + "..."
            print(colored("\tvalue:", "dark_grey"), colored(res_value, "blue"))
            print(colored("\traw:", "dark_grey"), colored(res.raw, "red"))
            print(colored("\tsource_pos =>", "dark_grey"))
            print(colored("\t\t  char.start: ", "dark_grey") + colored(res.position.char.start, "light_grey"))
            print(colored("\t\t  char.end: ", "dark_grey") + colored(res.position.char.end, "light_grey"))
            print(colored("\t\t  line.start: ", "dark_grey") + colored(res.position.line.start, "light_grey"))
            print(colored("\t\t  line.end: ", "dark_grey") + colored(res.position.line.end, "light_grey"))
            if len(res.matches) > 0:
                print_matches(res.matches, verbose)


def print_matches(RegexMatchCollectionArray, verbose):
    if not verbose:
        return

    print(colored("\tmatches (", "dark_grey") + colored(str(len(RegexMatchCollectionArray)), "light_grey") + colored(
        ")" + (":" if len(RegexMatchCollectionArray) > 0 else ""),
        "dark_grey"))
    for RegexMatchCollection in RegexMatchCollectionArray:
        # print(colored("\t\t[" + RegexMatchCollection.kind + "]", "green"))
        print(colored("\t\t  kind: ", "dark_grey") + colored(RegexMatchCollection.kind, "green"))

        # get the first non-empty line
        source = ""
        for line in RegexMatchCollection.source.splitlines():
            if line.strip() != "":
                source = line
                break
        source = source if len(source) < 100 else source[:97] + "..."
        print(colored("\t\t  source: ", "dark_grey") + colored(source, "light_grey"))
        print(colored("\t\t  matches (", "dark_grey") + colored(str(len(RegexMatchCollection.matches)), "light_grey") + colored(
        ")" + (":" if len(RegexMatchCollection.matches) > 0 else ""),
        "dark_grey"))
        for index, RegexMatch in enumerate(RegexMatchCollection.matches):
            print(colored("\t\t\t[", "dark_grey") + colored(str(index),"light_grey") + colored("]", "dark_grey"))
            print(colored("\t\t\t  value: ", "dark_grey") + colored(RegexMatch.value, "light_grey"))
            print(colored("\t\t\t  position => ", "dark_grey"))
            print(colored("\t\t\t\t  char.start: ", "dark_grey") + colored(RegexMatch.position.char.start, "light_grey"))
            print(colored("\t\t\t\t  char.end: ", "dark_grey") + colored(RegexMatch.position.char.end, "light_grey"))
            print(colored("\t\t\t\t  line.start: ", "dark_grey") + colored(RegexMatch.position.line.start, "light_grey"))
            print(colored("\t\t\t\t  line.end: ", "dark_grey") + colored(RegexMatch.position.line.end, "light_grey"))
            print(colored("\t\t\t  source_pos =>", "dark_grey"))
            print(colored("\t\t\t\t  char.start: ", "dark_grey") + colored(RegexMatch.source_pos.char.start, "light_grey"))
            print(colored("\t\t\t\t  char.end: ", "dark_grey") + colored(RegexMatch.source_pos.char.end, "light_grey"))
            print(colored("\t\t\t\t  line.start: ", "dark_grey") + colored(RegexMatch.source_pos.line.start, "light_grey"))
            print(colored("\t\t\t\t  line.end: ", "dark_grey") + colored(RegexMatch.source_pos.line.end, "light_grey"))


def test_regex(lang, verbose):

    if lang.lower() == "javascript":
        filepath = "./test/patterns/javascript/test.js"
    else:
        print(colored("Invalid regex language: <" + lang + ">", "red", "on_black"))
        return

    with open(filepath) as f:
        data = f.read()

    if verbose:
        print(colored(" # TEST REGEX: ", "white", "on_red") +
              colored(lang.upper(), "blue", "on_red") +
              colored(" # ", "white", "on_red"))

        print_matches(oxide_serpentine_string.do_regex(data), verbose)


def lang_tests(verbose):
    test_langs = [
        "JavaScript",
        "Python",
        "Json",
        "Toml"
    ]

    for lang in test_langs:
        test_parser(lang, verbose)
        test_regex(lang, verbose)


# begin
lang_tests(True)
