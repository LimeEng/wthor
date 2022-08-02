# Examples

## Extract information

```
cargo run --example serde extract_info
```

Running this example will display what tournaments a specific player participated in. It specifically showcases the relationship between the different types of files this library can parse. 

## Count wins by color

```
cargo run --example serde wins_by_color
```

This example counts the number of wins by color and displays the result. It uses an external Othello library to step through the moves in each game to determine the winner.

## Serde

```
cargo run --example serde --features serde
```

Serialization with [Serde](https://serde.rs/) can be enabled with the feature flag `serde`. This example demonstrates how serde can be used to serialize and deserialize the structures returned from the library.
