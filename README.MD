
# YAML Sorter

This project is a Rust application that processes and sorts YAML files based on specific configurations.

## Features

- **Sorting YAML Arrays**: Sorts arrays in YAML files based on a specified key.
- **Sorting YAML Hashes**: Sorts hashes in YAML files based on a predefined order and sorts remaining keys alphabetically.
- **Configurable**: Uses a `config.yaml` file for customizable sorting rules.

## Installation

To build and install the project, run the following command:

```sh
cargo build --release
```

The executable will be located in the `target/release` directory.

## Usage

### Command-line Interface

The application is run via the command line with the following syntax:

```sh
cargo run -- <action> <path>
```

- `<action>`: The action to perform. For example, `i` for in-place processing.
- `<path>`: The path to the YAML file to process.

Example:

```sh
cargo run -- i example.yaml
```

### Configuration

The application uses a configuration file named `config.yaml`. The configuration file should be placed in the root directory of the project. This file specifies the order in which keys should be sorted and the key used for sorting arrays.

Example `config.yaml`:

```yaml
preOrder:
  - key1
  - key2
sortKey: key_name
```

### Example

Given the following `example.yaml`:

```yaml
- age: 35
  name: Carol
- age: 30
  name: Alice
- age: 25
  name: Bob
```

Running the application with:

```sh
cargo run -- i example.yaml
```

And a `config.yaml`:

```yaml
preOrder:
  - name
  - age
sortKey: name
```

Will produce the following sorted `example.yaml`:

```yaml
- name: Alice
  age: 30
- name: Bob
  age: 25
- name: Carol
  age: 35
```

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
