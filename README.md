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
$ heck-string --to=kebab "exampleFromCamelCase"
example-from-camel-case

$ heck-string --to=kebab "ExampleFromPascalCase"
example-from-pascal-case

$ heck-string --to=kebab "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
example-from-shouty-kebab-case

$ heck-string --to=kebab "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
example-from-shouty-snake-case

$ heck-string --to=kebab "example_from_snake_case"
example-from-snake-case

$ heck-string --to=kebab "Example-From-Train-Case"
example-from-train-case
```

### `heck-string --to=camel`

Convert string to camel case

```shell
$ heck-string --to=camel "example-from-kebab-case"
exampleFromKebabCase

$ heck-string --to=camel "ExampleFromPascalCase"
exampleFromPascalCase

$ heck-string --to=camel "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
exampleFromShoutyKebabCase

$ heck-string --to=camel "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
exampleFromShoutySnakeCase

$ heck-string --to=camel "example_from_snake_case"
exampleFromSnakeCase

$ heck-string --to=camel "Example-From-Train-Case"
exampleFromTrainCase
```

### `heck-string --to=pascal`

Convert string to pascal case

```shell
$ heck-string --to=pascal "example-from-kebab-case"
ExampleFromKebabCase

$ heck-string --to=pascal "exampleFromCamelCase"
ExampleFromCamelCase

$ heck-string --to=pascal "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
ExampleFromShoutyKebabCase

$ heck-string --to=pascal "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
ExampleFromShoutySnakeCase

$ heck-string --to=pascal "example_from_snake_case"
ExampleFromSnakeCase

$ heck-string --to=pascal "Example-From-Train-Case"
ExampleFromTrainCase
```

### `heck-string --to=shouty-kebab`

Convert string to shouty-kebab case

```shell
$ heck-string --to=shouty-kebab "example-from-kebab-case"
EXAMPLE-FROM-KEBAB-CASE

$ heck-string --to=shouty-kebab "exampleFromCamelCase"
EXAMPLE-FROM-CAMEL-CASE

$ heck-string --to=shouty-kebab "ExampleFromPascalCase"
EXAMPLE-FROM-PASCAL-CASE

$ heck-string --to=shouty-kebab "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
EXAMPLE-FROM-SHOUTY-SNAKE-CASE

$ heck-string --to=shouty-kebab "example_from_snake_case"
EXAMPLE-FROM-SNAKE-CASE

$ heck-string --to=shouty-kebab "Example-From-Train-Case"
EXAMPLE-FROM-TRAIN-CASE
```

### `heck-string --to=shouty-snake`

Convert string to shouty-snake case

```shell
$ heck-string --to=shouty-snake "example-from-kebab-case"
EXAMPLE_FROM_KEBAB_CASE

$ heck-string --to=shouty-snake "exampleFromCamelCase"
EXAMPLE_FROM_CAMEL_CASE

$ heck-string --to=shouty-snake "ExampleFromPascalCase"
EXAMPLE_FROM_PASCAL_CASE

$ heck-string --to=shouty-snake "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
EXAMPLE_FROM_SHOUTY_KEBAB_CASE

$ heck-string --to=shouty-snake "example_from_snake_case"
EXAMPLE_FROM_SNAKE_CASE

$ heck-string --to=shouty-snake "Example-From-Train-Case"
EXAMPLE_FROM_TRAIN_CASE
```

### `heck-string --to=snake`

Convert string to snake case

```shell
$ heck-string --to=snake "example-from-kebab-case"
example_from_kebab_case

$ heck-string --to=snake "exampleFromCamelCase"
example_from_camel_case

$ heck-string --to=snake "ExampleFromPascalCase"
example_from_pascal_case

$ heck-string --to=snake "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
example_from_shouty_kebab_case

$ heck-string --to=snake "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
example_from_shouty_snake_case

$ heck-string --to=snake "Example-From-Train-Case"
example_from_train_case
```

### `heck-string --to=train`

Convert string to train case

```shell
$ heck-string --to=train "example-from-kebab-case"
Example-From-Kebab-Case

$ heck-string --to=train "exampleFromCamelCase"
Example-From-Camel-Case

$ heck-string --to=train "ExampleFromPascalCase"
Example-From-Pascal-Case

$ heck-string --to=train "EXAMPLE-FROM-SHOUTY-KEBAB-CASE"
Example-From-Shouty-Kebab-Case

$ heck-string --to=train "EXAMPLE_FROM_SHOUTY_SNAKE_CASE"
Example-From-Shouty-Snake-Case

$ heck-string --to=train "example_from_snake_case"
Example-From-Snake-Case
```

