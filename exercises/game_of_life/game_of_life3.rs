// game_of_life3.rs

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

    // implement method counting live neighbours of a cell in the universe
    // where cells on the edges have neighbors that wrap around to the other side of the universe
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {

    }
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
        assert_eq!(universe.live_neighbor_count(1, 1), 0);
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
        assert_eq!(universe.live_neighbor_count(1, 1), 1);
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
        assert_eq!(universe.live_neighbor_count(1, 1), 2);
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
        assert_eq!(universe.live_neighbor_count(1, 1), 8);
    }

    #[test]
    fn there_is_no_final_frontier() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: [
                Cell::Alive, Cell::Dead, Cell::Alive,
                Cell::Dead, Cell::Dead, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
            ].to_vec(),
        };
        assert_eq!(universe.live_neighbor_count(0, 0), 2);
        assert_eq!(universe.live_neighbor_count(0, 1), 3);
        assert_eq!(universe.live_neighbor_count(0, 2), 2);

        assert_eq!(universe.live_neighbor_count(1, 0), 3);
        assert_eq!(universe.live_neighbor_count(1, 1), 3);
        assert_eq!(universe.live_neighbor_count(1, 2), 3);

        assert_eq!(universe.live_neighbor_count(2, 0), 3);
        assert_eq!(universe.live_neighbor_count(2, 1), 2);
        assert_eq!(universe.live_neighbor_count(2, 2), 3);
    }
}
