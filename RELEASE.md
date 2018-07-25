* `0.1.2`- 24 Jul 2018
  - Fixed issue #1 where the `\r\n` return lines (pertinent inside MS XLS files in a Cell) would muck up the MD
     as they were left with just a `\r`
* `0.2.0`- 25 Jul 2018
  - Added the `--sheetname <name>` to the command line tool.
* `0.3.0` - 26 Jul 2018
  - From the command line, you can export the XLS* sheet(s) as YAML. 
    This is helpful if you want to "store the YAML sheet into a repo"
