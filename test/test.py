import os
import re

import oxide_serpentine_string
from termcolor import colored, cprint

def parser(lang, verbose):
    if lang.lower() == "javascript":
        filepath = "./test/language/javascript/test.js"
    elif lang.lower() == "python":
        filepath = "./test/language/python/test.py"
    elif lang.lower() == "json":
        filepath = "./test/language/json/test.json"
    elif lang.lower() == "toml":
        filepath = "./test/language/toml/test.toml"
    elif lang.lower() == "csv":
        filepath = "./test/language/csv/test.csv"
    else:
        if verbose:
            print(colored(" Invalid parser language: <" + lang.lower() + "> ", "red", "on_black"))
        return

    with open(filepath) as f:
        data = f.read()

    if verbose:
        print(colored(" # TEST PARSER: ", "white", "on_red") +
              colored(lang.upper(), "blue", "on_red") +
              colored(" # ", "white", "on_red"))
    try:
        results = oxide_serpentine_string.parse_with_lang_str(data, lang)
    except:
        print("Unable to parse input for: " + lang)
        results = []

    if verbose:
        for index, res in enumerate(results):
            print(colored("\nMatch [", "dark_grey") + colored(str(index), "light_grey"), end="")
            print(colored("] of (" + str(len(results)) + ")", "dark_grey"))

            if res.kind == oxide_serpentine_string.ParseMatchType.StringLiteral:
                kind = "string"
            elif res.kind == oxide_serpentine_string.ParseMatchType.Comment:
                kind = "comment"
            else:
                kind = "unknown"
            print(colored("\tkind: ", "dark_grey") + colored("<" + kind + ">", "light_grey"))

            res_value = res.value if len(res.value) < 100 else res.value[:97] + "..."
            print(colored("\tvalue:", "dark_grey"), colored(res_value, "light_grey"))
            print(colored("\traw:", "dark_grey"), colored(res.raw, "light_grey"))
            print(colored("\tposition =>", "dark_grey"))
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
        print(colored("\t\t  kind: ", "dark_grey") + colored(RegexMatchCollection.kind, "green"))

        # get the first non-empty line
        source = ""
        for line in RegexMatchCollection.source.splitlines():
            if line.strip() != "":
                source = line if len(RegexMatchCollection.source) <= 100 else line[:97] + "..."
                break
        source = source if len(source) <= 100 else source[:97] + "..."
        print(colored("\t\t  source: ", "dark_grey") + colored(source, "light_grey"))
        print(colored("\t\t  matches (", "dark_grey") + colored(str(len(RegexMatchCollection.matches)),
                                                                "light_grey") + colored(
            ")" + (":" if len(RegexMatchCollection.matches) > 0 else ""),
            "dark_grey"))
        for index, RegexMatch in enumerate(RegexMatchCollection.matches):
            print(colored("\t\t\t[", "dark_grey") + colored(str(index), "light_grey") + colored("]", "dark_grey"))
            print(colored("\t\t\t  value: ", "dark_grey") + colored(RegexMatch.value, "light_grey"))
            print(colored("\t\t\t  position => ", "dark_grey"))
            print(
                colored("\t\t\t\t  char.start: ", "dark_grey") + colored(RegexMatch.position.char.start, "light_grey"))
            print(colored("\t\t\t\t  char.end: ", "dark_grey") + colored(RegexMatch.position.char.end, "light_grey"))
            print(
                colored("\t\t\t\t  line.start: ", "dark_grey") + colored(RegexMatch.position.line.start, "light_grey"))
            print(colored("\t\t\t\t  line.end: ", "dark_grey") + colored(RegexMatch.position.line.end, "light_grey"))
            print(colored("\t\t\t  source_pos =>", "dark_grey"))
            print(colored("\t\t\t\t  char.start: ", "dark_grey") + colored(RegexMatch.source_pos.char.start,
                                                                           "light_grey"))
            print(colored("\t\t\t\t  char.end: ", "dark_grey") + colored(RegexMatch.source_pos.char.end, "light_grey"))
            print(colored("\t\t\t\t  line.start: ", "dark_grey") + colored(RegexMatch.source_pos.line.start,
                                                                           "light_grey"))
            print(colored("\t\t\t\t  line.end: ", "dark_grey") + colored(RegexMatch.source_pos.line.end, "light_grey"))


def regex(pattern, filepath, verbose):

    try:
        with open(filepath) as f:
            data = f.read()
    except IOError:
        print(colored(" Invalid regex pattern: <" + pattern + "> ", "red", "on_black"))
        return

    file_ext = os.path.splitext(filepath)[1][1:]

    if verbose:
        print(colored(" # TEST REGEX: ", "white", "on_red") +
              colored(pattern.upper(), "blue", "on_red") +
              colored(" (", "light_grey", "on_red") +
              colored(file_ext.upper(), "cyan", "on_red") +
              colored(")", "light_grey", "on_red") +
              colored(" # ", "white", "on_red"))

        results = oxide_serpentine_string.do_regex(data)

        if len(results) < 1:
            print("Matches: 0")
        else:
            for RegexMatchCollection in results:
                print("Matches: ", str(len(RegexMatchCollection.matches)))


