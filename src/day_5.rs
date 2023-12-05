use std::ops::Range;

pub fn solve_part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut lines = lines.skip(1);

    let mut category_maps = vec![];
    while let Some(header) = lines.next() {
        let mut header = header.trim().strip_suffix(" map:").unwrap().split("-to-");
        let source = header.next().unwrap().to_owned();
        let destination = header.next().unwrap().to_owned();

        let mut mappings = vec![];
        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                break;
            }

            let mut mapping = line.split(' ').map(|s| s.parse::<u64>().unwrap());

            mappings.push(Mapping::new(
                mapping.next().unwrap(),
                mapping.next().unwrap(),
                mapping.next().unwrap(),
            ));
        }

        category_maps.push(CategoryMap {
            source,
            destination,
            mappings,
        })
    }

    let almanac = Almanac {
        seeds,
        category_maps,
    };

    almanac.lowest_location()
}
struct Almanac {
    seeds: Vec<u64>,
    category_maps: Vec<CategoryMap>,
}

impl Almanac {
    pub fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .filter_map(|seed| self.locate(seed.clone()))
            .min()
            .unwrap()
    }

    fn locate(&self, seed: u64) -> Option<u64> {
        let mut category = "seed";
        let mut id = seed;

        while category != "location" {
            let category_map = self.find_category_map(category);

            if let Some(category_map) = category_map {
                id = category_map.map(id);
                category = &category_map.destination;
            } else {
                return None;
            }
        }

        Some(id)
    }

    fn find_category_map(&self, source: &str) -> Option<&CategoryMap> {
        self.category_maps
            .iter()
            .find(|mapping| mapping.source == source)
    }
}
struct CategoryMap {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

impl CategoryMap {
    pub fn map(&self, id: u64) -> u64 {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.maps_to(id))
            .unwrap_or(id)
    }
}

impl CategoryMap {}
struct Mapping {
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

impl Mapping {
    pub fn new(
        destination_range_start: u64,
        source_range_start: u64,
        range_length: u64,
    ) -> Mapping {
        Mapping {
            destination_range: Range {
                start: destination_range_start,
                end: destination_range_start + range_length,
            },
            source_range: Range {
                start: source_range_start,
                end: source_range_start + range_length,
            },
        }
    }

    pub fn maps_to(&self, id: u64) -> Option<u64> {
        if !self.source_range.contains(&id) {
            return None;
        }

        let offset = id - self.source_range.start;
        Some(self.destination_range.start + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_case = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!(solve_part1(test_case), 35);
    }
}
