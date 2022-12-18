// use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Clone, Debug, PartialEq)]
#[display(r"Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")]

struct SensorBeacon {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl SensorBeacon {
    fn radius(&self) -> i32 {
        (self.sensor_x.abs_diff(self.beacon_x) + self.sensor_y.abs_diff(self.beacon_y)) as i32
    }
    fn in_row(&self, row: i32) -> Option<i32> {
        let bottom = self.sensor_y + self.radius();
        let top = self.sensor_y - self.radius();
        // println!("self {} row {} bottom {} top {}", self, row, bottom, top);
        if row <= bottom && row >= top {
            // is 2r+1 in the middle, and 1 at the top.
            return Some((2 * self.radius() + 1) - (2 * (self.sensor_y.abs_diff(row) as i32)));
        }
        None
    }
    fn get_x_range(&self, row: i32) -> Option<(i32, i32)> {
        match self.in_row(row) {
            None => None,
            Some(_x) => {
                let xmin = self.sensor_x - self.radius() + self.sensor_y.abs_diff(row) as i32;
                let xmax = self.sensor_x + self.radius() - self.sensor_y.abs_diff(row) as i32;
                Some((xmin, xmax))
            }
        }
    }
}

fn test() {
    debug_assert!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
            .parse::<SensorBeacon>()
            .unwrap()
            == SensorBeacon {
                sensor_x: 2,
                sensor_y: 18,
                beacon_x: -2,
                beacon_y: 15
            }
    );
    let a = SensorBeacon {
        sensor_x: 8,
        sensor_y: 7,
        beacon_x: 2,
        beacon_y: 10,
    };
    debug_assert!(a.radius() == 9);
    debug_assert!(a.in_row(-2) == Some(1));
    debug_assert!(a.in_row(6) == Some(17));
    debug_assert!(a.in_row(7) == Some(19));
    debug_assert!(a.in_row(8) == Some(17));
    debug_assert!(a.in_row(16) == Some(1));
    debug_assert!(a.in_row(17).is_none());
    debug_assert!(a.get_x_range(0) == Some((6, 10)));
    debug_assert!(a.get_x_range(15) == Some((7, 9)));

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            10
        ) == 26
    );
}

fn part1(data: &str, row: i32) -> usize {
    let ranges = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<SensorBeacon>().unwrap())
        .filter_map(|x| x.get_x_range(row))
        .collect::<Vec<(i32, i32)>>();

    let mut v_beacons_on_row = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<SensorBeacon>().unwrap())
        .filter(|x| x.beacon_y == row)
        .map(|x| (x.beacon_x, x.beacon_y))
        .collect::<Vec<(i32, i32)>>();

    v_beacons_on_row.sort();
    v_beacons_on_row.dedup();
    let beacons_on_this_row = v_beacons_on_row.len();

    let mut v_sensors_on_row = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<SensorBeacon>().unwrap())
        .filter(|x| x.sensor_y == row)
        .map(|x| (x.sensor_x, x.sensor_y))
        .collect::<Vec<(i32, i32)>>();

    v_sensors_on_row.sort();
    v_sensors_on_row.dedup();
    let sensors_on_this_row = v_sensors_on_row.len();

    let xmin = ranges.iter().map(|x| x.0).min().unwrap();
    let xmax = ranges.iter().map(|x| x.1).max().unwrap();

    let xoffset = xmin.abs();
    let mut output = vec!['.'; xmax.abs_diff(xmin) as usize + 1];

    // println!("min: {} max: {} beaconsOnRow: {} ranges: {:?} output: {:?}",xmin,xmax,beacons_on_this_row, ranges,output);
    println!(
        "min: {} max: {} beacons/sensors OnRow: {}/{} sensorranges: {:?}",
        xmin, xmax, beacons_on_this_row, sensors_on_this_row, ranges
    );

    for range in ranges {
        for i in range.0..=range.1 {
            output[(i + xoffset) as usize] = '#';
        }
    }
    println!(
        "outlen: {} minus(Sens+Beacs): {}",
        output.len(),
        output.len() - (beacons_on_this_row + sensors_on_this_row)
    );

    output.len() - (beacons_on_this_row + sensors_on_this_row)
}

fn main() {
    test();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        2000000,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 5176944);
}

// See also.
// https://en.wikipedia.org/wiki/Taxicab_geometry
// https://en.wikipedia.org/wiki/Compressed_sensing
