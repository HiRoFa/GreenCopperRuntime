searchState.loadedDescShard("jsonc_parser", 0, "Options for collecting comments and tokens.\nMap where the comments are stored in collections where the …\nA JSON array.\nA JSON object.\nA JSON value.\nOptions for parsing.\nResult of parsing the text.\nConverts text into a stream of tokens.\nAllow comments (defaults to <code>true</code>).\nAllow words and numbers as object property names (defaults …\nAllow trailing commas on object literal and array literal …\nCollection of comments in the text.\nInclude comments in the result.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a value in the object by its name.\nGets a value from the array by index.\nGets an array property value from the object by name. …\nGets a boolean property value from the object by name. …\nGets a number property value from the object by name. …\nGets an object property value from the object by name. …\nGets a string property value from the object by name. …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGets if there are no properties.\nGets if the array is empty.\nIterates over the array elements.\nGets the number of properties.\nGets the number of elements.\nCreates a new scanner based on the provided text.\nCreates a new JsonObject.\nCreates a new JsonArray.\nParses a string containing JSONC to an AST with comments …\nParses a string containing JSONC to a `serde_json::Value.\nParses a string containing JSONC to a <code>JsonValue</code>.\nMoves to and returns the next token.\nTakes a value from the object by name. Returns <code>None</code> when …\nTakes an array property value from the object by name. …\nTakes a boolean property value from the object by name. …\nDrops the object returning the inner hash map.\nDrops the object returning the inner vector.\nTakes a number property value from the object by name. …\nTakes an object property value from the object by name. …\nTakes a string property value from the object by name. …\nGets the current token.\nGets the end position of the token.\nGets the start position of the token.\nCollection of tokens (excluding any comments).\nInclude tokens in the result.\nThe JSON value the text contained.\nRepresents an array that may contain elements (ex. <code>[]</code>, …\nRepresents a boolean (ex. <code>true</code> or <code>false</code>).\nJSONC comment.\nRepresents a comment block (ex. <code>/* my comment */</code>).\nKind of JSONC comment.\nRepresents a comment line (ex. <code>// my comment</code>).\nNode that can appear in the AST.\nKind of AST node.\nRepresents the null keyword (ex. <code>null</code>).\nRepresents a number (ex. <code>123</code>, <code>99.99</code>, <code>-1.2e+2</code>).\nRepresents an object that may contain properties (ex. <code>{}</code>, …\nRepresents an object property (ex. <code>&quot;prop&quot;: []</code>).\nRepresents an object property name that may or may not be …\nNode surrounded in double quotes (ex. <code>&quot;my string&quot;</code>).\nJSON value.\nA string that’s not in quotes. Usually the appearance of …\nGets the object property name as a string reference.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a property value in the object by its name.\nGets an array property value from the object by name. …\nGets a boolean property value from the object by name. …\nGets a number property value from the object by name. …\nGets an object property value from the object by name. …\nGets a string property value from the object by name. …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the object property name into a string.\nGets the node kind.\nGets the comment kind.\nTakes a value from the object by name. Returns <code>None</code> when …\nTakes an array property value from the object by name. …\nTakes a boolean property value from the object by name. …\nTakes a number property value from the object by name. …\nTakes an object property value from the object by name. …\nTakes a string property value from the object by name. …\nGets the text of the comment.\nPositional information about a start and end point in the …\nRepresents an object that has a range in the text.\nGets the byte index after the last character in the text.\nEnd position of the node in the text.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nGets the range.\nGets the byte index of the first character in the text.\nStart position of the node in the text.\nGets the text from the provided string.\nGets the end byte index minus the start byte index of the …\nError that could occur while parsing or tokenizing.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nError message.\nStart and end position of the error.\nA token found while scanning.\nA token with positional information.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")