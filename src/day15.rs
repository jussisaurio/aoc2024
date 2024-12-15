use std::collections::VecDeque;

use crate::util::aoc_read_day_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Move {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!(),
        }
    }
    fn to_direction(self) -> (isize, isize) {
        match self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Object {
    Empty,
    Wall,
    Robot,
    Rock,
    BoxLeft,
    BoxRight,
}

impl Object {
    fn from_char(c: char) -> Object {
        match c {
            '.' => Object::Empty,
            '#' => Object::Wall,
            '@' => Object::Robot,
            'O' => Object::Rock,
            '[' => Object::BoxLeft,
            ']' => Object::BoxRight,
            _ => unreachable!(),
        }
    }
    fn to_char(self) -> char {
        match self {
            Object::Empty => '.',
            Object::Wall => '#',
            Object::Robot => '@',
            Object::Rock => 'O',
            Object::BoxLeft => '[',
            Object::BoxRight => ']',
        }
    }
}

pub fn day15_part1(test_input: Option<Vec<String>>) -> usize {
    let lines = test_input.unwrap_or(aoc_read_day_lines(15));
    let mut robot_pos = (0, 0);
    // take until the first empty line
    let mut map_lines = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let obj = Object::from_char(c);
                    if obj == Object::Robot {
                        robot_pos = (x, y);
                    }
                    obj
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // concat all movelines to single line
    let moves = lines
        .iter()
        .skip(map_lines.len() + 1)
        .flat_map(|line| line.chars())
        .map(|c| Move::from_char(c))
        .collect::<Vec<_>>();

    let mut i = 0;
    'mainloop: for m in moves.iter() {
        i += 1;
        // clear screen, print map, sleep 500 ms
        // println!("\x1b[2J\x1b[H");
        // println!("move {}, trying to go {:?}", i, m.to_direction());
        // println!("robot pos: {:?}", robot_pos);
        // for line in map_lines.iter() {
        //     println!("{}", line.iter().map(|o| o.to_char()).collect::<String>());
        // }
        // std::thread::sleep(std::time::Duration::from_millis(10));
        let direction = m.to_direction();
        let cur_pos = robot_pos;
        let new_pos_candidate = (
            robot_pos.0 as isize + direction.0,
            robot_pos.1 as isize + direction.1,
        );
        if cant_push(&map_lines, new_pos_candidate) {
            continue;
        }
        // find the first empty space in the direction of the move. abort if any walls
        let mut rocks_pushed: usize = 0;
        loop {
            let pos = (
                new_pos_candidate.0 + rocks_pushed as isize * direction.0,
                new_pos_candidate.1 + rocks_pushed as isize * direction.1,
            );
            if cant_push(&map_lines, pos) {
                continue 'mainloop;
            }
            if pos_is(&map_lines, pos, Object::Empty) {
                break;
            }
            rocks_pushed += 1;
        }
        // starting from the furthest rock pushed, push all rocks in the direction of the move
        // then move the robot and replace its old position with empty
        for i in 1..=rocks_pushed {
            let rock_new_y = new_pos_candidate.1 as isize + i as isize * direction.1;
            let rock_new_x = new_pos_candidate.0 as isize + i as isize * direction.0;
            map_lines[rock_new_y as usize][rock_new_x as usize] = Object::Rock;
        }
        robot_pos = (new_pos_candidate.0 as usize, new_pos_candidate.1 as usize);
        map_lines[robot_pos.1 as usize][robot_pos.0 as usize] = Object::Robot;
        map_lines[cur_pos.1 as usize][cur_pos.0 as usize] = Object::Empty;
    }

    print_map(&map_lines);

    let mut gps = 0;
    for y in 0..map_lines.len() {
        for x in 0..map_lines[y].len() {
            if let Object::Rock = map_lines[y][x] {
                gps += 100 * y + x
            }
        }
    }
    gps
}

// prints map when assertion fails
macro_rules! assert_print_map {
    ($cond:expr, $map:expr, $msg:expr) => {
        if std::env::var("AOC_DEBUG").is_ok() {
            if !$cond {
                print_map($map);
                panic!("assertion failed: {}", $msg);
            }
        }
    };
}

