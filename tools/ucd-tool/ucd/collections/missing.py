from .code_point_range import CodePointRange


class Missing:
    def __init__(self):
        self._range_value_pairs = []

    def append(self, code_point_range: CodePointRange, default_value: str):
        self._range_value_pairs.append((code_point_range, default_value))

    def default_value_for(self, code_point: int):
        candidate = None
        for pairs in self._range_value_pairs:
            rng: CodePointRange = pairs[0]
            if code_point in rng:
                candidate = pairs[1]

        return candidate

    def __repr__(self):
        s = ''
        for pairs in self._range_value_pairs:
            s += str(pairs[0])
            s += ': '
            s += str(pairs[1])
            s += '\n'

        return s
