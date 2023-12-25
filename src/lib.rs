#![feature(lazy_cell)]

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::LazyLock;

static WORDS: LazyLock<LetterTree> = LazyLock::new(|| {
    let wordfile = File::open("wordlist_fin.txt").expect("Failed to open wordlist");
    let mut tree = LetterTree::new();
    BufReader::new(wordfile)
        .lines()
        .map_while(Result::ok)
        .for_each(|w| {
            tree.insert(
                &w.to_lowercase()
                    .chars()
                    // NOTE: skip the BOM
                    .skip_while(|c| *c == '\u{feff}')
                    .collect::<String>(),
            );
        });
    tree
});

/// Represents a node of a tree. Apart from the root node, each node represents one character
/// in a word.
///
/// Each path in the tree that ends in a node that has the [`LetterTree::is_word`]-field set to [`true`]
/// represents a word.
///
/// New words can be inserted with the [`LetterTree::insert`]-method.
///
/// The [`LetterTree::word_status`]-method is used get the [`Status`] of a specific word.
#[derive(Debug, Clone, Default)]
pub struct LetterTree {
    /// The character represented by the node
    pub value: Option<char>,
    /// Whether or not the node is the last letter of a valid word.
    pub is_word: bool,
    /// The children of the node ie. all the possible continuations for the path that produce a valid word
    pub leaves: Vec<LetterTree>,
}

/// Represents the status for a word (path) in the [`LetterTree`]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// The path produces a valid word
    Word,
    /// The path doesn't produce a valid word but it is a part of
    /// one or more valid words
    Possible,
    /// The path doesn't produce a valid word and there are no continuations
    /// for it that do.
    Impossible,
}

fn neighbours(pos: (i32, i32)) -> Vec<(i32, i32)> {
    (-1..=1)
        .flat_map(|xd| {
            (-1..=1)
                .map(|yd| (pos.0 + xd, pos.1 + yd))
                .collect::<Vec<_>>()
        })
        .filter(|(x, y)| *x >= 0 && *x < 4 && *y >= 0 && *y < 4)
        .collect::<Vec<_>>()
}

impl LetterTree {
    /// Construct a new node.
    pub fn new() -> Self {
        Self {
            value: None,
            is_word: false,
            leaves: Vec::new(),
        }
    }

    /// Insert a new word to the tree.
    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

        let mut word = word.chars().peekable();
        let letter = word.next().expect("Word is nonempty");

        let leaf = if let Some(leaf) = self.leaves.iter_mut().find(|l| l.value == Some(letter)) {
            leaf
        } else {
            self.leaves.push(LetterTree {
                value: Some(letter),
                is_word: word.peek().is_none(),
                leaves: Vec::new(),
            });
            self.leaves
                .last_mut()
                .expect("Post-push leaves is nonempty")
        };

        leaf.insert(&word.collect::<String>());
    }

    /// Get the [`Status`] of a specific word in the tree.
    pub fn word_status(&self, word: &str) -> Status {
        let mut word = word.chars();

        let Some(letter) = word.next() else {
            if self.is_word {
                return Status::Word;
            } else if !self.leaves.is_empty() {
                return Status::Possible;
            }
            return Status::Impossible;
        };

        if let Some(leaf) = self.leaves.iter().find(|l| l.value == Some(letter)) {
            leaf.word_status(&word.collect::<String>())
        } else {
            Status::Impossible
        }
    }
}

/// Get all possible words that can be represented as
/// a path on the given 4x4 `grid`.
///
/// Panics if the grid size is not 4x4.
///
/// Internally a BFS is performed, starting from each tile
/// on the grid. The BFS is terminated early for a branch
/// if there [`LetterTree::word_status`] returns [`Status::Impossible`]
/// for that word.
///
/// Returns a [`Vec<String>`] of the found words that is sorted
/// by word length in ascending order.
pub fn solve(grid: Vec<Vec<char>>) -> Vec<String> {
    assert!(grid.len() == 4, "Invalid grid size");
    assert!(grid[0].len() == 4, "Invalid grid size");

    let mut s = (0..4)
        .flat_map(|x| (0..4).map(|y| (x, y)).collect::<Vec<_>>())
        .map(|c| (c, vec![c]))
        .collect::<VecDeque<_>>();
    let mut found = HashSet::new();

    while let Some((pos, path)) = s.pop_front() {
        let word = path
            .iter()
            .map(|(x, y)| grid[(*y) as usize][(*x) as usize])
            .collect::<String>();

        match WORDS.word_status(&word) {
            Status::Word => {
                if word.len() > 2 {
                    found.insert(word);
                }
            }
            Status::Impossible => {
                continue;
            }
            Status::Possible => {}
        }

        neighbours(pos).iter().for_each(|n| {
            if !path.contains(n) {
                let mut newpath = path.clone();
                newpath.push(*n);
                s.push_back((*n, newpath));
            }
        });
    }

    let mut found_vec = found.into_iter().collect::<Vec<_>>();
    found_vec.sort_by_key(|w| w.chars().count());
    found_vec
}
