use std::env;

type InputType = String;
type Boxes = Vec<Vec<Lens>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn hash(string: &str) -> u32 {
    string.chars().fold(0, |curr_value, chr| {
        let ascii_val = chr as u32;
        ((curr_value + ascii_val) * 17) % 256
    })
}

fn part1(lines: &InputType) -> u32 {
    lines.split(",").map(hash).sum()
}

fn remove_lens(boxes: &mut Boxes, label: &String) {
    let key = hash(label);
    let b: &mut Vec<Lens> = boxes.get_mut(key as usize).unwrap();
    if let Some((idx, _)) = b.iter().enumerate().find(|(_, x)| x.label == *label) {
        b.remove(idx);
    }
}

fn add_or_update_lens(boxes: &mut Boxes, lens: Lens) {
    let key = hash(&lens.label);
    let b: &mut Vec<Lens> = boxes.get_mut(key as usize).unwrap();
    if let Some(existing) = b.iter_mut().find(|x| x.label == lens.label) {
        existing.focal_length = lens.focal_length
    } else {
        b.push(lens);
    }
}

fn part2(lines: &InputType) -> u32 {
    let mut boxes = Boxes::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for entry in lines.split(",") {
        if let Some(label) = entry.strip_suffix("-") {
            remove_lens(&mut boxes, &label.to_string());
        } else {
            let (label, focal_len) = entry.split_once("=").unwrap();
            add_or_update_lens(
                &mut boxes,
                Lens {
                    label: label.to_string(),
                    focal_length: focal_len.parse::<u32>().unwrap(),
                },
            );
        }
    }
    let mut score = 0;
    for (box_idx, box_contents) in boxes.iter().enumerate() {
        for (lens_idx, lens) in box_contents.iter().enumerate() {
            score += (box_idx + 1) as u32 * (lens_idx + 1) as u32 * lens.focal_length
        }
    }
    score
}

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = read_file_lines(filename);
    println!("Part1: {}", part1(&lines[0]));
    println!("Part2: {}", part2(&lines[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    type ExampleWithExpected = (InputType, u32);

    fn get_part1_examples() -> Vec<ExampleWithExpected> {
        let examples = vec![
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];
        examples
            .into_iter()
            .map(|(example, expected)| (example.to_string(), expected))
            .collect()
    }

    fn get_part2_expected() -> Vec<ExampleWithExpected> {
        vec![]
    }

    #[test]
    fn test_part1_e2e() {
        let examples = get_part1_examples();
        for (example, expected) in examples.into_iter() {
            assert_eq!(part1(&example), expected,);
        }

        assert_eq!(
            part1(&"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()),
            1320
        );
    }

    #[test]
    fn test_boxes_insert() {
        let mut boxes = Boxes::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::new());
        }
        let label = "rn".to_string();
        add_or_update_lens(
            &mut boxes,
            Lens {
                label: label.clone(),
                focal_length: 1,
            },
        );
        assert_eq!(
            boxes[hash(&label) as usize]
                .iter()
                .map(|lens| (lens.label.as_str(), lens.focal_length))
                .collect::<Vec<_>>(),
            vec![("rn", 1)],
        );
        add_or_update_lens(
            &mut boxes,
            Lens {
                label: "cm".to_string(),
                focal_length: 2,
            },
        );
        assert_eq!(
            boxes[hash(&label) as usize]
                .iter()
                .map(|lens| (lens.label.as_str(), lens.focal_length))
                .collect::<Vec<_>>(),
            vec![("rn", 1), ("cm", 2)],
        );
        add_or_update_lens(
            &mut boxes,
            Lens {
                label: label.clone(),
                focal_length: 3,
            },
        );
        assert_eq!(
            boxes[hash(&label) as usize]
                .iter()
                .map(|lens| (lens.label.as_str(), lens.focal_length))
                .collect::<Vec<_>>(),
            vec![("rn", 3), ("cm", 2)],
        );
    }

    #[test]
    fn test_boxes_remove() {
        let mut boxes = Boxes::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::new());
        }
        for (idx, val) in vec!["ot", "ab", "pc"].iter().enumerate() {
            let label = val.to_string();
            remove_lens(&mut boxes, &label); // verify this doesn't crash anything
            add_or_update_lens(
                &mut boxes,
                Lens {
                    label: label,
                    focal_length: idx as u32,
                },
            );
        }
        remove_lens(&mut boxes, &"ab".to_string());
        let key = hash(&"ot".to_string()) as usize;
        assert_eq!(
            boxes[key]
                .iter()
                .map(|lens| (lens.label.as_str(), lens.focal_length))
                .collect::<Vec<_>>(),
            vec![("ot", 0), ("pc", 2)],
        );
        remove_lens(&mut boxes, &"ot".to_string());
        assert_eq!(
            boxes[key]
                .iter()
                .map(|lens| (lens.label.as_str(), lens.focal_length))
                .collect::<Vec<_>>(),
            vec![("pc", 2)],
        );
    }

    #[test]
    fn test_part2_e2e() {
        assert_eq!(
            part2(&"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()),
            145
        );
    }
}
