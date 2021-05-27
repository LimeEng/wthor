![CI status](https://github.com/LimeEng/wthor/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/wthor.svg)](https://crates.io/crates/wthor)

# Wthor

Wthor is a rather archaic data-format used extensively by the [Fédération Française d'Othello](https://www.ffothello.org/informatique/la-base-wthor/) to document games of Othello. As a result of their massive collection, Wthor has become a popular data-format in this space. Unfortunately, the only [official documentation](https://www.ffothello.org/wthor/Format_WThor.pdf) on the format is both quite poorly described and in French, which I do not speak. After translating the page with Google Translate this library was written to extract the most interesting information.

The parsed data is difficult to name appropriately. The translated documentation is imprecise and the descriptions are sometimes hard to understand. That is why parameter `p1` is named `board_size` while parameter `p3` retains its name.

The information most relevant to Othello bots are the games themselves, which can be extracted with this library. The games are simply presented as a long number of moves but since players sometimes need to pass their turn, it is impossible to know which player played which move without evaluating the game, stepping through the moves one by one. This is also something this library can handle. The data extracted and optionally inferred can later be converted to more convenient and widely-supported formats.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wthor = "0.3"
```
