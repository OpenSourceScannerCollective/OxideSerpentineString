import os
import re
import oxide_serpentine_string
from termcolor import colored, cprint

def parser(filepath, lang, verbose):
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

    print_ParseResults(results, verbose)


def print_ParseResults(results, verbose=True):
    if not verbose:
        return

    for index, res in enumerate(results):
        if ONLY_SHOW_RESULTS and len(res.matches) < 1:
            continue

        print(colored("\nExtract [", "dark_grey") +
              colored(str(index + 1), "light_grey") +
              colored("] of [", "dark_grey") +
              colored(str(len(results)), "light_grey") +
              colored("]", "dark_grey"))

        if res.kind == oxide_serpentine_string.ParseMatchType.StringLiteral:
            kind = colored("<string>", "yellow")
        elif res.kind == oxide_serpentine_string.ParseMatchType.Comment:
            kind = colored("<comment>", "magenta")
        else:
            kind = colored("<unknown>", "red")
        print(colored("\tkind: ", "dark_grey") + kind)
        print(colored("\tvalue:", "dark_grey") +
              colored(trunc_str(res.value), "light_grey"))
        if SHOW_RAW:
            print(colored("\traw:", "dark_grey"), colored(trunc_str(res.raw), "light_grey"))
        print_MatchPos(res.position, "\t", "position", verbose)
        print_RegexMatchCollectionArray(res.matches, verbose)


def trunc_str(input_str, postfix=" (cont)"):
    if not TRUNCATE_VALUES:
        return input_str

    if len(input_str) < MAX_STRING_LENGTH:
        input_str = input_str
    else:
        input_str = input_str[:(MAX_STRING_LENGTH - len(postfix))] + colored(" (cont.)", "dark_grey")
    return input_str


def print_MatchPos(MatchPos, prefix="", head_prefix="position", verbose=True):
    if not SHOW_POS_INFO or not verbose:
        return
    print(colored(prefix + head_prefix + " =>", "dark_grey") + "\n" +
          colored(prefix + "\t  char.start: ", "dark_grey") +
          colored(MatchPos.char.start, "light_grey") + "\n" +
          colored(prefix + "\t  char.end: ", "dark_grey") +
          colored(MatchPos.char.end, "light_grey") + "\n" +
          colored(prefix + "\t  line.start: ", "dark_grey") +
          colored(MatchPos.line.start, "light_grey") + "\n" +
          colored(prefix + "\t  line.end: ", "dark_grey") +
          colored(MatchPos.line.end, "light_grey"))


def print_RegexMatchCollectionArray(RegexMatchCollectionArray, verbose=True):
    if not verbose:
        return

    if ONLY_SHOW_RESULTS and len(RegexMatchCollectionArray) < 1:
        return

    print(colored("\tpattern_matches (", "dark_grey") +
          colored(str(len(RegexMatchCollectionArray)), "light_grey") +
          colored(")" + (":" if len(RegexMatchCollectionArray) > 0 else ""), "dark_grey"))

    if len(RegexMatchCollectionArray) < 1:
        return

    RegexMatchCollectionArray = RegexMatchCollectionArray[:MAX_PRINT_ARRAY_SIZE]

    for RegexMatchCollection in RegexMatchCollectionArray:
        print(colored("\t\tkind: ", "dark_grey") +
              colored("[" + RegexMatchCollection.kind + "]", "green"))

        # get the first non-empty line
        source = ""
        for line in RegexMatchCollection.source.splitlines():
            if line.strip() != "":
                source = line if len(RegexMatchCollection.source) < MAX_STRING_LENGTH else line
                break
        source = trunc_str(source)
        print(colored("\t\tsource: ", "dark_grey") +
              colored(source, "light_grey"))
        if SHOW_RAW:
            print(colored("\t\traw: ", "dark_grey") +
                  colored(trunc_str(RegexMatchCollection.raw), "light_grey"))
        print_RegexMatchArray(RegexMatchCollection.matches, "\t\t")


def print_RegexMatchArray(RegexMatchArray, prefix="", verbose=True):
    if not verbose:
        return

    print(colored(prefix + "matches (", "dark_grey") +
          colored(str(len(RegexMatchArray)), "light_grey") +
          colored(")" + (":" if len(RegexMatchArray) > 0 else ""), "dark_grey"))

    prefix = prefix + "\t"
    RegexMatchArray = RegexMatchArray[:MAX_PRINT_ARRAY_SIZE]

    for index, RegexMatch in enumerate(RegexMatchArray):
        print(colored(prefix + "[", "dark_grey") +
              colored(str(index), "light_grey") +
              colored("]", "dark_grey"))
        print_RegexMatch(RegexMatch, prefix, verbose)


def print_RegexMatch(RegexMatch, prefix="", verbose=True):
    print(colored(prefix + "value: ", "dark_grey") +
          colored(trunc_str(RegexMatch.value), "cyan"))
    print_MatchPos(RegexMatch.position, prefix, "position", verbose)
    print_MatchPos(RegexMatch.source_pos, prefix, "source_pos", verbose)


def do_regex(pattern, filepath, verbose):
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

    if verbose and len(results) < 1:
        print("Matches: 0")
        return

    for RegexMatchCollection in results:
        print_RegexMatchArray(RegexMatchCollection.matches, "", verbose)


