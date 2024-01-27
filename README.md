# Overview:

A minimal [brainfuck](https://brainfuck.org/brainfuck.html) interpreter in Rust. It was built purely for fun.

> [!NOTE]
> Brainfuck, language itself, is a Turing-complete language created by Urban Müller. The language only consists of 8 operators, yet with the 8 operators, `<>+-[],.` you are capable of writing almost any program you can think of.

To learn more:
- [Quick Intro](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
- [Brainfuck Wiki](https://brainfuck.org/brainfuck.html)

### Getting started:
- Clone this repo and `cd` into it:
```bash
git clone https://github.com/PraneshASP/brainfuck-rs.git && cd brainfuck-rs
```

- Run example:
```bash
cargo run ./examples/hello-world.bf
```

### Example:

```bash
$ > cargo run ./examples/sierpinski.bf

                               *
                              * *
                             *   *
                            * * * *
                           *       *
                          * *     * *
                         *   *   *   *
                        * * * * * * * *
                       *               *
                      * *             * *
                     *   *           *   *
                    * * * *         * * * *
                   *       *       *       *
                  * *     * *     * *     * *
                 *   *   *   *   *   *   *   *
                * * * * * * * * * * * * * * * *

```