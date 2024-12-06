(function() {
    var type_impls = Object.fromEntries([["pem_rfc7468",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Decoder%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#25\">source</a><a href=\"#impl-Clone-for-Decoder%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"base64ct/decoder/struct.Decoder.html\" title=\"struct base64ct::decoder::Decoder\">Decoder</a>&lt;'i, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"base64ct/encoding/trait.Encoding.html\" title=\"trait base64ct::encoding::Encoding\">Encoding</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#25\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"base64ct/decoder/struct.Decoder.html\" title=\"struct base64ct::decoder::Decoder\">Decoder</a>&lt;'i, E&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","pem_rfc7468::Base64Decoder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decoder%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#43\">source</a><a href=\"#impl-Decoder%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"struct\" href=\"base64ct/decoder/struct.Decoder.html\" title=\"struct base64ct::decoder::Decoder\">Decoder</a>&lt;'i, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"base64ct/encoding/trait.Encoding.html\" title=\"trait base64ct::encoding::Encoding\">Encoding</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#50\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.new\" class=\"fn\">new</a>(input: &amp;'i [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"base64ct/decoder/struct.Decoder.html\" title=\"struct base64ct::decoder::Decoder\">Decoder</a>&lt;'i, E&gt;, <a class=\"enum\" href=\"base64ct/errors/enum.Error.html\" title=\"enum base64ct::errors::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new decoder for a byte slice containing contiguous\n(non-newline-delimited) Base64-encoded data.</p>\n<h5 id=\"returns\"><a class=\"doc-anchor\" href=\"#returns\">§</a>Returns</h5>\n<ul>\n<li><code>Ok(decoder)</code> on success.</li>\n<li><code>Err(Error::InvalidLength)</code> if the input buffer is empty.</li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_wrapped\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#87\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.new_wrapped\" class=\"fn\">new_wrapped</a>(\n    input: &amp;'i [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>],\n    line_width: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.usize.html\">usize</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"base64ct/decoder/struct.Decoder.html\" title=\"struct base64ct::decoder::Decoder\">Decoder</a>&lt;'i, E&gt;, <a class=\"enum\" href=\"base64ct/errors/enum.Error.html\" title=\"enum base64ct::errors::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new decoder for a byte slice containing Base64 which\nline wraps at the given line length.</p>\n<p>Trailing newlines are not supported and must be removed in advance.</p>\n<p>Newlines are handled according to what are roughly <a href=\"https://datatracker.ietf.org/doc/html/rfc7468\">RFC7468</a> conventions:</p>\n<div class=\"example-wrap\"><pre class=\"language-text\"><code>[parsers] MUST handle different newline conventions</code></pre></div>\n<p>RFC7468 allows any of the following as newlines, and allows a mixture\nof different types of newlines:</p>\n<div class=\"example-wrap\"><pre class=\"language-text\"><code>eol        = CRLF / CR / LF</code></pre></div><h5 id=\"returns-1\"><a class=\"doc-anchor\" href=\"#returns-1\">§</a>Returns</h5>\n<ul>\n<li><code>Ok(decoder)</code> on success.</li>\n<li><code>Err(Error::InvalidLength)</code> if the input buffer is empty or the line\nwidth is zero.</li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#107\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.decode\" class=\"fn\">decode</a>&lt;'o&gt;(&amp;mut self, out: &amp;'o mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&amp;'o [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>], <a class=\"enum\" href=\"base64ct/errors/enum.Error.html\" title=\"enum base64ct::errors::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Fill the provided buffer with data decoded from Base64.</p>\n<p>Enough Base64 input data must remain to fill the entire buffer.</p>\n<h5 id=\"returns-2\"><a class=\"doc-anchor\" href=\"#returns-2\">§</a>Returns</h5>\n<ul>\n<li><code>Ok(bytes)</code> if the expected amount of data was read</li>\n<li><code>Err(Error::InvalidLength)</code> if the exact amount of data couldn’t be read</li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode_to_end\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#169\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.decode_to_end\" class=\"fn\">decode_to_end</a>&lt;'o&gt;(\n    &amp;mut self,\n    buf: &amp;'o mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&amp;'o [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.u8.html\">u8</a>], <a class=\"enum\" href=\"base64ct/errors/enum.Error.html\" title=\"enum base64ct::errors::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Decode all remaining Base64 data, placing the result into <code>buf</code>.</p>\n<p>If successful, this function will return the total number of bytes\ndecoded into <code>buf</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.remaining_len\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#187\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.remaining_len\" class=\"fn\">remaining_len</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Get the length of the remaining data after Base64 decoding.</p>\n<p>Decreases every time data is decoded.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_finished\" class=\"method\"><a class=\"src rightside\" href=\"src/base64ct/decoder.rs.html#192\">source</a><h4 class=\"code-header\">pub fn <a href=\"base64ct/decoder/struct.Decoder.html#tymethod.is_finished\" class=\"fn\">is_finished</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Has all of the input data been decoded?</p>\n</div></details></div></details>",0,"pem_rfc7468::Base64Decoder"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[10022]}