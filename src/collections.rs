use num::Num;
use num::cast::AsPrimitive;

pub struct TwoStageTable<'a, T, IndexT> {
    stage_1: &'a [IndexT],
    stage_2: &'a [&'a [T]],
    block_size: usize,
}

impl<'a, T: Copy, IndexT: Num + AsPrimitive<usize>> TwoStageTable<'a, T, IndexT> {
    pub fn new(
        stage_1: &'a [IndexT],
        stage_2: &'a [&'a [T]],
        block_size: usize,
    ) -> TwoStageTable<'a, T, IndexT> {
        TwoStageTable {
            stage_1: stage_1,
            stage_2: stage_2,
            block_size: block_size,
        }
    }

    pub fn at(&self, cp: u32) -> T {
        let block_size = self.block_size as u32;
        let offset = cp % block_size;
        let stage_1_index: usize = (cp / block_size) as usize;

        self.stage_2[(self.stage_1[stage_1_index]).as_()][offset as usize]
    }
}

