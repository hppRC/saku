use crate::ControlFlow;

#[derive(Clone, Debug)]
pub struct CharTable {
    max_idx: usize,
    table: Vec<Option<ControlFlow>>,
}

impl Default for CharTable {
    #[inline]
    fn default() -> Self {
        Self {
            max_idx: 0,
            table: vec![None],
        }
    }
}

impl CharTable {
    #[inline(always)]
    pub(crate) fn insert(&mut self, idx: char, value: ControlFlow) {
        let idx = idx as usize;
        if idx > self.max_idx {
            self.table.extend(vec![None; idx - self.max_idx]);
            self.max_idx = idx;
        }
        self.table[idx] = Some(value)
    }
    #[inline(always)]
    pub(crate) fn get(&self, idx: char) -> Option<&ControlFlow> {
        let idx = idx as usize;
        if idx > self.max_idx {
            None
        } else {
            self.table[idx].as_ref()
        }
    }
}
