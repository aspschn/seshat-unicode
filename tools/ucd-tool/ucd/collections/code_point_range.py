class CodePointRange:
    def __init__(self, start: int, end: int=None):
        if end is None:
            end = start

        if end < start:
            raise ValueError('End value is lower than start value.')

        self._start = start
        self._end = end

    @property
    def start(self):
        return self._start

    @property
    def end(self):
        return self._end

    def __lt__(self, other):
        return (self.start < other.start) and (self.end < other.end)

    def __gt__(self, other):
        return (self.start > other.start) and (self.end > other.end)

    def __contains__(self, cp: int):
        return self.start <= cp <= self.end

    def __iter__(self):
        self._iter_i = self.start - 1
        return self

    def __next__(self):
        if self._iter_i < self.end:
            self._iter_i += 1
            return self._iter_i
        else:
            raise StopIteration

    def __repr__(self):
        return '{:04X}..{:04X}'.format(self.start, self.end)

    @staticmethod
    def parse(r: str):
        start = 0
        end = 0

        start_end = r.split('..')
        start = int(start_end[0], 16)
        if len(start_end) == 1:
            end = int(start_end[0], 16)
        else:
            end = int(start_end[1], 16)

        return CodePointRange(start, end)
