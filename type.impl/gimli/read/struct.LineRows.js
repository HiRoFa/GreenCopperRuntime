(function() {
    var type_impls = Object.fromEntries([["gimli",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LineRows%3CR,+Program,+Offset%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#167\">source</a><a href=\"#impl-Clone-for-LineRows%3CR,+Program,+Offset%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R, Program, Offset&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"gimli/read/struct.LineRows.html\" title=\"struct gimli::read::LineRows\">LineRows</a>&lt;R, Program, Offset&gt;<div class=\"where\">where\n    Program: <a class=\"trait\" href=\"gimli/read/trait.LineProgram.html\" title=\"trait gimli::read::LineProgram\">LineProgram</a>&lt;R, Offset&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    R: <a class=\"trait\" href=\"gimli/read/trait.Reader.html\" title=\"trait gimli::read::Reader\">Reader</a>&lt;Offset = Offset&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    Offset: <a class=\"trait\" href=\"gimli/read/trait.ReaderOffset.html\" title=\"trait gimli::read::ReaderOffset\">ReaderOffset</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#167\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"gimli/read/struct.LineRows.html\" title=\"struct gimli::read::LineRows\">LineRows</a>&lt;R, Program, Offset&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","gimli::read::line::StateMachine"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-LineRows%3CR,+Program,+Offset%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#167\">source</a><a href=\"#impl-Debug-for-LineRows%3CR,+Program,+Offset%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R, Program, Offset&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"gimli/read/struct.LineRows.html\" title=\"struct gimli::read::LineRows\">LineRows</a>&lt;R, Program, Offset&gt;<div class=\"where\">where\n    Program: <a class=\"trait\" href=\"gimli/read/trait.LineProgram.html\" title=\"trait gimli::read::LineProgram\">LineProgram</a>&lt;R, Offset&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    R: <a class=\"trait\" href=\"gimli/read/trait.Reader.html\" title=\"trait gimli::read::Reader\">Reader</a>&lt;Offset = Offset&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    Offset: <a class=\"trait\" href=\"gimli/read/trait.ReaderOffset.html\" title=\"trait gimli::read::ReaderOffset\">ReaderOffset</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#167\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","gimli::read::line::StateMachine"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LineRows%3CR,+Program,+Offset%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#185-259\">source</a><a href=\"#impl-LineRows%3CR,+Program,+Offset%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R, Program, Offset&gt; <a class=\"struct\" href=\"gimli/read/struct.LineRows.html\" title=\"struct gimli::read::LineRows\">LineRows</a>&lt;R, Program, Offset&gt;<div class=\"where\">where\n    Program: <a class=\"trait\" href=\"gimli/read/trait.LineProgram.html\" title=\"trait gimli::read::LineProgram\">LineProgram</a>&lt;R, Offset&gt;,\n    R: <a class=\"trait\" href=\"gimli/read/trait.Reader.html\" title=\"trait gimli::read::Reader\">Reader</a>&lt;Offset = Offset&gt;,\n    Offset: <a class=\"trait\" href=\"gimli/read/trait.ReaderOffset.html\" title=\"trait gimli::read::ReaderOffset\">ReaderOffset</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.header\" class=\"method\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#219-221\">source</a><h4 class=\"code-header\">pub fn <a href=\"gimli/read/struct.LineRows.html#tymethod.header\" class=\"fn\">header</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"gimli/read/struct.LineProgramHeader.html\" title=\"struct gimli::read::LineProgramHeader\">LineProgramHeader</a>&lt;R, Offset&gt;</h4></section></summary><div class=\"docblock\"><p>Get a reference to the header for this state machine’s line number\nprogram.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.next_row\" class=\"method\"><a class=\"src rightside\" href=\"src/gimli/read/line.rs.html#233-258\">source</a><h4 class=\"code-header\">pub fn <a href=\"gimli/read/struct.LineRows.html#tymethod.next_row\" class=\"fn\">next_row</a>(\n    &amp;mut self,\n) -&gt; <a class=\"type\" href=\"gimli/read/type.Result.html\" title=\"type gimli::read::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;(&amp;<a class=\"struct\" href=\"gimli/read/struct.LineProgramHeader.html\" title=\"struct gimli::read::LineProgramHeader\">LineProgramHeader</a>&lt;R, Offset&gt;, &amp;<a class=\"struct\" href=\"gimli/read/struct.LineRow.html\" title=\"struct gimli::read::LineRow\">LineRow</a>)&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Parse and execute the next instructions in the line number program until\nanother row in the line number matrix is computed.</p>\n<p>The freshly computed row is returned as <code>Ok(Some((header, row)))</code>.\nIf the matrix is complete, and there are no more new rows in the line\nnumber matrix, then <code>Ok(None)</code> is returned. If there was an error parsing\nan instruction, then <code>Err(e)</code> is returned.</p>\n<p>Unfortunately, the references mean that this cannot be a\n<code>FallibleIterator</code>.</p>\n</div></details></div></details>",0,"gimli::read::line::StateMachine"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[8740]}