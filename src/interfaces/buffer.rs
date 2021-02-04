pub trait Buffer {
    type Row;

    fn append_row(&mut self) -> &mut Self {
        self.insert_row(self.row_count());
        self
    }

    fn col_count(&self, row_index: usize) -> usize;
    fn delete_row(&mut self, row_index: usize) -> Option<Self::Row>;
    fn edit_row(&mut self, row_index: usize) -> Option<&mut Self::Row>;
    fn insert_row(&mut self, row_index: usize) -> &mut Self;
    fn row_count(&self) -> usize;
}
