var doc = `
Usage:
  markdown-table-maker.js [--jsonpath <jsonpath>] [--headings <headings>] [--must-have-all] <yml_file>
  markdown-table-maker.js -h | --help | --version

Options:
  --headings <headings>    Comma separated list of headings to ONLY use. eg: col1,col2,col_3
  --jsonpath <jsonpath>    json-path compatible selector to "locate" the Array of Maps in a document.
`;

const docopt = require('docopt-js');
const table  = require('markdown-table')
const yaml   = require('js-yaml');
const fs     = require('fs');
const jp     = require('jsonpath');
const path   = require('path');

const args = docopt.docopt(doc, { version: '0.1' });
// console.log(args);

/*
 * Collect "all" the keys in the array of maps
 */

function all_keys_from_array_of_maps(data_array) {

  var header_set = {}

  for (e in data_array) {
    for (k in data_array[e]) {
      header_set[k] = true;
    }
  }
  return Object.keys(header_set);
}

var data = null;

try {
    data = yaml.safeLoad(fs.readFileSync(path.join(process.cwd(),args["<yml_file>"]), 'utf8'));
    // If the user supplied a "selector" to get into the data, we will use that
    if (args["--jsonpath"]) {
       xdata = jp.query(data, args["--jsonpath"])
       data = xdata;
    }
} catch (e) {
    console.log(e);
}

// console.log(data);

var headings = all_keys_from_array_of_maps(data);

/*
 * if the user passed a select set of headings, we will filter for only these
 */

if (args["--headings"]) {
  headings_filter = args["--headings"].split(",");
  headings = headings_filter.filter(e => headings.includes(e)); 
}

table_data = [ headings ];

for (i in data) {
  var row_data = [];

  for (ih in headings) {
    // only put data in if we have it
    if (data[i][headings[ih]]) {
      row_data.push(data[i][headings[ih]]);
    }
  }

  if (args["--must-have-all"]) {

    // the size of the "row_data" length MUST match the length of the headings, else
    if (headings.length == row_data.length) {
      table_data.push(row_data);
    }

  } else {

    table_data.push(row_data);

  }

}

var t = table(table_data);

console.log(t);
