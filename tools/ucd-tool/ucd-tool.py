#!/usr/bin/env python3
import os
import json
import re
from typing import List, Tuple, Dict

from ucd.unicode import UNICODE_VERSION_MAJOR, UNICODE_VERSION_MINOR, UNICODE_VERSION_UPDATE
from ucd.unicode import EMOJI_VERSION_MAJOR, EMOJI_VERSION_MINOR
from ucd.collections import CodePointRange, TwoStageTable


UNICODE_DATA_DIR = 'data/{}.{}.{}'.format(
    UNICODE_VERSION_MAJOR,
    UNICODE_VERSION_MINOR,
    UNICODE_VERSION_UPDATE
)

property_info = {
    'bc': {
        'repr_size': 1,
    },
    'gc': {
        'repr_size': 1,
    },
    'hst': {
        'repr_size': 1,
    },
    'gcb': {
        'repr_size': 1,
    },
    'ccc': {
        'repr_size': 1,
    },
    'dt': {
        'repr_size': 1,
    },
    'blk': {
        'repr_size': 2,
    },
}


with open(os.path.join(UNICODE_DATA_DIR, 'PropertyAliases.json')) as f:
    property_aliases = json.load(f)

with open(os.path.join(UNICODE_DATA_DIR, 'PropertyValueAliases.json')) as f:
    property_value_aliases = json.load(f)


def find_key_by_value(d: Dict, val: str) -> str:
    for k, v in d.items():
        if v == val:
            return k


def to_snake_case(val: str):
    if val == 'ExtPict':
        return 'ext_pict'

    return val.lower()


def data_value_as_abbr(data: List[Tuple[CodePointRange, str]], prop: str):
    """Some property data file using (range, full_name) pair instead of (range, abbr_name).
This function converts these data to abbr version."""
    aliases = property_value_aliases[prop]
    new_data = []
    for pair in data:
        new_pair = (pair[0], find_key_by_value(aliases, pair[1]))
        new_data.append(new_pair)

    return new_data


def data_value_as_abbr_ccc(data: List[Tuple[CodePointRange, str]]):
    aliases = property_value_aliases['ccc']
    new_data = []
    for pair in data:
        new_pair = (pair[0], aliases[pair[1]])
        new_data.append(new_pair)

    return new_data


def select_minimal_tst(prop: str, data: List[Tuple[CodePointRange, str]], repr_size: int, default_prop: str=None) -> TwoStageTable:
    print('Select minimal table for: {}'.format(prop))
    tables = {64: None, 128: None, 256: None, 512: None}
    for block_size in tables.keys():
        tst = TwoStageTable.make(prop, data, block_size, default_prop)
        print('Block size {}: {}'.format(block_size, tst.table_bytes(repr_size)))
        tables[block_size] = tst
    print('----------------------------')

    return min(tables.values(), key=lambda x: x.table_bytes(repr_size))


def make_data(filename: str):
    """Make data used by argument of two-stage table.
filename: str - Path of JSON file."""
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


def make_grouped_data(filename: str):
    """e.g. emoji/emoji-data.json"""
    filename = os.path.join(UNICODE_DATA_DIR, filename)
    f = open(filename)
    json_str = f.read()
    f.close()
    d = json.loads(json_str)
    data_dict = {}
    for prop, ranges in d.items():
        data_dict[prop] = []
        for rng in ranges:
            rng = CodePointRange.parse(rng)
            data_dict[prop].append((rng, "true"))
        data_dict[prop] = sorted(data_dict[prop], key=lambda x: x[0])

    return data_dict


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


def na_table_rs() -> str:
    filename = os.path.join(UNICODE_DATA_DIR, 'extracted/DerivedName.json')
    f = open(filename)
    json_str = f.read()
    f.close()
    d = json.loads(json_str)
    txt = '#[allow(dead_code)]\n'
    txt += 'pub(super) const NA_MAP: &[(u32, &\'static str)] = &[\n'
    for k, name in d.items():
        cp = CodePointRange.parse(k)
        # Ignore `HANGUL SYLLABLE` prefix.
        if cp.start in CodePointRange.parse('AC00..D7A3'):
            continue
        # Ignore `CJK UNIFIED IDEOGRAPH-` prefix.
        if cp.start in CodePointRange.parse('3400..4DBF'):
            continue
        if cp.start in CodePointRange.parse('4E00..9FFC'):
            continue
        if cp.start in CodePointRange.parse('20000..2A6DD'):
            continue
        if cp.start in CodePointRange.parse('2A700..2B734'):
            continue
        if cp.start in CodePointRange.parse('2B740..2B81D'):
            continue
        if cp.start in CodePointRange.parse('2B820..2CEA1'):
            continue
        if cp.start in CodePointRange.parse('2CEB0..2EBE0'):
            continue
        if cp.start in CodePointRange.parse('30000..3134A'):
            continue
        # Ignore `TANGUT IDEOGRAPH-` prefix.
        if cp.start in CodePointRange.parse('17000..187F7'):
            continue
        if cp.start in CodePointRange.parse('18D00..18D08'):
            continue
        # Ignore `KHITAN SMALL SCRIPT CHARACTER-` prefix.
        if cp.start in CodePointRange.parse('18B00..18CD5'):
            continue
        # Ignore `NUSHU CHARACTER-` prefix.
        if cp.start in CodePointRange.parse('1B170..1B2FB'):
            continue
        # Ignore `CJK COMPATIBILITY IDEOGRAPH-` prefix.
        if cp.start in CodePointRange.parse('F900..FA6D') or \
                cp.start in CodePointRange.parse('FA70..FAD9') or \
                cp.start in CodePointRange.parse('2F800..2FA1D'):
            continue
        txt += '    (0x{:04X}, "{}"),\n'.format(cp.start, name)
    txt += '];\n'

    return txt