def detect_lang(lang, verbose):
    def callback(fpath, fpattern, verb):
        with open(fpath) as f:
            data = f.read()

        for index, path in enumerate([fpath]):  # don't detect language by contents
            # for index, path in enumerate([fpath, ""]):
            detected_lang = oxide_serpentine_string.detect_lang(data, path)
            #     detected_lang = oxide_serpentine_string.detect_lang_file(path)
            path = "\"" + path + "\""

            if not verb:
                continue

            print(colored("With filepath: ", "dark_grey") +
                  colored(path, "light_grey"))
            print(colored("\tDetected: ", "dark_grey") +
                  colored(detected_lang, "light_grey"))

    test_patterns = [
        {"name": "FILE",
         "pattern": re.compile(r".+\.[a-zA-Z0-9]+$")},
    ]

    process_test_files(test_patterns, callback, verbose)


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
    test_patterns = [
        {"name": "CSV",
         "pattern": re.compile(r".+\.csv$")},
        {"name": "JAVASCRIPT",
         "pattern": re.compile(r".+\.js$")},
        {"name": "JSON",
         "pattern": re.compile(r".+\.json$")},
        {"name": "PYTHON",
         "pattern": re.compile(r".+\.py$")},
        {"name": "TOML",
         "pattern": re.compile(r".+\.toml$")},
    ]

    def callback(fpath, fpattern, verb):
        filename = fpath.split("/")[-1]
        if filename.lower() == 'expect.json':
            return
        parser(fpath, fpattern["name"], verbose)

    process_test_files(test_patterns, test_lang_data, callback, verbose)


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

    def callback(fpath, fpattern, verb):
        do_regex(fpattern['name'], fpath, verb)

    process_test_files(test_patterns, test_data, callback, verbose)


def process_test_files(test_patterns, data, callback, verbose):
    for test_path in data:
        for file_path in test_path['paths']:
            for file_pattern in test_patterns:
                if file_pattern["pattern"].match(file_path):
                    callback(file_path, file_pattern, verbose)


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


def set_log_verbosity(verb='DEBUG'):

    global PRINT_VERBOSE
    global MAX_STRING_LENGTH
    global MAX_PRINT_ARRAY_SIZE
    global ONLY_SHOW_RESULTS
    global SHOW_POS_INFO
    global SHOW_RAW
    global TRUNCATE_VALUES

    if os.environ.get('PRINT_VERB_LEVEL'):
        verb = os.environ.get('PRINT_VERB_LEVEL')

    verb = verb.upper()

    if verb == 'DEBUG' or verb == 'DEFAULT' or verb == '':
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 1000
        MAX_PRINT_ARRAY_SIZE = 1000
        ONLY_SHOW_RESULTS = False
        SHOW_POS_INFO = True
        SHOW_RAW = True
        TRUNCATE_VALUES = True
    elif verb == 'DEBUG_EX':
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 200
        MAX_PRINT_ARRAY_SIZE = 100
        ONLY_SHOW_RESULTS = True
        SHOW_POS_INFO = True
        SHOW_RAW = False
        TRUNCATE_VALUES = True
    elif verb == 'RESULTS':
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 200
        MAX_PRINT_ARRAY_SIZE = 100
        ONLY_SHOW_RESULTS = True
        SHOW_POS_INFO = False
        SHOW_RAW = False
        TRUNCATE_VALUES = True
    elif verb == 'RESULTS_EX':
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 200
        MAX_PRINT_ARRAY_SIZE = 100
        ONLY_SHOW_RESULTS = True
        SHOW_POS_INFO = True
        SHOW_RAW = False
        TRUNCATE_VALUES = True
    elif verb == 'FULL':
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 9999
        MAX_PRINT_ARRAY_SIZE = 9999
        ONLY_SHOW_RESULTS = False
        SHOW_POS_INFO = True
        SHOW_RAW = True
        TRUNCATE_VALUES = True
    elif verb == 'SILENT':
        PRINT_VERBOSE = False
        MAX_STRING_LENGTH = 1
        MAX_PRINT_ARRAY_SIZE = 1
        ONLY_SHOW_RESULTS = True
        SHOW_POS_INFO = False
        SHOW_RAW = False
        TRUNCATE_VALUES = True
    else:  # DEBUG
        PRINT_VERBOSE = True
        MAX_STRING_LENGTH = 1000
        MAX_PRINT_ARRAY_SIZE = 1000
        ONLY_SHOW_RESULTS = False
        SHOW_POS_INFO = True
        SHOW_RAW = True
        TRUNCATE_VALUES = True


# begin
set_log_verbosity()

global test_data
test_valid_exts = ['.json', '.xml', '.yaml', '.txt']
test_data = get_directories_with_files('./test/data/', './test/data/', test_valid_exts)

global test_lang_data
lang_valid_exts = ['.js', '.py', '.toml', '.csv', '.json']
test_lang_data = get_directories_with_files('./test/language/', './test/language/', lang_valid_exts)

# test_detect_lang(PRINT_VERBOSE)
lang_parser(PRINT_VERBOSE)
# lang_regex(PRINT_VERBOSE)
