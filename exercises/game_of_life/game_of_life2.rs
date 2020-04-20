// game_of_life2.rs

// I AM NOT DONE

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

// implement method counting live neighbours of a cell in the universe
fn live_neighbor_count(universe: Universe, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for delta_row in [universe.height - 1, 0, 1].iter() {
        for delta_col in [universe.width - 1, 0, 1].iter() {
            if delta_row + delta_col == 0 {
                continue;
            }

            let neighbor_row = (row + *delta_row) % universe.height;
            let neighbor_col = (column + *delta_col) % universe.width;
            let idx = universe.get_index(neighbor_row, neighbor_col);
            count += universe.cells[idx] as u8;
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_universe_is_a_depressing_place() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: [Cell::Dead; 9].to_vec(),
        };
        assert_eq!(live_neighbor_count(universe, 1, 1), 0);
    }

    #[test]
    fn there_is_hope() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: [
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Dead, Cell::Dead,
                Cell::Dead, Cell::Dead, Cell::Dead,
            ].to_vec(),
        };
        assert_eq!(live_neighbor_count(universe, 1, 1), 1);
    }

    #[test]
    fn we_are_not_alone() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: [
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Dead, Cell::Alive,
            ].to_vec(),
        };
        assert_eq!(live_neighbor_count(universe, 1, 1), 2);
    }

    #[test]
    fn the_universe_is_booming() {
        let universe = Universe {
            width: 4,
            height: 4,
            cells: [
                Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
            ].to_vec(),
        };
        assert_eq!(live_neighbor_count(universe, 1, 1), 8);
    }
}
