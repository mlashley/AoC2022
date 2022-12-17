use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Air,
    Rock,
    ActiveSand,
    DeadSand,
}
#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

struct Grid {
    g: Vec<Vec<Element>>,
    xoffset: usize,
}

impl Grid {
    fn draw_rock_line(&mut self, line: &Line) {
        if line.start.x == line.end.x {
            let y1 = line.start.y.min(line.end.y) as usize;
            let y2 = line.start.y.max(line.end.y) as usize;
            let x1 = line.start.x as usize - self.xoffset as usize;
            for y in y1..=y2 {
                self.g[y][x1] = Element::Rock;
            }
        } else if line.start.y == line.end.y {
            let x1 = line.start.x.min(line.end.x) as usize - self.xoffset as usize;
            let x2 = line.start.x.max(line.end.x) as usize - self.xoffset as usize;
            let y1 = line.start.y as usize;
            for x in x1..=x2 {
                self.g[y1][x] = Element::Rock;
            }
        }
    }
    fn draw_point(&mut self, point: Point, e: Element) {
        self.g[point.y][point.x - self.xoffset] = e;
    }
    fn next_move(&self, point: Point) -> Option<Point> {
        if point.x - self.xoffset == 0
            || point.x + 1 - self.xoffset >= self.g[0].len()
            || point.y + 1 >= self.g.len()
        {
            println!("Falling off the edge");
            return Some(Point {
                x: std::usize::MAX,
                y: std::usize::MAX,
            });
        } else if self.g[point.y + 1][point.x - self.xoffset] == Element::Air {
            return Some(Point {
                x: point.x,
                y: point.y + 1,
            });
        } else if self.g[point.y + 1][point.x - 1 - self.xoffset] == Element::Air {
            return Some(Point {
                x: point.x - 1,
                y: point.y + 1,
            });
        } else if self.g[point.y + 1][point.x + 1 - self.xoffset] == Element::Air {
            return Some(Point {
                x: point.x + 1,
                y: point.y + 1,
            });
        }
        None
    }
    fn add_sand(&mut self) -> bool {
        let mut sand = Point { x: 500, y: 0 };
        let mut nextpoint = self.next_move(sand);

        while nextpoint.is_some() {
            if nextpoint.unwrap().x == std::usize::MAX {
                return false;
            }
            sand = nextpoint.unwrap();
            nextpoint = self.next_move(nextpoint.unwrap());
        }
        self.draw_point(sand, Element::DeadSand);
        true
    }
    fn count_sand(&self) -> u32 {
        let mut c = 0;
        for row in &self.g {
            for elem in row {
                if *elem == Element::DeadSand {
                    c += 1
                }
            }
        }
        c
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("");
        for row in &self.g {
            for e in row {
                match e {
                    Element::Air => s.push('.'),
                    Element::Rock => s.push('#'),
                    Element::ActiveSand => s.push('+'),
                    Element::DeadSand => s.push('o'),
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn from_string(s: &str) -> Self {
        let a: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
        Point { x: a[0], y: a[1] }
    }
}

fn part1(data: &str) -> u32 {
    // Parse lines
    let v_lines_points = data
        .split("\n")
        .map(|x| {
            x.split(" -> ")
                .filter(|y| !y.is_empty())
                .map(Point::from_string)
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let lines = v_lines_points
        .iter()
        .map(|x| {
            let mut last = None;
            let mut v = Vec::new();
            for y in x {
                match last {
                    Some(x) => {
                        v.push(Line { start: x, end: *y });
                        last = Some(*y)
                    }
                    None => last = Some(*y),
                }
            }
            v
        })
        .collect::<Vec<Vec<Line>>>();

    let maxy = lines
        .iter()
        .flatten()
        .map(|a| a.start.y.max(a.end.y))
        .max()
        .unwrap();
    let miny = 0; // We always have the sand-emitter in view
    let maxx = lines
        .iter()
        .flatten()
        .map(|a| a.start.x.max(a.end.x))
        .max()
        .unwrap();
    let minx = lines
        .iter()
        .flatten()
        .map(|a| a.start.x.min(a.end.x))
        .min()
        .unwrap();
    let width = (maxx - minx) + 1;
    let height = (maxy - miny) + 1;
    println!("min x,y => max x,y: {},{} => {},{}", minx, miny, maxx, maxy);
    println!("w: {} h: {}", width, height);

    // Build empty grid
    let mut grid: Grid = Grid {
        g: Vec::new(),
        xoffset: minx,
    };
    for _row in miny..=maxy {
        grid.g.push(vec![Element::Air; width.try_into().unwrap()]);
    }

    // Draw Rocks
    for row in lines {
        for line in row {
            grid.draw_rock_line(&line);
        }
    }

    // Go Sand
    while grid.add_sand() {
        println!("{}", grid);
    }
    grid.count_sand()
}

fn test() {
    debug_assert!(
        24 == part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        )
    );
}

fn main() {
    test();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 578);
}
