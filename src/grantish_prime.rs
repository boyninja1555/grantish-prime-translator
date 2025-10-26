use std::collections::HashMap;

type GrantishPrimeKey = HashMap<char, char>;

pub fn english_grantish_prime() -> GrantishPrimeKey {
    let mut key: GrantishPrimeKey = HashMap::new();
    key.insert(' ', '_');
    key.insert('a', '@');
    key.insert('b', 'ß');
    key.insert('c', '©');
    key.insert('d', 'ⓓ');
    key.insert('e', 'ⓔ');
    key.insert('f', '#');
    key.insert('g', '*');
    key.insert('h', '&');
    key.insert('i', '!');
    key.insert('j', ';');
    key.insert('k', '.');
    key.insert('l', '😏');
    key.insert('m', '/');
    key.insert('n', '(');
    key.insert('o', '\\');
    key.insert('p', 'ℙ');
    key.insert('q', '?');
    key.insert('r', '℟');
    key.insert('s', '$');
    key.insert('t', '₮');
    key.insert('u', '😭');
    key.insert('v', '^');
    key.insert('w', '😎');
    key.insert('x', ')');
    key.insert('y', '%');
    key.insert('z', 'z');
    key.insert('1', '1');
    key.insert('2', '2');
    key.insert('3', '3');
    key.insert('4', '4');
    key.insert('5', '5');
    key.insert('6', '6');
    key.insert('7', '7');
    key.insert('8', '8');
    key.insert('9', '9');
    key.insert('0', '0');
    key
}

pub fn grantish_prime_english() -> GrantishPrimeKey {
    let mut key: GrantishPrimeKey = HashMap::new();
    key.insert('_', ' ');
    key.insert('@', 'a');
    key.insert('ß', 'b');
    key.insert('©', 'c');
    key.insert('ⓓ', 'd');
    key.insert('ⓔ', 'e');
    key.insert('#', 'f');
    key.insert('*', 'g');
    key.insert('&', 'h');
    key.insert('!', 'i');
    key.insert(';', 'j');
    key.insert('.', 'k');
    key.insert('😏', 'l');
    key.insert('/', 'm');
    key.insert('(', 'n');
    key.insert('\\', 'o');
    key.insert('ℙ', 'p');
    key.insert('?', 'q');
    key.insert('℟', 'r');
    key.insert('$', 's');
    key.insert('₮', 't');
    key.insert('😭', 'u');
    key.insert('^', 'v');
    key.insert('😎', 'w');
    key.insert(')', 'x');
    key.insert('%', 'y');
    key.insert('z', 'z');
    key.insert('1', '1');
    key.insert('2', '2');
    key.insert('3', '3');
    key.insert('4', '4');
    key.insert('5', '5');
    key.insert('6', '6');
    key.insert('7', '7');
    key.insert('8', '8');
    key.insert('9', '9');
    key.insert('0', '0');
    key
}
