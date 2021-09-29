#!/usr/bin/env python
import os
import json

from ucd.unicode import UNICODE_VERSION_MAJOR, UNICODE_VERSION_MINOR, UNICODE_VERSION_UPDATE

UNICODE_DATA_DIR = 'data/{}.{}.{}'.format(
    UNICODE_VERSION_MAJOR,
    UNICODE_VERSION_MINOR,
    UNICODE_VERSION_UPDATE
)

files = [
    {
        'source': 'Blocks.txt',
        'target': 'Blocks.json',
    },
    {
        'source': 'CompositionExclusions.txt',
        'target': 'CompositionExclusions.json',
    },
    {
        'source': 'DerivedAge.txt',
        'target': 'DerivedAge.json',
    },
    {
        'source': 'DerivedNormalizationProps.txt',
        'target': 'DerivedNormalizationProps.json',
    },
    {
        'source': 'HangulSyllableType.txt',
        'target': 'HangulSyllableType.json',
    },
    {
        'source': 'NormalizationTest.txt',
        'target': 'NormalizationTest.json',
    },
    {
        'source': 'PropertyAliases.txt',
        'target': 'PropertyAliases.json',
    },
    {
        'source': 'PropertyValueAliases.txt',
        'target': 'PropertyValueAliases.json',
    },
    {
        'source': 'PropList.txt',
        'target': 'PropList.json',
    },
    {
        'source': 'Scripts.txt',
        'target': 'Scripts.json',
    },
    {
        'source': 'UnicodeData.txt',
        'target': 'UnicodeData.json',
    },
    {
        'source': 'auxiliary/GraphemeBreakProperty.txt',
        'target': 'auxiliary/GraphemeBreakProperty.json',
    },
    {
        'source': 'auxiliary/GraphemeBreakTest.txt',
        'target': 'auxiliary/GraphemeBreakTest.json',
    },
    {
        'source': 'auxiliary/WordBreakProperty.txt',
        'target': 'auxiliary/WordBreakProperty.json',
    },
    {
        'source': 'emoji/emoji-data.txt',
        'target': 'emoji/emoji-data.json',
    },
    {
        'source': 'extracted/DerivedBidiClass.txt',
        'target': 'extracted/DerivedBidiClass.json',
    },
    {
        'source': 'extracted/DerivedCombiningClass.txt',
        'target': 'extracted/DerivedCombiningClass.json',
    },
    {
        'source': 'extracted/DerivedDecompositionType.txt',
        'target': 'extracted/DerivedDecompositionType.json',
    },
    {
        'source': 'extracted/DerivedGeneralCategory.txt',
        'target': 'extracted/DerivedGeneralCategory.json',
    },
    {
        'source': 'extracted/DerivedName.txt',
        'target': 'extracted/DerivedName.json',
    },
]

def load_env(path):
    try:
        f = open(path, 'r')
        lines = f.readlines()
        for line in lines:
            if line.startswith('#'):
                continue
            (k, v) = line.split('=')
            os.environ[k] = v
        f.close()
    except FileNotFoundError:
        return

if __name__ == '__main__':
    load_env('.env')
    ucd_json_path = os.environ.get('UCD_JSON_DIR', None)
    if ucd_json_path is None:
        print('UCD_JSON_DIR is not set.')
        exit(1)
    ucd_json_dir = os.environ.get('UCD_JSON_DIR')

    os.system(f'mkdir -p {UNICODE_DATA_DIR}')
    os.system(f'mkdir -p {UNICODE_DATA_DIR}/emoji')
    os.system(f'mkdir -p {UNICODE_DATA_DIR}/extracted')
    os.system(f'mkdir -p {UNICODE_DATA_DIR}/auxiliary')
    for file in files:

        source = file['source']
        target = file['target']
        os.system(f'{ucd_json_dir}/ucd-json {source} > {UNICODE_DATA_DIR}/{target}')
