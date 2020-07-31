use crate::domain_model::cell::{CellState, ICellState};
use crate::error::ReversiError;
use crate::domain_model::cell_pos::CellPos;

#[derive(Debug)]
pub struct Board {
    pub cells: [[CellState; 10]; 10],
    search_directions: [CellPos; 8],
}

impl Board {
    pub fn new() -> Board {
        
        let serch_dir = [
            CellPos{row: 0, col: 1},
            CellPos{row: 1, col: 1},
            CellPos{row: 1, col: 0},
            CellPos{row: 1, col: -1},
            CellPos{row: 0, col: -1},
            CellPos{row: -1, col: -1},
            CellPos{row: -1, col: 0},
            CellPos{row: -1, col: 1},
        ];

        // cell初期化
        fn make_cells() -> [[CellState; 10]; 10] {
            let mut cells = [[CellState::OuterCell; 10]; 10];
            for i in 1..=8 {
                for j in 1..=8 {
                    cells[i][j] = CellState::BlankCell;
                }
            }
            cells[4][4] = CellState::WhiteStone;
            cells[4][5] = CellState::BlackStone;
            cells[5][4] = CellState::BlackStone;
            cells[5][5] = CellState::WhiteStone;
            cells
        }

        Board {
            cells: make_cells(),
            search_directions: serch_dir,
        }
    }

    /// 盤面の範囲内か
    pub fn is_in_range(&self, row: usize, col: usize) -> bool {
        !(row < 1 || 8 < row || col < 1 || 8 < col)
    }

    pub fn is_in_range_with_outercell(&self, row: usize, col: usize) -> bool {
        !(row < 0 || 9 < row || col < 0 || 9 < col)
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<CellState> {
        if self.is_in_range_with_outercell(row, col) {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell_state: CellState) -> Result<(), ReversiError> {
        if self.is_in_range(row, col) {
            self.cells[row][col] = cell_state;
            Ok(())
        } else {
            Err(ReversiError::new(format!("セルインデックスが範囲外です: row={}, col={}", row, col)))
        }
    }

    pub fn has_blank_cell(&self) -> bool {
        for row in 1..=8 {
            for col in 1..=8 {
                if self.cells[row][col] == CellState::BlankCell {
                    return true
                }
            }
        }

        false
    }

    pub fn count_black_stones(&self) -> i8 {
        let mut black_stone_count: i8 = 0;
        for row in 1..=8 {
            for col in 1..=8 {
                if self.cells[row][col] == CellState::BlackStone {
                    black_stone_count += 1;
                }
            }
        }

        black_stone_count
    }

    pub fn count_white_stones(&self) -> i8 {
        64 - self.count_black_stones()
    }

    pub fn flip(&mut self, cell_pos_list: &Vec<CellPos>) {
        for pos in cell_pos_list.iter() {
            self.cells[pos.row as usize][pos.col as usize] = self.cells[pos.row as usize][pos.col as usize].get_reverse_stone();
        }
    }

    // test: CellStateにBlankCell, OuterCellを指定したらエラー
    pub fn find_flippable_cells(&self, row: usize, col: usize, stone: CellState) -> Result<Vec<CellPos>, ReversiError> {
        match stone {
            CellState::BlankCell => return Err(ReversiError::new("BlankCellは指定できません")), 
            CellState::OuterCell => return Err(ReversiError::new("OuterCellは指定できません")),
            _ => {},
        }
        let row = row as i8;
        let col = col as i8;

		// 反転できるか探索する
		// 全8方向について、異なる色の石がある間、この石をカウントしながら進み、次の条件
		// ・盤の範囲外となる(OuterCell)
		// ・空のマスとなる(BlankCell)
		// ・同じ色の石となる(DarkDisk|LightDisk)
		// に合致すればループを終了する。
		// 同じ色の石が見つかったときのみ、異なる色の石を反転させる。
        
        //int s_row, s_col;
        let mut s_row: i8;
        let mut s_col: i8;
		// ICellState oppositeColorDisk = disk.getReverseDisk();
        let opposite_color_stone = stone.get_reverse_stone();
        
        // ArrayList<CellPos> flipCells = new ArrayList<CellPos>();
        let mut flip_cells: Vec<CellPos> = Vec::new();


        for dir in self.search_directions.iter() {
            let mut opposite_color_cells: Vec<CellPos> = Vec::new();
            s_row = row + dir.row;
            s_col = col + dir.col;
            while self.cells[s_row as usize][s_col as usize] == opposite_color_stone {
                opposite_color_cells.push(CellPos{row: s_row, col: s_col});
                s_row += dir.row;
                s_col += dir.col;
            }
			// 最後に探索したセルが同色の石の場合、反転セルに追加する
            if self.cells[s_row as usize][s_col as usize] == stone {
                flip_cells.append(&mut opposite_color_cells);
            }
        }

        Ok(flip_cells)
   }
}