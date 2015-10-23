var ffi = require('ffi');

var lib = ffi.Library('./bin/librust_example', {
  'hello_rust': [ 'string', [] ]
});

console.log( lib.hello_rust() );
