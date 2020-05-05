TOML Parser for Node.js
=======================

[![Build Status](https://travis-ci.org/sigoden/wasm-toml-js.png?branch=master)](https://travis-ci.org/sigoden/wasm-toml-js)

[![NPM](https://nodei.co/npm/wasm-toml-js.png?downloads=true)](https://nodei.co/npm/wasm-toml-js/)

If you haven't heard of TOML, well you're just missing out. [Go check it out now.](https://github.com/toml-lang/toml) Back? Good.

TOML Spec Support
-----------------

wasm-toml-js supports version 0.5.0 the TOML spec as specified by [mojombo/toml@v0.5.0](https://github.com/toml-lang/toml/blob/v0.5.0/versions/en/toml-v0.5.0.md)

Installation
------------

wasm-toml-js is available via npm.

    npm install wasm-toml-js

Usage
-----

```js
const toml = require("wasm-toml-js");
const tomlContent = `# This is a TOML document.
title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates
`
const output = toml.parse(tomlContent); 
console.log(JSON.stringify(output, null, 2)); 
```

License
-------

wasm-toml-js is licensed under the MIT license agreement. See the LICENSE file for more information.