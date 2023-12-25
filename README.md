# Sanamahti

Sanamahti is a program that plays the popular Sanajahti game (the game is called Wordz in English).
It finds and returns all possible words from the given 4x4 grid
that appear on its wordlist.

## Wordlist
The [finnish wordlist](./wordlist_fin.txt) provided, is a derivative work (words that contains symobols are removed)
of the [Nykysuomen sanalista](https://www.kotus.fi/aineistot/sana-aineistot/nykysuomen_sanalista) licensed under the
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license. The original wordlist is available for download
[here](https://kaino.kotus.fi/lataa/nykysuomensanalista2022.csv)

## License
All the files apart from the wordlist are licensed under the MIT license.
For more information see the [LICENSE](./LICENSE)-file

## Implementation
[/src/main.rs](/src/main.rs) contains an example usage of the interface provided by [/src/lib.rs](/src/lib.rs).

[/src/lib.rs](/src/lib.rs) implements a `LetterTree` datatype that is constructed on runtime from the wordlist.
Each node in the tree represents a letter that is a part of one or more words. If the node is the last letter of
a word, then it's field `is_word` is set to true.

Additionally a `solve`-function is implemented. The solve function performs a BFS starting from each tile on
the grid and returns all the words found. The `LetterTree` datatype allows the BFS to be terminated early for
a branch that cannot produce a valid word.

## Ideas for further development
* Allow grids of artificial size
* Implementation for languages other than Finnish.
