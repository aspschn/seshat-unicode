#!/usr/bin/env python3
import json
import os

from ucd.unicode import UNICODE_VERSION_MAJOR, UNICODE_VERSION_MINOR, UNICODE_VERSION_UPDATE

UNICODE_DATA_DIR = 'data/{}.{}.{}'.format(
    UNICODE_VERSION_MAJOR,
    UNICODE_VERSION_MINOR,
    UNICODE_VERSION_UPDATE
)


def json_string_to_rust_str(json_string):
    rust_str = '"'
    li = json_string.split(' ')
    for character in li:
        rust_str += '\\u{{{}}}'.format(character)
    rust_str += '"'

    return rust_str


if __name__ == '__main__':
    filename = os.path.join(UNICODE_DATA_DIR, 'auxiliary/GraphemeBreakTest.json')
    with open(filename) as f:
        json_str = f.read()
    test_cases = json.loads(json_str)
    txt = 'pub const TEST_CASES: &[(&str, &[&str])] = &[\n'
    for source in test_cases:
        # Origin string.
        txt += '    ("{}", '.format(
            ''.join(map(lambda x: '\\u{{{}}}'.format(x), source.split(' ')))
        )
        # Breaks.
        txt += '&['
        li = map(lambda x: json_string_to_rust_str(x), test_cases[source])
        txt += ', '.join(li)
        txt += ']),\n'
    txt += '];\n'

    with open('../../tests/grapheme_break_test_cases.rs', 'w') as f:
        f.write(txt)
