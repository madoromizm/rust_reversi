use crate::domain_model::turn::Turn;
use crate::domain_model::cell_pos::CellPos;

#[derive(Debug, Clone)]
pub struct Move {
    pub turn: Turn,
    pub put_pos: CellPos,
    pub flipped_pos_list: Vec<CellPos>,
}