// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub struct LineIter {
    x: i32,
    y: i32,
    counter: i32,
    error: i32,
    error_increment: i32,
    error_decrement: i32,
    increment: (i8, i8),
    correction_increment: (i8, i8)
}

impl LineIter {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> LineIter {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        let (delta_x, increment_x) = make_delta(start_x, end_x);
        let (delta_y, increment_y) = make_delta(start_y, end_y);
        if delta_x > delta_y {
            LineIter {
                x: start_x,
                y: start_y,
                counter: delta_x,
                error: delta_y * 2 - delta_x,
                error_increment: delta_y * 2,
                error_decrement: delta_x * 2,
                increment: (increment_x, 0),
                correction_increment: (0, increment_y)
            }
        } else {
            LineIter {
                x: start_x,
                y: start_y,
                counter: delta_y,
                error: delta_x * 2 - delta_y,
                error_increment: delta_x * 2,
                error_decrement: delta_y * 2,
                increment: (0, increment_y),
                correction_increment: (increment_x, 0)
            }
        }
    }
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.counter < 0 {
            return None;
        }
        self.counter -= 1;
        let (x, y) = (self.x, self.y);
        if self.error >= 0 {
            self.error -= self.error_decrement;
            self.x += self.correction_increment.0 as i32;
            self.y += self.correction_increment.1 as i32;
        }
        self.error += self.error_increment;
        self.x += self.increment.0 as i32;
        self.y += self.increment.1 as i32;
        Some((x, y))
    }
}

fn make_delta(start: i32, end: i32) -> (i32, i8) {
    if start < end {
        (end - start, 1)
    } else {
        (start - end, -1)
    }
}

#[test]
fn simple_line() {
    let line: Vec<(i32, i32)> = LineIter::new((0, 0), (9, 4)).collect();
    let compare_line = [(0, 0), (1, 0), (2, 1), (3, 1), (4, 2), (5, 2), (6, 3), (7, 3), (8, 4), (9, 4)];
    assert_eq!(line, compare_line);
}
