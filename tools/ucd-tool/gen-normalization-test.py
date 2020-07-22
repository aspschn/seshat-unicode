#!/usr/bin/env python3
import json
import os

from ucd.unicode import UNICODE_VERSION_MAJOR, UNICODE_VERSION_MINOR, UNICODE_VERSION_UPDATE

UNICODE_DATA_DIR = 'data/{}.{}.{}'.format(
    UNICODE_VERSION_MAJOR,
    UNICODE_VERSION_MINOR,
    UNICODE_VERSION_UPDATE
)

with open(os.path.join(UNICODE_DATA_DIR, 'PropertyValueAliases.json')) as f:
    property_value_aliases = json.load(f)


if __name__ == '__main__':
    filename = os.path.join(UNICODE_DATA_DIR, 'NormalizationTest.json')
    with open(filename) as f:
        json_str = f.read()
    test_cases = json.loads(json_str)
    txt = 'pub const TEST_CASES: &[(&str, &str, &str, &str, &str)] = &[\n'
    # source; NFC; NFD; NFKC; NFKD
    for source in test_cases.keys():
        txt += '    ("{}", "{}", "{}", "{}", "{}"),\n'.format(
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), source.split(' '))),
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), test_cases[source]['NFC'].split(' '))),
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), test_cases[source]['NFD'].split(' '))),
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), test_cases[source]['NFKC'].split(' '))),
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), test_cases[source]['NFKD'].split(' ')))
        )
    txt += '];\n'

    with open('../../tests/normalization_test_cases.rs', 'w') as f:
        f.write(txt)
