# retronix.rs

Retronix.rs is a simple [Xonix](https://en.wikipedia.org/wiki/Xonix) clone for the terminal, and my first voyage into [Rust](https://www.rust-lang.org).

For a more elaborate, cross-platform version in Java, see [http://retronix.czak.pl](http://retronix.czak.pl).

## How to run

```sh
$ git clone https://github.com/czak/retronix.rs.git
$ cd retronix.rs
$ cargo run
```

Tested with Rust stable 1.24.1.

## Screenshots

[Click here](https://czak.github.io/retronix.rs/index.html) for a moving picture.

![startup](https://raw.githubusercontent.com/czak/retronix.rs/assets/screenshots/startup.png)
![game](https://raw.githubusercontent.com/czak/retronix.rs/assets/screenshots/game.png)

## About the game

Use the cursor keys to control the player (the magenta block).
Your goal is to fill the board, while avoiding the enemies.
Fill 80% and you advance to the next level. Every level brings more enemies.
Enjoy!

## License

Copyright (c) 2018 Łukasz Adamczak

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
