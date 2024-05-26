# madato

![Rust CI workflow](https://github.com/inosion/madato/actions/workflows/rust-build-release.yml/badge.svg) ![Python CI workflow](https://github.com/inosion/madato/actions/workflows/python-build-release.yml/badge.svg) ![Rust Version](https://img.shields.io/crates/v/madato.svg) ![Rust Version](https://img.shields.io/pypi/v/madato.svg)


***madato is a library and command line tool for working tabular data, and Markdown***

--------------------------------------------------------------------------------

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [madato](#madato)
  - [Usage](#usage)
    - [CLI](#cli)
    - [Rust](#rust)
    - [Python](#python)
  - [Details](#details)
    - [Example CLI usage](#example-cli-usage)
  - [Internals](#internals)
  - [Tips](#tips)
  - [Python](#python-1)
  - [More Commandline](#more-commandline)
    - [Sheet List](#sheet-list)
    - [YAML to Markdown](#yaml-to-markdown)
    - [Excel/ODS to YAML](#excelods-to-yaml)
  - [Features](#features)
  - [Future Goals](#future-goals)
    - [Known Issues](#known-issues)
  - [License](#license)
    - [Contribution](#contribution)

<!-- /code_chunk_output -->



1. `madato (library)` - this library, which reads YAML, CSV, JSON, XLSX/ODS and writes Markdown
3. `madato (cli)` - providing a helpful command line tool of the above
4. The full library is available as a python module, or a rust library.

The tools is primarly centered around getting tabular data (spreadsheets, CSVs)
into Markdown. 

## Usage
### CLI

Download from [https://github.com/inosion/madato/releases](https://github.com/inosion/madato/releases)

### Rust

The library, if you need spreadsheet support, then add the `spreadsheets` feature.

```
madato = { version = "0", features = ["spreadsheets"] }

```

### Python 
```
pip install madato
```

## Details

When generating the output:
- Filter the Rows using basic Regex over Key/Value pairs
- Limit the columns to named headings
- Re-order the columns, or repeat them using the same column feature
- Only generate a table for a named "sheet" (applicable for the XLS/ODS formats)

Madato is: 
- Command Line Tool (Windows, Mac, Linux) - good for CI/CD preprocessing
- Rust Library - Good for integration into Rust Markdown tooling
- Node JS WASM API - To be used later for Atom and VSCode Extensions

Madato expects that every column has a heading row. That is, the first row are headings/column names. If a cell in that first row is blank, it will create `NULL0..NULLn` entries as required.

### Example CLI usage

* Extract the `3rd Sheet` sheet from an MS Excel Document
```
08:39 $ madato table --type xlsx test/sample_multi_sheet.xlsx --sheetname "3rd Sheet"
|col1|col2| col3 |col4 |                         col5                          |NULL5|
|----|----|------|-----|-------------------------------------------------------|-----|
| 1  |that| are  |wider|  value ‘aaa’ is in the next cell, but has no heading  | aaa |
|than|the |header| row |       (open the spreadsheet to see what I mean)       |     |
```

* Extract and reorder just 3 Columns
```
08:42 $ madato table --type xlsx test/sample_multi_sheet.xlsx --sheetname "3rd Sheet" -c col2 -c col3 -c NULL5
|col2| col3 |NULL5|
|----|------|-----|
|that| are  | aaa |
|the |header|     |
```
* Pull from the `second_sheet` sheet
* Only extract `Heading 4` column
* Use a Filter, where `Heading 4` values must only have a letter or number.

```
08:48 $ madato table --type xlsx test/sample_multi_sheet.xlsx --sheetname second_sheet -c "Heading 4" -f 'Heading 4=[a-zA-Z0-9]'
|        Heading 4         |
|--------------------------|
|         << empty         |
|*Some Bolding in Markdown*|
|   `escaped value` foo    |
|           0.22           |
|         #DIV/0!          |
|  “This cell has quotes”  |
|       😕 ← Emoticon       |
```

* Filtering on a Column, ensuring that a "+" is there in `Trend` Column

```
09:00 $ madato table --type xlsx test/sample_multi_sheet.xlsx --sheetname Sheet1 -c Rank -c Language -c Trend -f "Trend=\+"
|                         Rank                         |  Language  |Trend |
|------------------------------------------------------|------------|------|
|                          1                           |   Python   |+5.5 %|
|                          3                           | Javascript |+0.2 %|
|                          7                           |     R      |+0.0 %|
|                          12                          | TypeScript |+0.3 %|
|                          16                          |   Kotlin   |+0.5 %|
|                          17                          |     Go     |+0.3 %|
|                          20                          |    Rust    |+0.0 %|
```

## Internals
madato uses:
- [calamine](https://github.com/tafia/calamine) for reading XLS and ODS sheets
- [wasm bindings](https://github.com/rustwasm/wasm-bindgen) to created JS API versions of the Rust API
- [regex]() for filtering, and [serde]() for serialisation.
- PyO3 and Maturin for Python Support

## Tips

* I have found that copying the "table" I want from a website: HTML, to a spreadsheet, then through `madato` gives an excellent Markdown table of the original.


## Python 

```python
pip install madato

# py
from IPython.display import display, Markdown
import madato
display(Markdown(madato.spreadsheet_to_md("../test/Financial Sample.xlsx")
print(madato.spreadsheet_to_md(str(my_sample_spreadsheet)))

```
* For more examples see [pysource/tests](pysource/tests)

## More Commandline

### Sheet List

You can list the "sheets" of an XLS*, ODS file with 

```
$ madato sheetlist test/sample_multi_sheet.xlsx 
Sheet1
second_sheet
3rd Sheet
```

### YAML to Markdown 

Madato reads a "YAML" file, in the same way it can a Spreadsheet.
This is useful for "keeping" tabular data in your source repository, and perhaps not
the XLS.

`madato table -t yaml test/www-sample/test.yml`

```
|col3| col4  |  data1  |       data2        |
|----|-------|---------|--------------------|
|100 |gar gar|somevalue|someother value here|
|190x|       |  that   |        nice        |
|100 | ta da |  this   |someother value here|
```

*Please see the [test/www-sample/test.yml](test/www-sample/test.yml) file for the expected layout of this file*

### Excel/ODS to YAML

Changing the output from default "Markdown (MD)" to "YAML", you get a Markdown file of the Spreadsheet.

```
madato table -t xlsx test/sample_multi_sheet.xslx.xlsx -s Sheet1 -o yaml
---
- Rank: "1"
  Change: ""
  Language: Python
  Share: "23.59 %"
  Trend: "+5.5 %"
- Rank: "2"
  Change: ""
  Language: Java
  Share: "22.4 %"
  Trend: "-0.5 %"
- Rank: "3"
  Change: ""
  Language: Javascript
  Share: "8.49 %"
...
```

If you omit the sheet name, it will dump all sheets into an order map of array of maps.


## Features

* `[x]` Reads a formatted YAML string and renders a Markdown Table
* `[x]` Can take an optional list of column headings, and only display those from the table (filtering out other columns present)
* `[X]` Native Binary Command Line (windows, linux, osx)
* `[X]` Read an XLSX file and produce a Markdown Table
* `[X]` Read an ODS file and produce a Markdown Table
* `[X]` Read a CSV
* `[X]` Published as a Python Module
* `[ ]` TSV, PSV (etc) file and produce a Markdown Table
* `[ ]` Support Nested Structures in the YAML input
* `[ ]` Read a Markdown File, and select the "table" and turn it back into YAML

## Future Goals
* Finish the testing and publishing of the JS WASM Bindings. (PS - it works.. 
  (see : [test/www-sample](test/www-sample) and the [Makefile](Makefile) )
* Embed the "importing" of YAML, CSV and XLS* files into the `mume` Markdown Preview Enhanced Plugin. [https://shd101wyy.github.io/markdown-preview-enhanced/](https://shd101wyy.github.io/markdown-preview-enhanced/) So we can have Awesome Markdown Documents.
* Provide a `PreRenderer` for `[rust-lang-nursery/mdBook](https://github.com/rust-lang-nursery/mdBook) to "import" MD tables from files.

### Known Issues

* A Spreadsheet Cell with a Date will come out as the "magic" Excel date number :-( - https://github.com/tafia/calamine/issues/116

## License

Serde is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
