# Advanced constrained secret Santa generator

## Introduction



## Installation

To compile this project, first install the Rust toolchain from [the official website](https://rust-lang.org/tools/install/).

Follow the instructions there to set up Rust and Cargo on your system.

Then, clone this repository and navigate to its directory:

```bash
git clone https://github.com/thomasarmel/advanced_constrained_secret_santa.git
cd advanced_constrained_secret_santa
```

Finally, build the project using Cargo:

```bash
cargo build --release
```

The executable will be located in the `target/release` subdirectory.

## Usage

### Configuration

First prepare a config JSON5 file describing the participants and constraints.
We provide an example config file in [example_config.json5](example_config.json5).

Here is the schema of the JSON5 config file:

- **n_gifts** *(integer)*
    The number of gifts each participant should give and receiver.
- **participants** *(array of objects)*
    An array of participant objects, each containing:
    - **name** *(string)*
        The name of the participant (**must be unique**).
    - **family** *(integer)*
        Family identifier to group participants from the same family. A participant cannot give a gift to another participant from the same family.
    - **last_year_targets** *(array of strings)*
        A list of names of participants who received gifts from this participant in the previous year. A participant cannot give a gift to anyone in this list.

### Running the program

To run the program, use the following command:

```bash
cargo run --release -- <path_to_json5_config_file>
```

Optionally you can generate a HTML page that will allow each participant to see their assigned targets. To do so, add the `--to-html` argument:

```bash
cargo run --release -- <path_to_json5_config_file> --to-html
```

This will generate a `santa_results.html` file in the current directory.

The web page should be responsive.

You can host it on your server, and share the link with the participants.

Each participant can then enter their name to see their assigned targets:

![Participant can enter their name](assets/enter_name.png)

After entering their name, the participant will see their assigned targets:

![Participant can see their targets](assets/see_targets.png)

<details>
<summary>For the administrator</summary>

You can type "`perenoel`" as the name to see the full assignment of all participants.
</details>