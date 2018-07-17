const mtm = import("./markdown_tools");
const yaml = require('js-yaml');
const md = require('markdown-it')();

var myyml = `
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
`;

try {
  const config = yaml.safeLoad(myyml);
  const indentedJson = JSON.stringify(config, null, 4);
  console.log(indentedJson);
} catch (e) {
  console.log(e);
}

mtm.then(mtm => {
  var md = mtm.mk_md_table_from_yaml(myyml);
});

function render_table() {
  if($.trim($('#headings').val()).length > 0) {
    mtm.then(mtm => {
      $('#markdown').val( mtm.mk_md_table_from_yaml_with_headings_list( $('#headings').val(), $('#yamlContent').val() ) );
      $('#markdownResult').html( md.render(  $('#markdown').val() ) );
      $( "table" ).addClass("table");
    });  
  } else {
    mtm.then(mtm => {
      $('#markdown').val( mtm.mk_md_table_from_yaml( $('#yamlContent').val() ) );
      $('#markdownResult').html( md.render(  $('#markdown').val() ) );
      $( "table" ).addClass("table");
    });
  }
} 



$(function ()
    {
        $('#yamlContent').change(function () { render_table() });
        $('#headings').change(function () { render_table() });
    });


// $('#yamlContent').on('change', function(e){
//   alert(this.value);
//   console.log(this.value,mtm.mk_md_table_from_yaml(this.value));
// });