pub fn day15_part2(test_input: Option<Vec<String>>) -> usize {
    let lines = test_input.unwrap_or(aoc_read_day_lines(15));
    let mut robot_pos = (0, 0);
    // take until the first empty line
    let mut map_lines = vec![];
    for (y, line) in lines.iter().take_while(|line| !line.is_empty()).enumerate() {
        let chars = line.chars().enumerate().collect::<Vec<_>>();
        let mut i = 0;
        let mut x = 0;
        let mut line_vec = vec![];
        while i < chars.len() {
            let obj = Object::from_char(chars[i].1);
            if obj == Object::Robot {
                robot_pos = (x, y);
                line_vec.push(Object::Robot);
                line_vec.push(Object::Empty);
            }
            if obj == Object::Wall {
                line_vec.push(obj);
                line_vec.push(obj);
            }
            if obj == Object::Rock {
                line_vec.push(Object::BoxLeft);
                line_vec.push(Object::BoxRight);
            }
            if obj == Object::Empty {
                line_vec.push(Object::Empty);
                line_vec.push(Object::Empty);
            }
            i += 1;
            x += 2;
        }
        map_lines.push(line_vec);
    }
    // concat all movelines to single line
    let moves = lines
        .iter()
        .skip(map_lines.len() + 1)
        .flat_map(|line| line.chars())
        .map(|c| Move::from_char(c))
        .collect::<Vec<_>>();

    let mut eval_queue: VecDeque<(isize, isize)> = VecDeque::from([]);
    let mut to_move_stack: Vec<(isize, isize)> = vec![];

    for m in moves.iter() {
        eval_queue.clear();
        to_move_stack.clear();
        assert_print_map!(
            every_boxleft_has_boxright_next_to_it(&map_lines),
            &map_lines,
            "every boxleft has boxright next to it"
        );
        let direction = m.to_direction();

        let cur_pos = robot_pos;
        eval_queue.push_back((cur_pos.0 as isize, cur_pos.1 as isize));
        assert!(map_lines[eval_queue[0].1 as usize][eval_queue[0].0 as usize] == Object::Robot);
        while let Some(cur) = eval_queue.pop_front() {
            if to_move_stack.contains(&cur) {
                continue;
            }
            if *m == Move::Left {
                let cur_obj = map_lines[cur.1 as usize][cur.0 as usize];
                match cur_obj {
                    Object::BoxLeft => {
                        let on_left = (cur.0 - 1, cur.1);
                        if cant_push(&map_lines, on_left) {
                            eval_queue.clear();
                            to_move_stack.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, on_left, Object::Empty) {
                            continue;
                        }
                        if pos_is(&map_lines, on_left, Object::BoxRight) {
                            assert!(pos_is(
                                &map_lines,
                                (on_left.0 + 1, on_left.1),
                                Object::BoxLeft
                            ));
                            eval_queue.push_back((on_left.0 - 1, on_left.1));
                        }
                    }
                    Object::Robot => {
                        let on_left = (cur.0 - 1, cur.1);
                        if cant_push(&map_lines, on_left) {
                            eval_queue.clear();
                            to_move_stack.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, on_left, Object::Empty) {
                            continue;
                        }
                        if pos_is(&map_lines, on_left, Object::BoxRight) {
                            assert!(pos_is(
                                &map_lines,
                                (on_left.0 - 1, on_left.1),
                                Object::BoxLeft
                            ));
                            eval_queue.push_back((on_left.0 - 1, on_left.1));
                        }
                    }
                    _ => unreachable!(),
                }
            } else if *m == Move::Right {
                let cur_obj = map_lines[cur.1 as usize][cur.0 as usize];
                match cur_obj {
                    Object::BoxLeft => {
                        let on_right = (cur.0 + 2, cur.1);
                        if cant_push(&map_lines, on_right) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, on_right, Object::Empty) {
                            continue;
                        }
                        if pos_is(&map_lines, on_right, Object::BoxLeft) {
                            assert!(pos_is(
                                &map_lines,
                                (on_right.0 + 1, on_right.1),
                                Object::BoxRight
                            ));
                            eval_queue.push_back(on_right);
                        }
                    }
                    Object::Robot => {
                        let on_right = (cur.0 + 1, cur.1);
                        if cant_push(&map_lines, on_right) {
                            eval_queue.clear();
                            to_move_stack.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, on_right, Object::Empty) {
                            continue;
                        }
                        if pos_is(&map_lines, on_right, Object::BoxLeft) {
                            assert!(pos_is(
                                &map_lines,
                                (on_right.0 + 1, on_right.1),
                                Object::BoxRight
                            ));
                            eval_queue.push_back(on_right);
                        }
                    }
                    _ => unreachable!(),
                }
            } else if *m == Move::Up {
                let cur_obj = map_lines[cur.1 as usize][cur.0 as usize];
                match cur_obj {
                    Object::BoxLeft => {
                        let above_direct = (cur.0, cur.1 - 1);
                        let above_upright = (cur.0 + 1, cur.1 - 1);
                        if cant_push(&map_lines, above_direct) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        if cant_push(&map_lines, above_upright) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, above_direct, Object::Empty)
                            && pos_is(&map_lines, above_upright, Object::Empty)
                        {
                            continue;
                        }
                        if pos_is(&map_lines, above_direct, Object::BoxLeft) {
                            eval_queue.push_back(above_direct);
                        } else if pos_is(&map_lines, above_direct, Object::BoxRight) {
                            assert!(pos_is(
                                &map_lines,
                                (above_direct.0 - 1, above_direct.1),
                                Object::BoxLeft
                            ));
                            eval_queue.push_back((above_direct.0 - 1, above_direct.1));
                        }

                        if pos_is(&map_lines, above_upright, Object::BoxLeft) {
                            eval_queue.push_back(above_upright);
                        }
                    }
                    Object::Robot => {
                        let above = (cur.0, cur.1 - 1);
                        if cant_push(&map_lines, above) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, above, Object::BoxLeft) {
                            eval_queue.push_back(above);
                        } else if pos_is(&map_lines, above, Object::BoxRight) {
                            assert!(pos_is(&map_lines, (above.0 - 1, above.1), Object::BoxLeft));
                            eval_queue.push_back((above.0 - 1, above.1));
                        } else if pos_is(&map_lines, above, Object::Empty) {
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
            } else if *m == Move::Down {
                let cur_obj = map_lines[cur.1 as usize][cur.0 as usize];
                match cur_obj {
                    Object::BoxLeft => {
                        let below_direct = (cur.0, cur.1 + 1);
                        let below_downright = (cur.0 + 1, cur.1 + 1);
                        if cant_push(&map_lines, below_direct) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        if cant_push(&map_lines, below_downright) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, below_direct, Object::Empty) && pos_is(&map_lines, below_downright, Object::Empty) {
                            continue;
                        }
                        if pos_is(&map_lines, below_direct, Object::BoxLeft) {
                            eval_queue.push_back(below_direct);
                        } else if pos_is(&map_lines, below_direct, Object::BoxRight) {
                            assert!(pos_is(&map_lines, (below_direct.0 - 1, below_direct.1), Object::BoxLeft));
                            eval_queue.push_back((below_direct.0 - 1, below_direct.1));
                        }
                        if pos_is(&map_lines, below_downright, Object::BoxLeft) {
                            eval_queue.push_back(below_downright);
                        }
                    }
                    Object::Robot => {
                        let below = (cur.0, cur.1 + 1);
                        if cant_push(&map_lines, below) {
                            to_move_stack.clear();
                            eval_queue.clear();
                            break;
                        }
                        to_move_stack.push(cur);
                        if pos_is(&map_lines, below, Object::BoxLeft) {
                            eval_queue.push_back(below);
                        } else if pos_is(&map_lines, below, Object::BoxRight) {
                            assert!(pos_is(&map_lines, (below.0 - 1, below.1), Object::BoxLeft));
                            eval_queue.push_back((below.0 - 1, below.1));
                        } else if pos_is(&map_lines, below, Object::Empty) {
                            continue;
                        }
                    }
                    other => assert_print_map!(false, &map_lines, format!("invalid object in push_stack: {:?}, push_stack: {:?}, push_stack_types: {:?}", other, to_move_stack, to_move_stack.iter().map(|(x, y)| map_lines[*y as usize][*x as usize]).collect::<Vec<_>>())),
                }
            }
        }

        while let Some(pos) = to_move_stack.pop() {
            let obj = map_lines[pos.1 as usize][pos.0 as usize];
            match obj {
                Object::BoxLeft => {
                    if *m == Move::Left {
                        let empty_pos = (pos.0 - 1, pos.1);
                        assert!(map_lines[empty_pos.1 as usize][empty_pos.0 as usize] == Object::Empty, "target at {:?} is not empty {:?}", empty_pos, map_lines[empty_pos.1 as usize][empty_pos.0 as usize]);
                        map_lines[empty_pos.1 as usize][empty_pos.0 as usize] = Object::BoxLeft;
                        map_lines[pos.1 as usize][pos.0 as usize] = Object::BoxRight;
                        map_lines[pos.1 as usize][(pos.0 + 1) as usize] = Object::Empty;
                    } else if *m == Move::Right {
                        let empty_pos = (pos.0 + 2, pos.1);
                        assert!(map_lines[empty_pos.1 as usize][empty_pos.0 as usize] == Object::Empty, "target at {:?} is not empty {:?}", empty_pos, map_lines[empty_pos.1 as usize][empty_pos.0 as usize]);
                        map_lines[pos.1 as usize][(pos.0 + 1) as usize] = Object::BoxLeft;
                        map_lines[pos.1 as usize][(pos.0 + 2) as usize] = Object::BoxRight;
                        map_lines[pos.1 as usize][pos.0 as usize] = Object::Empty;
                    } else if *m == Move::Up {
                        let empty_pos = (pos.0, pos.1 - 1);
                        assert!(map_lines[empty_pos.1 as usize][empty_pos.0 as usize] == Object::Empty, "target at {:?} is not empty {:?}", empty_pos, map_lines[empty_pos.1 as usize][empty_pos.0 as usize]);
                        assert!(map_lines[empty_pos.1 as usize][(empty_pos.0 + 1) as usize] == Object::Empty, "target at {:?} is not empty {:?}", (empty_pos.0 + 1, empty_pos.1), map_lines[empty_pos.1 as usize][(empty_pos.0 + 1) as usize]);
                        map_lines[(pos.1 - 1) as usize][pos.0 as usize] = Object::BoxLeft;
                        map_lines[(pos.1 - 1) as usize][(pos.0 + 1) as usize] = Object::BoxRight;
                        map_lines[pos.1 as usize][pos.0 as usize] = Object::Empty;
                        map_lines[pos.1 as usize][(pos.0 + 1) as usize] = Object::Empty;
                    } else if *m == Move::Down {
                        let empty_pos = (pos.0, pos.1 + 1);
                        assert!(map_lines[empty_pos.1 as usize][empty_pos.0 as usize] == Object::Empty, "target at {:?} is not empty {:?}", empty_pos, map_lines[empty_pos.1 as usize][empty_pos.0 as usize]);
                        assert!(map_lines[(empty_pos.1) as usize][(empty_pos.0 + 1) as usize] == Object::Empty, "target at {:?} is not empty {:?}", (empty_pos.0, empty_pos.1 + 1), map_lines[(empty_pos.1 + 1) as usize][(empty_pos.0 + 1) as usize]);
                        map_lines[(pos.1 + 1) as usize][pos.0 as usize] = Object::BoxLeft;
                        map_lines[(pos.1 + 1) as usize][(pos.0 + 1) as usize] = Object::BoxRight;
                        map_lines[pos.1 as usize][pos.0 as usize] = Object::Empty;
                        map_lines[pos.1 as usize][(pos.0 + 1) as usize] = Object::Empty;
                    }
                }
                Object::Robot => {
                    map_lines[pos.1 as usize][pos.0 as usize] = Object::Empty;
                    let new_pos = (pos.0 as isize + direction.0, pos.1 as isize + direction.1);
                    if cant_push(&map_lines, new_pos) {
                        assert_print_map!(false, &map_lines, "robot moved out of bounds");
                    }
                    map_lines[new_pos.1 as usize][new_pos.0 as usize] = Object::Robot;
                    robot_pos = (new_pos.0 as usize, new_pos.1 as usize);
                }
                other => assert_print_map!(false, &map_lines, format!("invalid object in push_stack: {:?}, push_stack: {:?}, push_stack_types: {:?}", other, to_move_stack, to_move_stack.iter().map(|(x, y)| map_lines[*y as usize][*x as usize]).collect::<Vec<_>>())),
            }
        }
    }

    print_map(&map_lines);

    let mut gps = 0;
    for y in 0..map_lines.len() {
        for x in 0..map_lines[y].len() {
            if let Object::BoxLeft = map_lines[y][x] {
                gps += 100 * y + x
            }
        }
    }
    gps
}

