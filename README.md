![travis-ci](https://travis-ci.org/inosion/markdown-tools.svg?branch=master)


![under_construction.png](images/under_construction.png)

**The RUST and JS APIs of this libraray are not yet final.**

If you wish to use this lib (other than the command line); create an issue for a stable API and it will get priority. Currently, focus is on features, and the APIs will "arise" from that.

# Markdown Tooling

* Supports making Markdown Tables from tabular data; YAML files and Excel/OpenOffice Spreadsheets
* Supports converting an Excel/ODS Spreadsheet to a YAML file.

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

### Excel, (XLSX, XLS, XLSM, XLSB, ODS)  to Markdown

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

...

**second_sheet**
|       Sample XLS Data Type        |                The Resulting Value                 |        Random Stuff        |        Heading 4         |
|-----------------------------------|----------------------------------------------------|----------------------------|--------------------------|
|           >> =”Formula”           |                      Formula                       |                            |                          |
|         >> Simple String          |                       Value                        |                            |         << empty         |
|          >> Some Number           |                 0.0977795336595647                 |                            |                          |
|       >> Unicode Characters       |                    😕 ← Emoticon                    |        >> Div by 0         |         #DIV/0!          |

...

**3rd Sheet**
|col1|col2| col3 |col4 |                         col5                          |NULL5|
|----|----|------|-----|-------------------------------------------------------|-----|
| 1  |that| are  |wider|  value ‘aaa’ is in the next cell, but has no heading  | aaa |
|than|the |header| row |       (open the spreadsheet to see what I mean)       |     |

```
### YAML to Markdown 

`md_tools table -t yaml test/www-sample/test.yml`

```
|col3| col4  |  data1  |       data2        |
|----|-------|---------|--------------------|
|100 |gar gar|somevalue|someother value here|
|190x|       |  that   |        nice        |
|100 | ta da |  this   |someother value here|
```

### Excel/ODS to YAML

```
md_tools table -t xlsx test/sample_multi_sheet.xslx.xlsx -s Sheet1 -o yaml
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
