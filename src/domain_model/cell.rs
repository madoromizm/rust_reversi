use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellState {
    OuterCell, // 8x8の外側
    BlankCell, // 石が置いてないセル
    BlackStone, // 黒
    WhiteStone, // 白
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CellState::OuterCell => "  ",
            CellState::BlankCell => "  ",
            CellState::BlackStone => "黒",
            CellState::WhiteStone => "白",
            // CellState::BlackStone => "●",
            // CellState::WhiteStone => "○",
        };
        write!(f, "{}", s)
    }    
}

pub trait ICellState {
    fn draw(&self);
    fn get_reverse_stone(&self) -> CellState;
}

impl ICellState for CellState {
    fn draw(&self) {
        match self {
            CellState::OuterCell => print!("  "),
            CellState::BlankCell => print!("  "),
            CellState::BlackStone => print!("●"),
            CellState::WhiteStone => print!("○"),
        }
    }

    fn get_reverse_stone(&self) -> CellState {
        match self {
            CellState::OuterCell => CellState::OuterCell,
            CellState::BlankCell => CellState::BlankCell,
            CellState::BlackStone => CellState::WhiteStone,
            CellState::WhiteStone => CellState::BlackStone,
        }
    }
}