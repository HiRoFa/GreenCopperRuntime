(function() {
    var type_impls = Object.fromEntries([["futures_intrusive",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#834-849\">source</a><a href=\"#impl-Clone-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType, T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html\" title=\"struct futures_intrusive::channel::shared::GenericReceiver\">GenericReceiver</a>&lt;MutexType, T, A&gt;<div class=\"where\">where\n    MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>,\n    A: <a class=\"trait\" href=\"futures_intrusive/buffer/trait.RingBuf.html\" title=\"trait futures_intrusive::buffer::RingBuf\">RingBuf</a>&lt;Item = T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#839-848\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Self</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.83.0/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::Receiver","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::UnbufferedReceiver"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#791-799\">source</a><a href=\"#impl-Debug-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType, T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html\" title=\"struct futures_intrusive::channel::shared::GenericReceiver\">GenericReceiver</a>&lt;MutexType, T, A&gt;<div class=\"where\">where\n    MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>,\n    A: <a class=\"trait\" href=\"futures_intrusive/buffer/trait.RingBuf.html\" title=\"trait futures_intrusive::buffer::RingBuf\">RingBuf</a>&lt;Item = T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#796-798\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::Receiver","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::UnbufferedReceiver"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#851-869\">source</a><a href=\"#impl-Drop-for-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType, T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html\" title=\"struct futures_intrusive::channel::shared::GenericReceiver\">GenericReceiver</a>&lt;MutexType, T, A&gt;<div class=\"where\">where\n    MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>,\n    A: <a class=\"trait\" href=\"futures_intrusive/buffer/trait.RingBuf.html\" title=\"trait futures_intrusive::buffer::RingBuf\">RingBuf</a>&lt;Item = T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#856-868\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.83.0/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::Receiver","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::UnbufferedReceiver"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#946-985\">source</a><a href=\"#impl-GenericReceiver%3CMutexType,+T,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType, T, A&gt; <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html\" title=\"struct futures_intrusive::channel::shared::GenericReceiver\">GenericReceiver</a>&lt;MutexType, T, A&gt;<div class=\"where\">where\n    MutexType: 'static + <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>,\n    A: 'static + <a class=\"trait\" href=\"futures_intrusive/buffer/trait.RingBuf.html\" title=\"trait futures_intrusive::buffer::RingBuf\">RingBuf</a>&lt;Item = T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.receive\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#953-959\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html#tymethod.receive\" class=\"fn\">receive</a>(&amp;self) -&gt; <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.ChannelReceiveFuture.html\" title=\"struct futures_intrusive::channel::shared::ChannelReceiveFuture\">ChannelReceiveFuture</a>&lt;MutexType, T&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"ChannelReceiveFuture&lt;MutexType, T&gt;\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Returns a future that gets fulfilled when a value is written to the channel.\nIf the channels gets closed, the future will resolve to <code>None</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_receive\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#962-964\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html#tymethod.try_receive\" class=\"fn\">try_receive</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, <a class=\"enum\" href=\"futures_intrusive/channel/enum.TryReceiveError.html\" title=\"enum futures_intrusive::channel::TryReceiveError\">TryReceiveError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Attempt to receive form the channel without waiting.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.close\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#970-972\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html#tymethod.close\" class=\"fn\">close</a>(&amp;self) -&gt; <a class=\"enum\" href=\"futures_intrusive/channel/enum.CloseStatus.html\" title=\"enum futures_intrusive::channel::CloseStatus\">CloseStatus</a></h4></section></summary><div class=\"docblock\"><p>Closes the channel.\nAll pending future send attempts will fail.\nReceive attempts will continue to succeed as long as there are items\nstored inside the channel. Further attempts will return <code>None</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_stream\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/mpmc.rs.html#978-984\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/shared/struct.GenericReceiver.html#tymethod.into_stream\" class=\"fn\">into_stream</a>(self) -&gt; <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.SharedStream.html\" title=\"struct futures_intrusive::channel::shared::SharedStream\">SharedStream</a>&lt;MutexType, T, A&gt;</h4></section></summary><div class=\"docblock\"><p>Returns a stream that will receive values from this channel.</p>\n<p>This stream does not yield <code>None</code> when the channel is empty,\ninstead it yields <code>None</code> when it is terminated.</p>\n</div></details></div></details>",0,"futures_intrusive::channel::mpmc::if_alloc::shared::if_std::Receiver","futures_intrusive::channel::mpmc::if_alloc::shared::if_std::UnbufferedReceiver"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[11237]}