fn cant_push(map_lines: &Vec<Vec<Object>>, pos: (isize, isize)) -> bool {
    !in_bounds(pos, map_lines) || map_lines[pos.1 as usize][pos.0 as usize] == Object::Wall
}

fn pos_is(map_lines: &Vec<Vec<Object>>, pos: (isize, isize), obj: Object) -> bool {
    map_lines[pos.1 as usize][pos.0 as usize] == obj
}

fn every_boxleft_has_boxright_next_to_it(map_lines: &Vec<Vec<Object>>) -> bool {
    for y in 0..map_lines.len() {
        for x in 0..map_lines[y].len() {
            if map_lines[y][x] == Object::BoxLeft {
                if !in_bounds((x as isize + 1, y as isize), map_lines)
                    || map_lines[y][x + 1] != Object::BoxRight
                {
                    return false;
                }
            }
            if map_lines[y][x] == Object::BoxRight {
                if !in_bounds((x as isize - 1, y as isize), map_lines)
                    || map_lines[y][x - 1] != Object::BoxLeft
                {
                    return false;
                }
            }
        }
    }
    true
}

fn print_map(map_lines: &Vec<Vec<Object>>) {
    for line in map_lines.iter() {
        println!("{}", line.iter().map(|o| o.to_char()).collect::<String>());
    }
}

fn in_bounds(pos: (isize, isize), map_lines: &Vec<Vec<Object>>) -> bool {
    pos.0 >= 0
        && pos.0 < map_lines[0].len() as isize
        && pos.1 >= 0
        && pos.1 < map_lines.len() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_day15_part1() {
        assert_eq!(
            day15_part1(Some(EXAMPLE.lines().map(|s| s.to_string()).collect())),
            10092
        );
    }

    #[test]
    fn test_day15_part2() {
        assert_eq!(
            day15_part2(Some(EXAMPLE.lines().map(|s| s.to_string()).collect())),
            9021
        );
    }
}
