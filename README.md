# My Bot Checker

[![Build Status](https://travis-ci.org/devactivity/my-bot-checker.svg?branch=main)](https://travis-ci.org/devactivity/my-bot-checker)

**My Bot Checker** is a Rust library for detecting bot user agents using regular expressions. It reads patterns from a JSON file, compiles them into a regex, and checks user agents against these patterns.

## Features

- **Bot Detection**: Identify whether a given user agent string matches known bot patterns.
- **Customizable**: Use your own bot patterns by providing a JSON file.
- **Efficient**: Uses the `pcre2` crate for high-performance regex matching.
- **Error Handling**: Robust error handling with detailed messages.

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
my_bot_checker = "0.1"
```

Ensure you also have the required dependencies:

```toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
pcre2 = "0.4"
once_cell = "1.9"
```

## Usage
### Initialize the Pattern

Before you can use the bot detection functions, you need to initialize the pattern by reading it from a JSON file:

```rust
use my_bot_checker::init_pattern;

fn main() {
    let regex = init_pattern("patterns.json").unwrap();
}
```

### Check if User Agent is a Bot

To check if a given user agent string matches any known bot patterns:

```rust
use my_bot_checker::is_bot;

fn main() {
    let is_bot = is_bot("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
    println!("Is bot: {}", is_bot);
}
```

### Find the Matching Bot Pattern

If you want to know which bot pattern matched the user agent:

```rust
use my_bot_checker::is_bot_match;

fn main() {
    let matched_pattern = is_bot_match("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
    if let Some(pattern) = matched_pattern {
        println!("Matched bot pattern: {}", pattern);
    } else {
        println!("No bot pattern matched.");
    }
}
```

## Patterns File Format

The bot patterns are stored in a JSON file, with each pattern being a regular expression string. Here is an example `patterns.json`:

```json
[
  " daum[ /]",
  " deusu/",
  " yadirectfetcher",
  "(?:^|[^g])news(?!sapphire)",
  "(?<! (?:channel/|google/))google(?!(app|/google| pixel))",
  "(?<! cu)bots?(?:\\b|_)"
]
```

Each string in the array is a pattern that will be compiled into a single regular expression to match against user agent strings.

## Running Tests

To run the tests, you can use the following command:

```bash
cargo test
```

This will also run the documentation tests to ensure that all code examples in the documentation are correct. The integration test is just an example usage, you can ignore it.

## Format/Linting

Ensure your code follows the Rust style guidelines by running:

```bash
cargo clippy -- -W clippy::pedantic
```

## Credits

This library was inspired by and largely based on the work from [isbot](https://github.com/omrilotan/isbot). The original JavaScript code was rewritten in Rust.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have ideas for improvements or found a bug.
