TOML format for Node.js
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
const { parse, stringify } = require("../pkg");
const assert = require("assert");
const tomlContent = `title = 'TOML Example'

[owner]
name = 'Tom Preston-Werner'
`
const jsonValue = parse(tomlContent); 
const tomlString = stringify(jsonValue);

assert.deepEqual(tomlContent, tomlString);
```

License
-------

wasm-toml-js is licensed under the MIT license agreement. See the LICENSE file for more information.