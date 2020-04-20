// game_of_life1.rs
// Time to implement Game of Life
// I AM NOT DONE

// Create Cell enum with two states DEAD and ALIVE
// enum Cell { ...

// Create method computing next cell state based on live neighbours count
// fn next_state(...)

#[cfg(test)]
mod tests {
    use super::*;

    // Rule 1: Any live cell with fewer than two live neighbours
    // dies, as if caused by underpopulation.
    #[test]
    fn rule_1() {
        for n in 0..2 {
            assert_eq!(next_state(Cell::Alive, n), Cell::Dead);
        }
    }

    // Rule 2: Any live cell with two or three live neighbours
    // lives on to the next generation.
    #[test]
    fn rule_2() {
        for n in 2..=3 {
            assert_eq!(next_state(Cell::Alive, n), Cell::Alive);
        }
    }

    // Rule 3: Any live cell with more than three live
    // neighbours dies, as if by overpopulation.
    #[test]
    fn rule_3() {
        for n in 4..=8 {
            assert_eq!(next_state(Cell::Alive, n), Cell::Dead);
        }
    }

    // Rule 4: Any dead cell with exactly three live neighbours
    // becomes a live cell, as if by reproduction.
    #[test]
    fn rule_4() {
        assert_eq!(next_state(Cell::Dead, 3), Cell::Alive);
    }

    // All other cells remain dead.
    #[test]
    fn dead_remain_dead() {
        for n in [0, 1, 2, 4, 5, 6, 7, 8].iter() {
            assert_eq!(next_state(Cell::Dead, *n), Cell::Dead);
        }
    }
}
