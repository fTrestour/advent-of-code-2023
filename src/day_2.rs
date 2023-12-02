pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from)
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input.lines().map(Game::from).map(|game| game.power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_CASE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_CASE), 2286);
    }
}

struct Handful {
    blue: u32,
    green: u32,
    red: u32,
}

impl From<&str> for Handful {
    fn from(value: &str) -> Self {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for color in value.split(',') {
            let mut color = color.trim().split(' ');

            let number = color.next().unwrap().parse().unwrap();
            let color = color.next().unwrap();

            match color {
                "blue" => blue = number,
                "red" => red = number,
                "green" => green = number,
                _ => panic!("Invalid input"),
            };
        }

        Handful { blue, green, red }
    }
}

impl Handful {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut input = value.trim().split(':');

        let id = input
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .unwrap();
        let handfuls = input
            .next()
            .unwrap()
            .split(';')
            .map(Handful::from)
            .collect();

        Game { id, handfuls }
    }
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.handfuls.iter().all(|handful| handful.is_possible())
    }

    pub fn power(&self) -> u32 {
        let mut result = 1;

        for value in self.minimum_set_of_cubes() {
            if value > 0 {
                result *= value;
            }
        }

        result
    }

    fn minimum_set_of_cubes(&self) -> Vec<u32> {
        vec![
            self.handfuls.iter().map(|x| x.blue).max().unwrap(),
            self.handfuls.iter().map(|x| x.red).max().unwrap(),
            self.handfuls.iter().map(|x| x.green).max().unwrap(),
        ]
    }
}
