![travis-ci](https://travis-ci.org/inosion/markdown-tools.svg?branch=master)

# Markdown Tooling

* Supports making Markdown Tables from tabular data

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

## Commandline

Excel, (XLSX, XLS, XLSM, XLSB, ODS)

`md_tools table -t xlsx test/sample_multi_sheet.xslx.xlsx`
```
**Sheet1**
|Change|  Language  |                         Rank                         | Share |Trend |
|------|------------|------------------------------------------------------|-------|------|
|      |   Python   |                          1                           |23.59 %|+5.5 %|
|      |    Java    |                          2                           |22.4 % |-0.5 %|
|      | Javascript |                          3                           |8.49 % |+0.2 %|
|      |    PHP     |                          4                           |7.93 % |-1.5 %|
...

**second_sheet**
|        Heading 4         |        Random Stuff        |       Sample XLS Data Type        |                The Resulting Value                 |
|--------------------------|----------------------------|-----------------------------------|----------------------------------------------------|
|                          |                            |           >> =”Formula”           |                      Formula                       |
|         << empty         |                            |         >> Simple String          |                       Value                        |
|                          |                            |          >> Some Number           |                 0.0977795336595647                 |
|*Some Bolding in Markdown*|   >> Markdown Formatting   |        >> Large Cell Value        |Something longer than expected for most of the table|
|   `escaped value` foo    |                            |>> Conflicting Table Markdown Text |             This cell has \| pipes \|              |
|           0.22           |>> Percentage (cell is 0.22)|           >> Multiline            |            This Cell<br/>Is multi-line             |
|                          |                            |       >> Invalid Reference        |                       #REF!                        |
|                          |                            |     >> Valid Reference (=B1)      |                The Resulting Value                 |
|         #DIV/0!          |        >> Div by 0         |       >> Unicode Characters       |                    😕 ← Emoticon                    |
|                          |                            |           >> Date Style           |                       43122                        |
...

**3rd Sheet**
|NULL5|col1|col2| col3 |col4 |                         col5                          |
|-----|----|----|------|-----|-------------------------------------------------------|
| aaa | 1  |that| are  |wider|  value ‘aaa’ is in the next cell, but has no heading  |
|     |than|the |header| row |       (open the spreadsheet to see what I mean)       |
```

`md_tools table -t yaml www-sample/test.yml`
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

### Known Issues
* A Spreadsheet Cell with a Date will come out as the "magic" Excel date number :-( - https://github.com/tafia/calamine/issues/116
* Order of columns in the YAML are not preserved. LinkedHashMap fixes it, but the serde bindings doesn't seem to work. Not sure why yet.


