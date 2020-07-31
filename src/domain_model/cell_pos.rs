

#[derive(Debug, Clone)]
pub struct CellPos {
    pub row: i8,
    pub col: i8,
}

// impl TryInto<usize> for i8 {
//     fn try_into(self) -> usize {
//         self.try_into().unwrap()
//     }
// }