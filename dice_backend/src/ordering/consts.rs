/// Dice3 is a constant collection of the values a d3 can roll
#[derive(Clone)]
pub struct Dice3 {
    arg: [i32; 3],
}
impl Dice3 {
    /// build a new Dice3
    pub const fn new() -> Dice3 {
        Dice3 { arg: [1, 2, 3] }
    }
}
impl IntoIterator for Dice3 {
    type Item = i32;
    type IntoIter = Dice3Iter;
    fn into_iter(self) -> Dice3Iter {
        Dice3Iter {
            pos: 0,
            arg: self.arg,
        }
    }
}

/// Dice3Iter iterates over a Dice3 collection
#[derive(Clone)]
pub struct Dice3Iter {
    pos: usize,
    arg: [i32; 3],
}
impl Iterator for Dice3Iter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.pos >= 3 {
            None
        } else {
            let x: i32 = self.arg[self.pos].clone();
            self.pos += 1;
            Some(x)
        }
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(3))
    }
}
impl ::std::iter::ExactSizeIterator for Dice3Iter {
    #[inline(always)]
    fn len(&self) -> usize {
        if self.pos >= 3 {
            0
        } else {
            3 - self.pos
        }
    }
}

/// Dice6 is a constant collection of what dice a d6 can roll
#[derive(Clone)]
pub struct Dice6 {
    arg: [i32; 6],
}
impl Dice6 {
    pub const fn new() -> Dice6 {
        Dice6 {
            arg: [1, 2, 3, 4, 5, 6],
        }
    }
}
impl IntoIterator for Dice6 {
    type Item = i32;
    type IntoIter = Dice6Iter;
    fn into_iter(self) -> Dice6Iter {
        Dice6Iter {
            pos: 0,
            arg: self.arg,
        }
    }
}

/// Dice6Iter iterates over a Dice6 collection
#[derive(Clone)]
pub struct Dice6Iter {
    pos: usize,
    arg: [i32; 6],
}
impl Iterator for Dice6Iter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.pos >= 6 {
            None
        } else {
            let x: i32 = self.arg[self.pos].clone();
            self.pos += 1;
            Some(x)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(6))
    }
}
impl ::std::iter::ExactSizeIterator for Dice6Iter {
    fn len(&self) -> usize {
        if self.pos >= 6 {
            0
        } else {
            6 - self.pos
        }
    }
}
