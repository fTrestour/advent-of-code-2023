pub fn solve_part1(input: &str) -> u64 {
    input.replace('\n', "").split(',').map(hash).sum()
}

fn hash(s: &str) -> u64 {
    let mut result = 0;
    for c in s.chars().map(|c| c as u64) {
        result = ((result + c) * 17) % 256
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(
            solve_part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        )
    }
}