def detect_lang(lang, verbose):
    if lang.lower() == "javascript":
        ext = "js"
    elif lang.lower() == "python":
        ext = "py"
    elif lang.lower() == "json":
        ext = "json"
    elif lang.lower() == "toml":
        ext = "toml"
    elif lang.lower() == "csv":
        ext = "csv"
    else:
        if verbose:
            print(colored(" Invalid parser language: <" + lang.lower() + "> ", "red", "on_black"))
        return

    if verbose:
        print(colored(" # TEST LANG DETECT: ", "white", "on_red") +
              colored(lang.upper(), "blue", "on_red") +
              colored(" # ", "white", "on_red"))

    filepath = "./test/language/" + lang.lower() + "/test." + ext

    with open(filepath) as f:
        data = f.read()

    for index, path in enumerate([filepath, ""]):
        detected_lang = oxide_serpentine_string.detect_language(data, path)
        path = "\"" + path + "\""
        print(colored("With filepath: ", "dark_grey") + colored(path, "light_grey"))
        print(colored("\tDetected: ", "dark_grey") + colored(detected_lang, "light_grey"))


def test_detect_lang(verbose):

    test_langs = [
        "JavaScript",
        "Python",
        "Json",
        "Toml",
        "Csv"
    ]

    for lang in test_langs:
        detect_lang(lang, verbose)


def lang_parser(verbose):

    test_langs = [
        "JavaScript",
        "Python",
        "Json",
        "Toml",
        "Csv"
    ]

    for lang in test_langs:
        parser(lang, verbose)


def lang_regex(verbose):

    test_patterns = [
        {"name": "GOOGLE_API_KEY",
         "pattern": re.compile(r".+/Google_API_Key\.[a-z]+$")},
        {"name": "GOOGLE_SERVICE_ACCOUNT",
         "pattern": re.compile(r".+/Google_GCP_Service_account\.[a-z]+$")},
        {"name": "GOOGLE_OAUTH_TOKEN",
         "pattern": re.compile(r".+/Google_OAuth_Access_Token\.[a-z]+$")},
        {"name": "SSH_DSA_PRIVATE_KEY",
         "pattern": re.compile(r".+/PEM_DSA_(1024|2048|4096)\.[a-z]+$")},
        {"name": "EC_PRIVATE_KEY",
         "pattern": re.compile(r".+/PEM_EC_(256|384|521)\.[a-z]+$")},
        {"name": "PGP_PRIVATE_KEY",
         "pattern": re.compile(r".+/PEM_PGP\.[a-z]+$")},
        {"name": "RSA_PRIVATE_KEY",
         "pattern": re.compile(r".+/PEM_RSA_(512|1024|2048|3072|4096)\.[a-z]+$")}
    ]

    for test_path in test_data:
        for file_path in test_path['paths']:
            for file_pattern in test_patterns:
                match = file_pattern["pattern"].match(file_path)
                if match:
                    regex(file_pattern['name'], file_path, verbose)


def get_directories_with_files(path, root_path, valid_extensions=None):
    directories = []
    # Normalize the valid extensions to ensure consistency
    if valid_extensions is not None:
        valid_extensions = [ext.lower() for ext in valid_extensions]

    # Strip the root path from the current path
    relative_path = os.path.relpath(path, root_path)

    # If we're at the root directory, relative path would be ".", so we keep it an empty string
    if relative_path == ".":
        relative_path = ""

    # List to store file paths for the current directory
    file_paths = []

    # Loop through the contents of the directory
    for entry in os.scandir(path):
        if entry.is_dir(follow_symlinks=False):
            # Recursively get directories and files from the current directory
            directories.extend(get_directories_with_files(entry.path, root_path, valid_extensions))
        else:
            # Check if the file has a valid extension before adding it to the list
            if valid_extensions is None or os.path.splitext(entry.name)[1].lower() in valid_extensions:
                file_paths.append(entry.path)

    # Only add the directory object if it contains files
    if file_paths:
        directories.append({
            "name": relative_path.replace("\\", "/"),  # Normalize to use forward slashes
            "paths": file_paths
        })

    return directories


def load_test_data():
    root_path_to_scan = './test/data'
    valid_exts = ['.json', '.xml', '.yaml', '.txt']
    global test_data
    test_data = get_directories_with_files(root_path_to_scan, root_path_to_scan, valid_exts)


# begin
load_test_data()
test_detect_lang(True)
lang_parser(True)
lang_regex(True)
