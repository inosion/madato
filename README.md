# Markdown Tooling

The library is a set of helper functions written in Rust and exported to JS via WASM bindings.

For Rust users and Node/JS users.

# TL;DR For Install and Usage

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

[x] Reads a formatted YAML string and renders a Markdown Table
[x] Can take an optional list of column headings, and only display those from the table (filtering out other columns present)
[ ] Native Binary Command Line (windows, linux, osx)
[ ] Read an XLSX file and ptroduce a Markdown Table
[ ] Read a CSV, TSV, PSV (etc) file and produce a Markdown Table
[ ] Support Nested Structures in the YAML imput


