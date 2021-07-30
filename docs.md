# bzzz Documentation

## Executable Documentation
Switches:
- `--help | -h` :
  <br>
  Prints the help menu.
- `--encode | -e [filename, ...]` :
  <br>
  Performs a text-to-bee-speak conversion on the text of any number of files.
  If no files are given, input is taken from the standard input stream instead.
- `--decode | -d [filename, ...]` :
  <br>
  Performs a bee-speak-to-text conversion on the text of any number of files.
  If no files are given, input is taken from the standard input stream instead.

## Encoding Specification
The Bee-Speak encoding is isomorphic to a list of bytes. Bee-Speak is composed
of space-separated units, comprised of a `b` followed by between 1 and 16 `z`s.
Such units shall, for brevity sake, be referred to as "words". Two adjacent
words shall be called a "phrase". A Bee-Speak string is composed of any number
of phrases. Each phrase in a Bee-Speak string corresponds directly with a
single byte. A byte's relationship with a phrase shall be a derivation of that
byte's component higher nybble and lower nybble's relationships with that
phrase's former word and latter word, respectively. A nybble's relationship to
a word is as such: the value of the nybble shall be one less than the number of
`z`s in the word. Inversely, the number of `z`s in the word shall be one
greater than the value of the nybble.

A well-formed Bee-Speak string is one which, being interpreted as the
corresponding string of bytes, is also a well-formed utf8 string.

An example conversion is as follows:
`A` -> `0x41` -> `bzzzzz bzz`

