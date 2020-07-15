#!/usr/bin/env python3
import os
import json
import re
from typing import List, Tuple

from ucd.collections import CodePointRange, TwoStageTable

UNICODE_VERSION_MAJOR = 13
UNICODE_VERSION_MINOR = 0
UNICODE_VERSION_UPDATE = 0
EMOJI_VERSION_MAJOR = 13
EMOJI_VERSION_MINOR = 0

UNICODE_DATA_DIR = 'data/{}.{}.{}'.format(
    UNICODE_VERSION_MAJOR,
    UNICODE_VERSION_MINOR,
    UNICODE_VERSION_UPDATE
)

property_info = {
    'gc': {
        'repr_size': 1,
    }
}


with open(os.path.join(UNICODE_DATA_DIR, 'PropertyAliases.json')) as f:
    property_aliases = json.load(f)


def to_snake_case(val: str):
    if val == 'ExtPict':
        return 'ext_pict'

    return val.lower()


def select_minimal_tst(prop: str, data: List[Tuple[CodePointRange, str]], repr_size: int, default_prop: str=None) -> TwoStageTable:
    tables = {64: None, 128: None, 256: None, 512: None}
    for block_size in tables.keys():
        tst = TwoStageTable.make(prop, data, block_size, default_prop)
        print('Block size {}: {}'.format(block_size, tst.table_bytes(repr_size)))
        tables[block_size] = tst

    return min(tables.values(), key=lambda x: x.table_bytes(repr_size))


def make_data(filename: str):
    filename = os.path.join(UNICODE_DATA_DIR, filename)
    f = open(filename)
    json_str = f.read()
    f.close()
    d = json.loads(json_str)
    data = []
    for k, v in d.items():
        cp_range = CodePointRange.parse(k)
        data.append((cp_range, v))
    data = sorted(data, key=lambda x: x[0])

    return data


def binary_props_rs() -> str:
    filename = os.path.join(UNICODE_DATA_DIR, 'PropList.json')
    f = open(filename)
    json_str = f.read()
    f.close()
    d = json.loads(json_str)
    txt = ''
    prev_prop = ''
    for k, ranges in d.items():
        # Closing function.
        if prev_prop != k and prev_prop != '':
            txt += '\n    false\n'
            txt += '}\n\n'
        # Opening function.
        if prev_prop != k:
            fn_name = ''
            for alias, long in property_aliases.items():
                if long == k:
                    fn_name = to_snake_case(alias)
            txt += 'pub(crate) fn {fn_name}(cp: u32) -> bool {{\n'.format(fn_name=fn_name)
        for r in ranges:
            r = CodePointRange.parse(r)
            txt += '    if (0x{:04X}..0x{:04X} + 1).contains(&cp) {{\n'.format(r.start, r.end)
            txt += '        return true;\n'
            txt += '    }\n'
        prev_prop = k
    txt += '\n    false\n'
    txt += '}\n'
    return txt


if __name__ == '__main__':
    from pprint import pprint

    # Make gc data.
    gc_data = make_data('extracted/DerivedGeneralCategory.json')
    tst = select_minimal_tst('Gc', gc_data, property_info['gc']['repr_size'])
    f = open('../../src/unicode/ucd/gc.rs', 'w')
    f.write(tst.to_seshat())
    f.close()
    # Make binary properties data.
    f = open('../../src/unicode/ucd/binary_props.rs', 'w')
    f.write(binary_props_rs())
    f.close()
