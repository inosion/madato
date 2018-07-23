![travis-ci](https://travis-ci.org/inosion/markdown-tools.svg?branch=master)

# Markdown Tooling

* Supports making Markdown Tables from tabular data; YAML files and Excel/OpenOffice Spreadsheets

provided as 

* Command Line Tool (Windows, Mac, Linux)
* Rust API
* Node JS WASM API

## Tables
* Converts a YAML file, or a Spreadsheet to a Markdown Table
* Spreadsheets - via Calamine
    - excel like (xls, xlsx, xlsm, xlsb, xla, xlam)
    - opendocument spreadsheets (ods)
* YAML (Array of Maps)

# TL;DR For Install and Usage

* Cut and Paste some random tables from some HTML file, save it to Excel / OpenOffice. and run this tool, it will give you a nicely formatted Markdown Table of the same :-D

## Commandline

### Excel, (XLSX, XLS, XLSM, XLSB, ODS)

`md_tools table -t xlsx test/sample_multi_sheet.xslx.xlsx` 

See [test/sample_multi_sheet.xslx.xlsx](test/sample_multi_sheet.xslx.xlsx) for the original file.

```
**Sheet1**
|                         Rank                         |Change|  Language  | Share |Trend |
|------------------------------------------------------|------|------------|-------|------|
|                          1                           |      |   Python   |23.59 %|+5.5 %|
|                          2                           |      |    Java    |22.4 % |-0.5 %|
|                          3                           |      | Javascript |8.49 % |+0.2 %|
|                          4                           |      |    PHP     |7.93 % |-1.5 %|
|                          5                           |      |     C#     |7.84 % |-0.5 %|
|                          6                           |      |   C/C++    |6.28 % |-0.8 %|
|                          7                           |      |     R      |4.18 % |+0.0 %|
|                          8                           |      |Objective-C | 3.4 % |-1.0 %|
|                          9                           |      |   Swift    |2.65 % |-0.9 %|
|                          10                          |      |   Matlab   |2.25 % |-0.3 %|
|                          11                          |      |    Ruby    |1.59 % |-0.5 %|
|                          12                          |      | TypeScript |1.58 % |+0.3 %|
|                          13                          |      |    VBA     |1.42 % |-0.1 %|
|                          14                          |      |Visual Basic| 1.2 % |-0.2 %|
|                          15                          |      |   Scala    | 1.2 % |-0.1 %|
|                          16                          |      |   Kotlin   |0.97 % |+0.5 %|
|                          17                          |      |     Go     |0.93 % |+0.3 %|
|                          18                          |      |    Perl    |0.78 % |-0.1 %|
|                          19                          |      |    Lua     |0.42 % |-0.1 %|
|                          20                          |      |    Rust    |0.36 % |+0.0 %|
|                          21                          |      |  Haskell   | 0.3 % |-0.1 %|
|                          22                          |      |   Delphi   |0.25 % |-0.1 %|
|( Source - http://pypl.github.io/PYPL.html - Jul 2018)|      |            |       |      |

**second_sheet**
|       Sample XLS Data Type        |                The Resulting Value                 |        Random Stuff        |        Heading 4         |
|-----------------------------------|----------------------------------------------------|----------------------------|--------------------------|
|           >> =”Formula”           |                      Formula                       |                            |                          |
|         >> Simple String          |                       Value                        |                            |         << empty         |
|          >> Some Number           |                 0.0977795336595647                 |                            |                          |
|        >> Large Cell Value        |Something longer than expected for most of the table|   >> Markdown Formatting   |*Some Bolding in Markdown*|
|>> Conflicting Table Markdown Text |             This cell has \| pipes \|              |                            |   `escaped value` foo    |
|           >> Multiline            |            This Cell<br/>Is multi-line             |>> Percentage (cell is 0.22)|           0.22           |
|       >> Invalid Reference        |                       #REF!                        |                            |                          |
|     >> Valid Reference (=B1)      |                The Resulting Value                 |                            |                          |
|       >> Unicode Characters       |                    😕 ← Emoticon                    |        >> Div by 0         |         #DIV/0!          |
|           >> Date Style           |                       43122                        |                            |                          |
|                                   |                       43122                        |         >> Quotes          |  “This cell has quotes”  |
|                                   |                      -888.78                       |                            |                          |
|                                   |                      #VALUE!                       |                            |       😕 ← Emoticon       |

**3rd Sheet**
|col1|col2| col3 |col4 |                         col5                          |NULL5|
|----|----|------|-----|-------------------------------------------------------|-----|
| 1  |that| are  |wider|  value ‘aaa’ is in the next cell, but has no heading  | aaa |
|than|the |header| row |       (open the spreadsheet to see what I mean)       |     |
```

`md_tools table -t yaml test/www-sample/test.yml`

### YAML Example

```
|col3| col4  |  data1  |       data2        |
|----|-------|---------|--------------------|
|100 |gar gar|somevalue|someother value here|
|190x|       |  that   |        nice        |
|100 | ta da |  this   |someother value here|
```
## Rust

`TODO` - Publish on crates.io

## JavaScript

`TODO` - Publish on npm

# Details

## Table Maker

Converts YAML to a Markdown Table String
```
- data1: somevalue
  data2: someother value here
  col3: 100 
  col4: gar gar
- data1: that
  data2: nice
  col3: 190x 
- data1: this
  data2: someother value here
  col3: 100 
  col4: ta da
```

to 

```
|col3| col4  |  data1  |       data2        |
|----|-------|---------|--------------------|
|100 |gar gar|somevalue|someother value here|
|190x|       |  that   |        nice        |
|100 | ta da |  this   |someother value here|

```

And as Markdown:

|col3| col4  |  data1  |       data2        |
|----|-------|---------|--------------------|
|100 |gar gar|somevalue|someother value here|
|190x|       |  that   |        nice        |
|100 | ta da |  this   |someother value here|


### Features

* `[x]` Reads a formatted YAML string and renders a Markdown Table
* `[x]` Can take an optional list of column headings, and only display those from the table (filtering out other columns present)
* `[X]` Native Binary Command Line (windows, linux, osx)
* `[X]` Read an XLSX file and produce a Markdown Table
* `[X]` Read an ODS file and produce a Markdown Table
* `[ ]` Read a CSV, TSV, PSV (etc) file and produce a Markdown Table
* `[ ]` Support Nested Structures in the YAML imput

### Future Goals
* Finish the testing and publishing of the JS WASM Bindings. (PS - it works.. 
  (see : [test/www-sample](test/www-sample) and the [Makefile](Makefile) )
* Embed the "importing" of YAML, CSV and XLS* files into the `mume` Markdown Preview Enhanced Plugin. [https://shd101wyy.github.io/markdown-preview-enhanced/](https://shd101wyy.github.io/markdown-preview-enhanced/) So we can have Awesome Markdown Documents.

### Known Issues
* A Spreadsheet Cell with a Date will come out as the "magic" Excel date number :-( - https://github.com/tafia/calamine/issues/116
* Order of columns in the YAML are not preserved. LinkedHashMap fixes it, but the serde bindings doesn't seem to work. Not sure why yet.
