from typing import List, Tuple, Dict

from .code_point_range import CodePointRange


class TwoStageTable:
    def __init__(self, prop: str, block_size: int):
        self._prop = prop
        self._block_size = block_size

        self._cur_cp = -1
        self._cur_block = []
        self._stage_1 = []
        self._stage_2: List[Tuple] = []
        self._blocks: Dict[Tuple, int] = {}

    @property
    def prop(self):
        return self._prop

    @property
    def block_size(self):
        return self._block_size

    def _int_size(self):
        """Get minimal int size of index of stage-2."""
        if len(self._stage_2) <= 256:
            return 1
        elif len(self._stage_2) <= (2 ** 16):
            return 2
        elif len(self._stage_2) <= (2 ** 32):
            return 4
        elif len(self._stage_2) <= (2 ** 64):
            return 8
        else:
            raise ValueError('error! len(self._stage_2): {}'.format(len(self._stage_2)))

    def _index_t(self):
        return {
            1: 'u8',
            2: 'u16',
            4: 'u32',
            8: 'u64',
        }[self._int_size()]

    def add_char(self, cp: int, value: str):
        self._cur_cp = cp
        self._cur_block.append(value)

        if len(self._cur_block) == self.block_size:
            block_tuple = tuple(self._cur_block)
            idx = self._blocks.get(block_tuple)
            # If not exists, add to stage-2.
            if idx is None:
                idx = len(self._stage_2)
                self._blocks[block_tuple] = idx
                self._stage_2.append(block_tuple)
            self._stage_1.append(idx)
            self._cur_block = []

    def to_seshat(self, prefix=False, use=True, boolean=False) -> str:
        txt = ''
        if use is True:
            if boolean is False:
                txt += 'use crate::unicode::props::{};\n'.format(self.prop)
            txt += 'use crate::collections::TwoStageTable;\n\n'

        prefix_lambda = lambda x: self.prop.upper() + '_' if x is True else ''

        # Stage-1
        txt += 'const {prefix}STAGE_1: [{int_type}; {size}] = ['.format(
            prefix=prefix_lambda(prefix),
            int_type=self._index_t(),
            size=len(self._stage_1)
        )
        for i, idx in enumerate(self._stage_1):
            if i % 16 == 0:
                txt += '\n    '
            txt += '{},'.format(idx)
            if i % 16 != 15:
                txt += ' '
        txt += '\n];\n\n'

        # Stage-2
        txt += 'const {prefix}STAGE_2: &[&[{enum_type}]] = &['.format(
            prefix=prefix_lambda(prefix),
            enum_type=(lambda x: self.prop if boolean is False else 'bool')(self.prop)
            # block_size=self.block_size,
            # size=len(self._stage_2)
        )
        for i, block in enumerate(self._stage_2):
            txt += '\n    &['
            txt += '\n        // Block {}'.format(i)
            for j, val in enumerate(block):
                if j % 8 == 0:
                    txt += '\n        '
                if boolean is False:
                    txt += '{}::{},'.format(self.prop, val)
                else:
                    txt += '{},'.format(val)
                if j % 8 != 7:
                    txt += ' '
            txt += '\n    ],'
        txt += '\n];\n\n'

        txt += 'pub(crate) fn {fn_name}(cp: u32) -> {prop} {{\n'.format(
            fn_name=self.prop.lower(),
            prop=(lambda x: self.prop if boolean is False else 'bool')(self.prop)
        )
        txt += '    let tst: TwoStageTable<{prop}, {index_t}> = TwoStageTable::new(&{prefix}STAGE_1, {prefix}STAGE_2, {block_size});\n'.format(
            prefix=prefix_lambda(prefix),
            prop=(lambda x: self.prop if boolean is False else 'bool')(self.prop),
            index_t=self._index_t(),
            block_size=self.block_size
        )
        txt += '    tst.at(cp)\n'
        txt += '}\n'

        return txt

    def table_bytes(self, repr_size: int) -> int:
        """repr_size: Representation byte size of enum type."""
        stage_1_bytes = self._int_size() * len(self._stage_1)
        stage_2_bytes = (len(self._stage_2) * self.block_size) * repr_size

        return stage_1_bytes + stage_2_bytes

    @staticmethod
    def make(prop: str, data: List[Tuple[CodePointRange, str]], block_size: int, default_prop: str=None):
        tst = TwoStageTable(prop, block_size)
        cur_cp = 0

        for pair in data:
            # Fill default property for implicit ranges.
            if cur_cp not in pair[0]:
                for cp in CodePointRange(cur_cp, pair[0].start - 1):
                    tst.add_char(cur_cp, default_prop)
                    cur_cp += 1
            # Add each characters in range.
            for cp in pair[0]:
                tst.add_char(cur_cp, pair[1])
                cur_cp += 1
        # Fill default property for remains characters.
        if cur_cp <= 0x10FFFF:
            for cp in CodePointRange(cur_cp, 0x10FFFF):
                tst.add_char(cur_cp, default_prop)
                cur_cp += 1

        return tst
