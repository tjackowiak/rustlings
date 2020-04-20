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

    // TODO: implement me!
    // based on current state count live neighbours for every cell
    // and prepare new Vec<Cell> representing updated universe
    pub fn tick(&mut self) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loneliness_is_deadly() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: [
                Cell::Alive, Cell::Dead, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Dead, Cell::Dead
            ].to_vec(),
        };

        universe.tick();

        assert_eq!(universe.cells, [Cell::Dead; 9]);
    }

    #[test]
    fn stable_together() {
        let cells = [
            Cell::Alive, Cell::Alive, Cell::Dead,
            Cell::Alive, Cell::Alive, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Dead
        ].to_vec();

        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: cells.clone(),
        };

        universe.tick();

        assert_eq!(universe.cells, cells);
    }

    #[test]
    fn make_me_a_pulsar() {
        let mut universe = Universe {
            width: 3,
            height: 5,
            cells: [
                Cell::Dead, Cell::Dead, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Dead, Cell::Dead, Cell::Dead,
            ].to_vec(),
        };

        universe.tick();

        assert_eq!(universe.cells, [
            Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Alive, Cell::Alive, Cell::Alive,
            Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Dead,
        ].to_vec());
    }
}
