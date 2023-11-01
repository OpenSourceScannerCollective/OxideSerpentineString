import string_extract
from termcolor import colored, cprint


def test_parser():

    print(colored("# TEST PARSER #", "red", "on_cyan"))

    with open("./test/test.js") as f:
        data = f.read()

    results = string_extract.parse(data, "JavaScript")

    for res in results:
        print(colored("kind: ", "dark_grey"), end="")
        if res.kind == "string":
            print(colored("< " + res.kind + " >", "cyan"))
        else:
            print(colored("< " + res.kind + " >", "magenta"))
        print(colored("\tvalue:", "dark_grey"), colored(res.value, "blue"))
        print(colored("\traw:", "dark_grey"), colored(res.raw, "red"))
        print(colored("\tloc:", "dark_grey"), colored(res.line.start, "light_grey"))
        if len(res.matches) > 0:
            print_matches(res.matches)


def print_matches(matches):
    print(colored("\tmatches (", "dark_grey") + colored(str(len(matches)), "light_grey") + colored(")" + (":" if len(matches) > 0 else ""),
                                                                                                                              "dark_grey"))
    for key, value in matches.items():
        print(colored("\t\t[" + key + "]", "green") + colored(" => ", "dark_grey") + colored("{" + value + "}",
                                                                                             "yellow"))


def test_regex():
    print(colored("# TEST REGEX #", "red", "on_cyan"))
    test_str = """
        // comment with key -----BEGIN RSA PRIVATE KEY-----
        
        var mystr = "variable with key -----BEGIN EC PRIVATE KEY----- "
    """
    matches = string_extract.do_regex(test_str);
    print_matches(matches)


# begin
test_parser()
test_regex()