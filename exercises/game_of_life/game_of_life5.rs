// game_of_life3.rs

// I AM NOT DONE

use std::fmt;

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
    pub fn render(&self) -> String {
        self.to_string()
    }
}


// TODO: implement me
// use `write!(f, ...)?;` to write your input
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "just an example")
        // Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn im_alive() {
        let mut universe = Universe {
            width: 1,
            height: 1,
            cells: [Cell::Alive].to_vec(),
        };

        assert_eq!(universe.render(), "◼\n" );
    }

    #[test]
    fn and_im_not() {
        let mut universe = Universe {
            width: 1,
            height: 1,
            cells: [Cell::Dead].to_vec(),
        };

        assert_eq!(universe.render(), "◻\n" );
    }


    #[test]
    fn draw_me_like_a_star() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: [
                Cell::Alive, Cell::Dead, Cell::Alive,
                Cell::Dead, Cell::Alive, Cell::Dead,
                Cell::Alive, Cell::Dead, Cell::Alive
            ].to_vec(),
        };

        assert_eq!(universe.render(), concat!(
        "◼◻◼\n",
        "◻◼◻\n",
        "◼◻◼\n",
        ));
    }
}
