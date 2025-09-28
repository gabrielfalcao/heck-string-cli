# heck-string-cli

command-line tool to apply [heck](https://crates.io/crates/heck) convert case of strings `--to`:
 - `kebab`
 - `camel`
 - `pascal`
 - `shouty-kebab`
 - `shouty-snake`
 - `snake`
 - `train`

## Installation

```shell
cargo install heck-string-cli
```

## Usage

### `heck-string --to=kebab`

Convert string to kebab case

```shell
$ heck-string --to=kebab "exampleToCamelCase"
example-to-kebab-case

$ heck-string --to=kebab "ExampleToPascalCase"
example-to-kebab-case

$ heck-string --to=kebab "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
example-to-kebab-case

$ heck-string --to=kebab "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
example-to-kebab-case

$ heck-string --to=kebab "example_to_snake_case"
example-to-kebab-case

$ heck-string --to=kebab "Example-To-Train-Case"
example-to-kebab-case
```

### `heck-string --to=camel`

Convert string to camel case

```shell
$ heck-string --to=camel "example-to-kebab-case"
exampleToCamelCase

$ heck-string --to=camel "ExampleToPascalCase"
exampleToCamelCase

$ heck-string --to=camel "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
exampleToCamelCase

$ heck-string --to=camel "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
exampleToCamelCase

$ heck-string --to=camel "example_to_snake_case"
exampleToCamelCase

$ heck-string --to=camel "Example-To-Train-Case"
exampleToCamelCase
```

### `heck-string --to=pascal`

Convert string to pascal case

```shell
$ heck-string --to=pascal "example-to-kebab-case"
ExampleToPascalCase

$ heck-string --to=pascal "exampleToCamelCase"
ExampleToPascalCase

$ heck-string --to=pascal "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
ExampleToPascalCase

$ heck-string --to=pascal "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
ExampleToPascalCase

$ heck-string --to=pascal "example_to_snake_case"
ExampleToPascalCase

$ heck-string --to=pascal "Example-To-Train-Case"
ExampleToPascalCase
```

### `heck-string --to=shouty-kebab`

Convert string to shouty-kebab case

```shell
$ heck-string --to=shouty-kebab "example-to-kebab-case"
EXAMPLE-TO-SHOUTY-KEBAB-CASE

$ heck-string --to=shouty-kebab "exampleToCamelCase"
EXAMPLE-TO-SHOUTY-KEBAB-CASE

$ heck-string --to=shouty-kebab "ExampleToPascalCase"
EXAMPLE-TO-SHOUTY-KEBAB-CASE

$ heck-string --to=shouty-kebab "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
EXAMPLE-TO-SHOUTY-KEBAB-CASE

$ heck-string --to=shouty-kebab "example_to_snake_case"
EXAMPLE-TO-SHOUTY-KEBAB-CASE

$ heck-string --to=shouty-kebab "Example-To-Train-Case"
EXAMPLE-TO-SHOUTY-KEBAB-CASE
```

### `heck-string --to=shouty-snake`

Convert string to shouty-snake case

```shell
$ heck-string --to=shouty-snake "example-to-kebab-case"
EXAMPLE_TO_SHOUTY_SNAKE_CASE

$ heck-string --to=shouty-snake "exampleToCamelCase"
EXAMPLE_TO_SHOUTY_SNAKE_CASE

$ heck-string --to=shouty-snake "ExampleToPascalCase"
EXAMPLE_TO_SHOUTY_SNAKE_CASE

$ heck-string --to=shouty-snake "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
EXAMPLE_TO_SHOUTY_SNAKE_CASE

$ heck-string --to=shouty-snake "example_to_snake_case"
EXAMPLE_TO_SHOUTY_SNAKE_CASE

$ heck-string --to=shouty-snake "Example-To-Train-Case"
EXAMPLE_TO_SHOUTY_SNAKE_CASE
```

### `heck-string --to=snake`

Convert string to snake case

```shell
$ heck-string --to=snake "example-to-kebab-case"
example_to_snake_case

$ heck-string --to=snake "exampleToCamelCase"
example_to_snake_case

$ heck-string --to=snake "ExampleToPascalCase"
example_to_snake_case

$ heck-string --to=snake "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
example_to_snake_case

$ heck-string --to=snake "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
example_to_snake_case

$ heck-string --to=snake "Example-To-Train-Case"
example_to_snake_case
```

### `heck-string --to=train`

Convert string to train case

```shell
$ heck-string --to=train "example-to-kebab-case"
Example-To-Train-Case

$ heck-string --to=train "exampleToCamelCase"
Example-To-Train-Case

$ heck-string --to=train "ExampleToPascalCase"
Example-To-Train-Case

$ heck-string --to=train "EXAMPLE-TO-SHOUTY-KEBAB-CASE"
Example-To-Train-Case

$ heck-string --to=train "EXAMPLE_TO_SHOUTY_SNAKE_CASE"
Example-To-Train-Case

$ heck-string --to=train "example_to_snake_case"
Example-To-Train-Case
```