def dm_map_rs():
    filename = os.path.join(UNICODE_DATA_DIR, 'UnicodeData.json')
    with open(filename) as f:
        json_str = f.read()
    unicode_data = json.loads(json_str)
    txt = 'pub(super) const DM_MAP: &[(u32, &str)] = &[\n'
    for k, props in unicode_data.items():
        dm_raw = props['dm']
        if dm_raw == '':
            continue
        cp = CodePointRange.parse(k)
        # Make dm_raw to str.
        if dm_raw.startswith('<'):
            dm_raw = re.sub('<.+> ', '', dm_raw)
        dm_str = ''
        for code in dm_raw.split(' '):
            dm_str += '\\u{{{}}}'.format(code)
        txt += '    (0x{:04X}, "{}"),\n'.format(cp.start, dm_str)
    txt += '];\n'

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
    # Make na data table.
    f = open('../../src/unicode/ucd/na/na_table.rs', 'w')
    f.write(na_table_rs())
    f.close()
    # Make hst properties data.
    hst_data = make_data('HangulSyllableType.json')
    tst = select_minimal_tst('Hst', hst_data, property_info['hst']['repr_size'], default_prop='NA')
    f = open('../../src/unicode/ucd/hst.rs', 'w')
    f.write(tst.to_seshat())
    f.close()
    # Make Emoji data properties data.
    emoji_props_rs = ''
    emoji_data_dict = make_grouped_data('emoji/emoji-data.json')
    for i, prop in enumerate(emoji_data_dict.keys()):
        use = False
        if i == 0:
            use = True
        prop_alias = to_snake_case(find_key_by_value(property_aliases, prop))
        tst = select_minimal_tst(prop_alias, emoji_data_dict[prop], 1, default_prop='false')
        emoji_props_rs += tst.to_seshat(use=use, prefix=True, boolean=True)
    with open('../../src/unicode/ucd/emoji_props.rs', 'w') as f:
        f.write(emoji_props_rs)
    # Make GCB data.
    gcb_data = make_data('auxiliary/GraphemeBreakProperty.json')
    gcb_data = data_value_as_abbr(gcb_data, "GCB")
    tst = select_minimal_tst('Gcb', gcb_data, property_info['gcb']['repr_size'], default_prop='XX')
    with open('../../src/unicode/ucd/gcb.rs', 'w') as f:
        f.write(tst.to_seshat())
    # Make bc data.
    bc_data = make_data('extracted/DerivedBidiClass.json')
    tst = select_minimal_tst('Bc', bc_data, property_info['bc']['repr_size'], default_prop='L')
    with open('../../src/unicode/ucd/bc.rs', 'w') as f:
        f.write(tst.to_seshat())
    # Make ccc data.
    ccc_data = make_data('extracted/DerivedCombiningClass.json')
    ccc_data = data_value_as_abbr_ccc(ccc_data)
    tst = select_minimal_tst('Ccc', ccc_data, property_info['ccc']['repr_size'], default_prop='NR')
    with open('../../src/unicode/ucd/ccc.rs', 'w') as f:
        f.write(tst.to_seshat())
    # Make dt data.
    dt_data = make_data('extracted/DerivedDecompositionType.json')
    dt_data = data_value_as_abbr(dt_data, 'dt')
    tst = select_minimal_tst('Dt', dt_data, property_info['dt']['repr_size'], default_prop='None')
    with open('../../src/unicode/ucd/dt.rs', 'w') as f:
        f.write(tst.to_seshat())
    # Make dm data.
    with open('../../src/unicode/ucd/dm/dm_map.rs', 'w') as f:
        f.write(dm_map_rs())
