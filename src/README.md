# ccwc-rs
## A wc clone, written in Rust!
---
Written with the guidance of John Crickett's Coding Challenges at https://codingchallenges.fyi/.

### Counting Bytes
``` bash
$ ccwc-rs -c test.txt
    341836  test.txt
```

### Counting Lines
``` bash
$ ccwc-rs -l test.txt
    7137  test.txt
```

### Counting Words
`wc` counts words by checking for whitespace characters that are followed by a non-whitespace character.
``` bash
$ ccwc-rs -w test.text
    58159  test.txt
```

### Counting Chars
The number of chars may be different from the number of bytes if there are multibyte characters present in the text.
``` bash
$ ccwc-rs -m test.txt
    339120  test.txt
```

### Default Behavior
``` bash
$ ccwc-rs test.txt
    7137  58159  341836 test.txt
```

### Pipe from Stdin
``` bash
$ cat test.txt | ccwc-rs
    7137  58159  341836
```
