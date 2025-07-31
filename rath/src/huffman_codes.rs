/// See https://cs-214.epfl.ch/labs/huffman-coding/index.html for good explanation
///
use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::bit_vector::Bit;

#[derive(Eq, Debug)]
pub struct HuffNode {
    charaters: String,
    weight: u32,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn new_leaf(charaters: String, weight: u32) -> HuffNode {
        Self {
            charaters,
            weight,
            left: None,
            right: None,
        }
    }

    pub fn new_branch(
        charaters: String,
        weight: u32,
        left: Option<HuffNode>,
        right: Option<HuffNode>,
    ) -> HuffNode {
        Self {
            charaters,
            weight,
            left: left.map(|n| Box::new(n)),
            right: right.map(|n| Box::new(n)),
        }
    }
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.charaters == other.charaters
    }
}

/// Gets frequency count for each character
fn frequency_count(message: &str) -> HashMap<char, u32> {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for ch in message.chars() {
        counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
}

/// builds huffman tree from frequency counts
pub fn code_tree(counts: &HashMap<char, u32>) -> HuffNode {
    let mut priority_queue: BinaryHeap<Reverse<HuffNode>> = BinaryHeap::new();

    // Create a leaf node for each symbol, associated with a weight which denotes the frequency
    // of appearance of that symbol.
    for (ch, weight) in counts {
        priority_queue.push(Reverse(HuffNode {
            charaters: ch.to_string(),
            weight: weight.clone(),
            left: None,
            right: None,
        }));
    }

    // While there’s more than one code trees, removes the two trees with the lowest root
    // weight and merge them into a new tree by creating a new branching node.
    // A branching node can be thought of as a set containing the symbols present
    // in the leaves below it, with its weight being the total weight of those leaves.
    while priority_queue.len() > 1 {
        let n1: HuffNode = priority_queue.pop().unwrap().0;
        let n2: HuffNode = priority_queue.pop().unwrap().0;

        let (a, b) = if n1.weight == n2.weight {
            // order by character when weights are equal for consistency;
            if n1.charaters > n2.charaters {
                (n2, n1)
            } else {
                (n1, n2)
            }
        } else {
            (n1, n2)
        };

        let mut characters = a.charaters.clone();
        characters.push_str(&b.charaters);

        let branch = HuffNode {
            charaters: characters,
            weight: a.weight + b.weight,
            left: Some(Box::new(a)),
            right: Some(Box::new(b)),
        };

        priority_queue.push(Reverse(branch));
    }

    priority_queue.pop().unwrap().0
}

/// encde a single char using coding tree into {bits}
fn encode_char(ch: char, coding: &HuffNode, bits: &mut Vec<char>) {
    if coding.charaters.contains(ch) && coding.charaters.len() == 1 {
        return;
    } else if coding
        .left
        .as_ref()
        .map(|l| l.charaters.contains(ch))
        .unwrap_or(false)
    {
        bits.push('0');
        encode_char(ch, coding.left.as_ref().unwrap(), bits);
    } else if coding
        .right
        .as_ref()
        .map(|l| l.charaters.contains(ch))
        .unwrap_or(false)
    {
        bits.push('1');
        encode_char(ch, coding.right.as_ref().unwrap(), bits);
    } else {
        panic!("Invalid chracter: {}", ch)
    };
}

fn encode_char2(ch: char, coding: &HuffNode) -> Vec<Bit> {
    let mut bits: Vec<Bit> = Vec::new();

    let mut curr = coding;

    while !curr.is_leaf() {
        if curr
            .left
            .as_ref()
            .map(|l| l.charaters.contains(ch))
            .unwrap_or(false)
        {
            bits.push(Bit::Zero);
            curr = curr.left.as_ref().unwrap();
        } else if curr
            .right
            .as_ref()
            .map(|l| l.charaters.contains(ch))
            .unwrap_or(false)
        {
            bits.push(Bit::One);
            curr = curr.right.as_ref().unwrap();
        } else {
            panic!(
                "Invalid chracter: {}, {}, {}, {}, {}",
                ch,
                &curr.charaters,
                &curr.charaters.contains(ch),
                &curr.left.as_ref().unwrap().charaters,
                &curr.right.as_ref().unwrap().charaters
            )
        };
    }

    bits
}

pub fn encode_data2(data: &str, coding: &HuffNode) -> Vec<Bit> {
    data.chars()
        .into_iter()
        .flat_map(|c| encode_char2(c, coding))
        .collect()
}

pub fn encode_data(data: &str, coding: &HuffNode) -> Vec<char> {
    let chars: Vec<char> = data.chars().collect();

    chars
        .into_iter()
        .flat_map(|c| {
            let mut bits: Vec<char> = Vec::new();
            encode_char(c, coding, &mut bits);
            bits
        })
        .collect()
}

/// decodes bits into {out}
pub fn decode_bits<'a, I>(bits: I, coding: &HuffNode) -> String
where
    I: Iterator<Item = &'a Bit>,
{
    let mut curr = coding;
    let mut out: String = String::new();

    for bit in bits {
        if *bit == Bit::Zero {
            curr = curr.left.as_ref().unwrap();
        } else {
            curr = curr.right.as_ref().unwrap();
        }

        if curr.is_leaf() {
            println!("Decoding progress: {}", out);
            out.push_str(&curr.charaters.clone());
            // reset curr to start after leaf is found
            curr = coding;
        }
    }

    out
}

fn all_codes(data: &str, coding: &HuffNode) -> HashMap<String, String> {
    let chars: Vec<char> = data.chars().collect();

    chars
        .into_iter()
        .map(|c| {
            let mut bits: Vec<char> = Vec::new();
            encode_char(c, coding, &mut bits);
            let str_bits: String = bits.iter().collect();
            (String::from(c), str_bits)
        })
        .collect()
}

pub fn build(data: &str) -> HuffNode {
    let counts = frequency_count(&data);
    let tree = code_tree(&counts);
    tree
}

pub fn compression_ratio(data: &str, encoded: &Vec<char>) -> f64 {
    return data.as_bytes().len() as f64 / (encoded.len() as f64 / 8.0);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_frequency_count() {
        let result: HashMap<char, u32> = frequency_count("message");
        let mut expected: HashMap<char, u32> = HashMap::new();
        expected.insert('m', 1);
        expected.insert('e', 2);
        expected.insert('s', 2);
        expected.insert('a', 1);
        expected.insert('g', 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_frequency_count_empty_message() {
        let result: HashMap<char, u32> = frequency_count("");
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_code_tree_returns_head() {
        let mut counts: HashMap<char, u32> = HashMap::new();
        counts.insert('m', 1);
        counts.insert('e', 2);
        counts.insert('s', 2);
        counts.insert('a', 1);
        counts.insert('g', 1);

        let actual = code_tree(&counts);

        assert_eq!(actual.weight, 7);
        assert_eq!(
            HashSet::from_iter(actual.charaters.chars()),
            HashSet::from(['m', 'e', 's', 'a', 'g'])
        );
    }

    #[test]
    fn test_code_tree_correctness() {
        let mut counts: HashMap<char, u32> = HashMap::new();
        counts.insert('A', 5);
        counts.insert('B', 2);
        counts.insert('C', 1);
        counts.insert('D', 1);
        counts.insert('E', 2);

        let actual = code_tree(&counts);

        assert_eq!(actual.weight, 11);
        assert_eq!(
            HashSet::from_iter(actual.charaters.chars()),
            HashSet::from(['A', 'B', 'C', 'D', 'E'])
        );

        let left_a = actual.left.unwrap();
        assert_eq!(left_a.charaters, "A");
        assert_eq!(left_a.weight, 5);
        assert_eq!(left_a.left, None);
        assert_eq!(left_a.right, None);

        let bced = actual.right.unwrap();
        assert_eq!(
            HashSet::from_iter(bced.charaters.chars()),
            HashSet::from(['B', 'E', 'C', 'D'])
        );
        assert_eq!(bced.weight, 6);

        let cd = bced.left.unwrap();
        assert_eq!(
            HashSet::from_iter(cd.charaters.chars()),
            HashSet::from(['C', 'D'])
        );
        assert_eq!(cd.weight, 2);

        let be = bced.right.unwrap();
        assert_eq!(
            HashSet::from_iter(be.charaters.chars()),
            HashSet::from(['B', 'E'])
        );
        assert_eq!(be.weight, 4);

        let b = be.left.unwrap();
        assert_eq!(b.charaters, "B");
        assert_eq!(b.weight, 2);
        assert_eq!(b.left, None);
        assert_eq!(b.right, None);

        let e = be.right.unwrap();
        assert_eq!(e.charaters, "E");
        assert_eq!(e.weight, 2);
        assert_eq!(e.left, None);
        assert_eq!(e.right, None);

        let c = cd.left.unwrap();
        assert_eq!(c.charaters, "C");
        assert_eq!(c.weight, 1);
        assert_eq!(c.left, None);
        assert_eq!(c.right, None);

        let d = cd.right.unwrap();
        assert_eq!(d.charaters, "D");
        assert_eq!(d.weight, 1);
        assert_eq!(d.left, None);
        assert_eq!(d.right, None);
    }

    #[test]
    fn test_encode_char_single_node() {
        let tree = HuffNode::new_leaf(String::from("B"), 1);

        let mut bits: Vec<char> = Vec::new();

        encode_char('B', &tree, &mut bits);
        assert_eq!(bits, vec![]);
    }

    #[test]
    fn test_encode_char_multi_node() {
        let tree = HuffNode::new_branch(
            String::from("ABC"),
            5,
            Some(HuffNode::new_leaf(String::from("A"), 3)),
            Some(HuffNode::new_branch(
                String::from("BC"),
                2,
                Some(HuffNode::new_leaf(String::from("B"), 1)),
                Some(HuffNode::new_leaf(String::from("C"), 1)),
            )),
        );

        let mut bits: Vec<char> = Vec::new();
        encode_char('B', &tree, &mut bits);

        bits.clear();
        encode_char('C', &tree, &mut bits);
        assert_eq!(bits, vec!['1', '1']);

        bits.clear();
        encode_char('A', &tree, &mut bits);
        assert_eq!(bits, vec!['0']);
    }

    #[test]
    fn test_encode_char_big_tree() {
        let tree = HuffNode::new_branch(
            String::from("ABCDEFGH"),
            17,
            Some(HuffNode::new_leaf(String::from("A"), 8)),
            Some(HuffNode::new_branch(
                String::from("BCDEFGH"),
                9,
                Some(HuffNode::new_branch(
                    String::from("BCD"),
                    5,
                    Some(HuffNode::new_leaf(String::from("B"), 3)), /*B*/
                    Some(HuffNode::new_branch(
                        String::from("CD"),
                        2,
                        Some(HuffNode::new_leaf(String::from("C"), 1)),
                        Some(HuffNode::new_leaf(String::from("D"), 1)),
                    )), /*CD*/
                )), /*BCD*/
                Some(HuffNode::new_branch(
                    String::from("EFGH"),
                    4,
                    Some(HuffNode::new_branch(
                        String::from("EF"),
                        2,
                        Some(HuffNode::new_leaf(String::from("E"), 1)),
                        Some(HuffNode::new_leaf(String::from("F"), 1)),
                    )), /*EF*/
                    Some(HuffNode::new_branch(
                        String::from("GH"),
                        2,
                        Some(HuffNode::new_leaf(String::from("G"), 1)),
                        Some(HuffNode::new_leaf(String::from("H"), 1)),
                    )), /*GH*/
                )), /*EFGH*/
            )),
        );

        let mut bits: Vec<char> = Vec::new();
        encode_char('D', &tree, &mut bits);

        assert_eq!(bits, vec!['1', '0', '1', '1']);
    }

    #[test]
    fn test_decode_bits_big_tree() {
        let tree = HuffNode::new_branch(
            String::from("ABCDEFGH"),
            17,
            Some(HuffNode::new_leaf(String::from("A"), 8)),
            Some(HuffNode::new_branch(
                String::from("BCDEFGH"),
                9,
                Some(HuffNode::new_branch(
                    String::from("BCD"),
                    5,
                    Some(HuffNode::new_leaf(String::from("B"), 3)), /*B*/
                    Some(HuffNode::new_branch(
                        String::from("CD"),
                        2,
                        Some(HuffNode::new_leaf(String::from("C"), 1)),
                        Some(HuffNode::new_leaf(String::from("D"), 1)),
                    )), /*CD*/
                )), /*BCD*/
                Some(HuffNode::new_branch(
                    String::from("EFGH"),
                    4,
                    Some(HuffNode::new_branch(
                        String::from("EF"),
                        2,
                        Some(HuffNode::new_leaf(String::from("E"), 1)),
                        Some(HuffNode::new_leaf(String::from("F"), 1)),
                    )), /*EF*/
                    Some(HuffNode::new_branch(
                        String::from("GH"),
                        2,
                        Some(HuffNode::new_leaf(String::from("G"), 1)),
                        Some(HuffNode::new_leaf(String::from("H"), 1)),
                    )), /*GH*/
                )), /*EFGH*/
            )),
        );

        assert_eq!(
            decode_bits(vec![Bit::Zero].iter(), &tree),
            String::from("A")
        );
        assert_eq!(
            decode_bits(vec![Bit::One, Bit::Zero, Bit::Zero].iter(), &tree),
            String::from("B")
        );

        assert_eq!(
            decode_bits(
                vec![
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero
                ]
                .iter(),
                &tree,
            ),
            String::from("BAC")
        );
    }

    #[test]
    fn test_full() {
        let data = "BAC";
        let tree = build(data);

        let encoded: Vec<Bit> = encode_data2(data, &tree);

        let decoded = decode_bits(encoded.iter(), &tree);

        assert_eq!(decoded, data);
    }
}
