* `0.1.2`- 24 Jul 2018
  - Fixed issue #1 where the `\r\n` return lines (pertinent inside MS XLS files in a Cell) would muck up the MD
     as they were left with just a `\r`
* `0.2.0`- 25 Jul 2018
  - Added the `--sheetname <name>` to the command line tool.
* `0.3.0` - 26 Jul 2018
  - From the command line, you can export the XLSn/ODS sheet(s) as YAML.
    This is helpful if you want to "store the YAML sheet into a repo"
* `0.4.0` - 26 Jul 2018
  - To help with CI/CD type environments, and scripting, you can list out the sheetnames
    like `md_tools listsheets <filename>`
* `0.5.0` - 12 Aug 2018
  - Completed the filter options that support regex
  - Renamed to `madato` - rhymes with tomato
* `0.5.1` - 12 Aug 2018
  - tidy up for crates.io release
* `0.6.0` - 4 May 2024
  - added CSV support
* `0.7.0` - 5 May 2024
  - Added python library. Can now use it in python
