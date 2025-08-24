# 📊 Tabler

```text

 _____         _      _
|_   _|  __ _ | |__  | |  ___  _ __
  | |   / _` || '_ \ | | / _ \| '__|
  | |  | (_| || |_) || ||  __/| |
  |_|   \__,_||_.__/ |_| \___||_|

```

📊 Tabler: A lightweight TUI tool to view, query, and navigate CSV, TSV, and Parquet data files.

![Tabler Demo](images/demo.gif)

## ✨ Features

## Features

- ⌨️ **Vim-style Keybindings** - Navigate efficiently with familiar, keyboard-driven controls.
- 🛠️ **SQL Query Support** - Perform advanced queries directly within the interface.
- 🗂️ **Multi-table Management** - Seamlessly load and switch between multiple data tables.
- 📊 **Broad File Format Support** - Compatible with CSV, Parquet, JSON, JSONL, Arrow, and Fixed-Width Formats (FWF).

## 🚀 Installation

To install **tabler**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/tabler.git
cd tabler

cargo build --release
cp ./target/release/tabler /usr/local/bin/
```

Running the below command will globally install the `tabler` binary.

```bash
cargo install tabler
```

Optionally, you can add `~/.cargo/bin` to your PATH if it's not already there

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Usage

Start Tabler with `tabler`

```bash
tabler <path_to_csv(s)>
```

Options:

- `--no-header`: Use this option if the CSV file does not contain a header row.
- `--ignore-errors`: Ignore parsing errors while loading the CSV file.
- `--infer-schema`: Set the schema inference method. Options are no, fast, full, and safe.
- `--quote-char`: Set the quote character.
- `--separator`: Set the separator character.
- `--theme`: Set the theme.

To open TSV file(s), use:

```bash
tabler <path_to_tsv(s)> --separator $'\t' --no-header
```

To open parquet file(s), use:

```bash
tabler <path_to_parquet(s)> -f parquet
```

## Tutorial

For a guide on using Tabler, including instructions on opening files, navigating tables, performing queries, and using inline queries, kindly visit the [tutorial page](https://github.com/trinhminhtriet/tabler/blob/master/tutorial/tutorial.md).

## Keybindings️

| Key Combination         | Functionality                                       |
| ----------------------- | --------------------------------------------------- |
| `v`                     | Switch view                                         |
| `k` or `Arrow Up`       | Move up in the table or scroll up in sheet view     |
| `j` or `Arrow Down`     | Move down in the table or scroll down in sheet view |
| `h` or `Arrow Left`     | Move to the previous item in sheet view             |
| `l` or `Arrow Right`    | Move to the next item in sheet view                 |
| `Page Up` or `Ctrl+b`   | Move one page up                                    |
| `Page Down` or `Ctrl+f` | Move one page down                                  |
| `H`                     | Select previous tab                                 |
| `L`                     | Select next tab                                     |
| `Ctrl+u`                | Move up half a page                                 |
| `Ctrl+d`                | Move down half a page                               |
| `Home` or `g`           | Move to the first row                               |
| `End` or `G`            | Move to the last row                                |
| `R`                     | Select a random row                                 |
| `q`                     | Close current tab                                   |
| `:`                     | Command mode                                        |

## Commands

| Command           | Example                                         | Description                                                                                         |
| ----------------- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `:Q` or `:query`  | `:Q SELECT * FROM df`                           | Query the data in Structured Query Language(SQL). The table name is the file name without extension |
| `:S` or `:select` | `:S price, area, bedrooms, parking`             | Query current data frame for columns/functions                                                      |
| `:F` or `:filter` | `:F price < 20000 AND bedrooms > 4`             | Filter current data frame, keeping rows were the condition(s) match                                 |
| `:O` or `:order`  | `:O area`                                       | Sort current data frame by column(s)                                                                |
| `:tabn`           | `:tabn SELECT * FORM user WHERE balance > 1000` | Create a new tab with the given query                                                               |
| `:q` or `:quit`   | `:q`                                            | Return to table from sheet view otherwise quit                                                      |
| `:schema`         | `:schema`                                       | Show loaded data frame(s) alongside their path(s)                                                   |
| `:reset`          | `:reset`                                        | Reset the table to the original data frame                                                          |
| `:help`           | `:help`                                         | Show help menu                                                                                      |

## Themes

### Monokai (default):

![Image Alt text](/images/theme-monokai.png "Monokai")

### Argonaut:

![Image Alt text](/images/theme-argonaut.png "Argonaut")

### Terminal:

![Image Alt text](/images/theme-terminal.png "Terminal")

## 🗑️ Uninstallation

Running the below command will globally uninstall the `tabler` binary.

```bash
cargo uninstall tabler
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/tabler
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 🙏 Acknowledgements

- [shshemi/tabiew](https://github.com/shshemi/tabiew)

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
