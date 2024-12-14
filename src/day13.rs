use crate::util::aoc_read_day_lines;

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct WinCondition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
struct Game {
    pub a: Button,
    pub b: Button,
    pub prize: WinCondition,
}

const MOVE_COST_A: usize = 3;
const MOVE_COST_B: usize = 1;

fn solve(game: &Game) -> Option<usize> {
    let a_x = game.a.x as f64;
    let a_y = game.a.y as f64;
    let b_x = game.b.x as f64;
    let b_y = game.b.y as f64;
    let prize_x = game.prize.x as f64;
    let prize_y = game.prize.y as f64;

    // Equations:
    // #1: n*aX + m*bX = prizeX
    // #2: n*aY + m*bY = prizeY

    // From equation #1:
    // n*aX + m*bX = prizeX
    // m*bX = prizeX - n*aX
    // #3: m = (prizeX - n*aX) / bX

    // Substitute #3 into #2:
    // n*aY + ((prizeX - n*aX) / bX) * bY = prizeY
    // n*aY + prizeX*bY/bX - n*aX*bY/bX = prizeY
    // n*(aY - aX*bY/bX) = prizeY - prizeX*bY/bX
    // n = (prizeY*bX - prizeX*bY) / (aY*bX - aX*bY)

    // Check if denominator is 0
    let denominator = a_y * b_x - a_x * b_y;
    if denominator == 0.0 {
        return None;
    }
    let numerator = prize_y * b_x - prize_x * b_y;

    let n = numerator / denominator;

    // Calculate m using equation (3)
    let m = (prize_x - n * a_x) / b_x;

    // Check if n and m are non-negative
    if n < 0.0 || m < 0.0 {
        return None;
    }

    // Verify n and m are integers
    if n % 1.0 != 0.0 || m % 1.0 != 0.0 {
        return None;
    }

    // Convert back to usize for final cost calculation
    let n = n as usize;
    let m = m as usize;
    Some(MOVE_COST_A * n + MOVE_COST_B * m)
}

pub fn day13_part1() -> usize {
    let lines = aoc_read_day_lines(13);
    let empty_lines_removed = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let mut games = Vec::new();
    // parse 3 lines at a time
    for i in (0..empty_lines_removed.len()).step_by(3) {
        games.push(parse_game(&empty_lines_removed[i..i + 3]));
    }
    games
        .iter()
        .filter_map(|g| solve(g))
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn day13_part2() -> usize {
    let lines = aoc_read_day_lines(13);
    let empty_lines_removed = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let mut games = Vec::new();
    // parse 3 lines at a time
    for i in (0..empty_lines_removed.len()).step_by(3) {
        games.push(parse_game(&empty_lines_removed[i..i + 3]));
    }
    for g in games.iter_mut() {
        g.prize.x += 10000000000000;
        g.prize.y += 10000000000000;
    }
    games
        .iter()
        .filter_map(|g| solve(g))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn parse_game(lines: &[&String]) -> Game {
    let button_a_line = lines[0];
    let button_b_line = lines[1];
    let win_condition_line = lines[2];
    let button_a = parse_button(button_a_line);
    let button_b = parse_button(button_b_line);
    let win_condition = parse_win_condition(win_condition_line);
    Game {
        a: button_a,
        b: button_b,
        prize: win_condition,
    }
}

fn parse_button(line: &str) -> Button {
    let after_colon = line.split(":").nth(1).unwrap();
    let xy = after_colon
        .split(",")
        .map(|s| {
            let s = s.trim();
            let amt = s
                .split('+')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            amt
        })
        .collect::<Vec<_>>();
    let x = xy[0];
    let y = xy[1];
    Button { x, y }
}

fn parse_win_condition(line: &str) -> WinCondition {
    let after_colon = line.split(":").nth(1).unwrap();
    let xy = after_colon
        .split(",")
        .map(|s| {
            let s = s.trim();
            let amt = s
                .split('=')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            amt
        })
        .collect::<Vec<_>>();
    let x = xy[0];
    let y = xy[1];
    WinCondition { x, y }
}
