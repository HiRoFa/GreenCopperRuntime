(function() {
    var type_impls = Object.fromEntries([["futures_intrusive",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#336-342\">source</a><a href=\"#impl-Debug-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html\" title=\"struct futures_intrusive::channel::GenericStateBroadcastChannel\">GenericStateBroadcastChannel</a>&lt;MutexType, T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#339-341\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.83.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.83.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","futures_intrusive::channel::state_broadcast::if_std::StateBroadcastChannel","futures_intrusive::channel::state_broadcast::LocalStateBroadcastChannel"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#344-404\">source</a><a href=\"#impl-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>, T&gt; <a class=\"struct\" href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html\" title=\"struct futures_intrusive::channel::GenericStateBroadcastChannel\">GenericStateBroadcastChannel</a>&lt;MutexType, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#349-356\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html\" title=\"struct futures_intrusive::channel::GenericStateBroadcastChannel\">GenericStateBroadcastChannel</a>&lt;MutexType, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h4></section></summary><div class=\"docblock\"><p>Creates a new State Broadcast Channel in the given state</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.send\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#364-366\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html#tymethod.send\" class=\"fn\">send</a>(&amp;self, value: T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.83.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"futures_intrusive/channel/struct.ChannelSendError.html\" title=\"struct futures_intrusive::channel::ChannelSendError\">ChannelSendError</a>&lt;T&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Writes a single value to the channel.</p>\n<p>This will notify waiters about the availability of the value.\nIf the maximum amount of values had been written to the channel,\nor if the channel is closed, the new value will be rejected and\nreturned inside the error variant.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.close\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#374-376\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html#tymethod.close\" class=\"fn\">close</a>(&amp;self) -&gt; <a class=\"enum\" href=\"futures_intrusive/channel/enum.CloseStatus.html\" title=\"enum futures_intrusive::channel::CloseStatus\">CloseStatus</a></h4></section></summary><div class=\"docblock\"><p>Closes the channel.</p>\n<p>This will notify waiters about closure, by fulfilling pending <code>Future</code>s\nwith <code>None</code>.\n<code>send(value)</code> attempts which follow this call will fail with a\n<a href=\"futures_intrusive/channel/struct.ChannelSendError.html\" title=\"struct futures_intrusive::channel::ChannelSendError\"><code>ChannelSendError</code></a>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.receive\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#386-395\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html#tymethod.receive\" class=\"fn\">receive</a>(&amp;self, state_id: <a class=\"struct\" href=\"futures_intrusive/channel/struct.StateId.html\" title=\"struct futures_intrusive::channel::StateId\">StateId</a>) -&gt; <a class=\"struct\" href=\"futures_intrusive/channel/struct.StateReceiveFuture.html\" title=\"struct futures_intrusive::channel::StateReceiveFuture\">StateReceiveFuture</a>&lt;'_, MutexType, T&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"StateReceiveFuture&lt;&#39;_, MutexType, T&gt;\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Returns a future that gets fulfilled when a value is written to the channel\nor the channel is closed.\n<code>state_id</code> specifies the minimum state ID that should be retrieved\nby the <code>receive</code> operation.</p>\n<p>The returned <a href=\"futures_intrusive/channel/struct.StateReceiveFuture.html\" title=\"struct futures_intrusive::channel::StateReceiveFuture\"><code>StateReceiveFuture</code></a> will get fulfilled with the\nretrieved value as well as the <a href=\"futures_intrusive/channel/struct.StateId.html\" title=\"struct futures_intrusive::channel::StateId\"><code>StateId</code></a> which is required to retrieve\nthe following state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_receive\" class=\"method\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#401-403\">source</a><h4 class=\"code-header\">pub fn <a href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html#tymethod.try_receive\" class=\"fn\">try_receive</a>(&amp;self, state_id: <a class=\"struct\" href=\"futures_intrusive/channel/struct.StateId.html\" title=\"struct futures_intrusive::channel::StateId\">StateId</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.83.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;(<a class=\"struct\" href=\"futures_intrusive/channel/struct.StateId.html\" title=\"struct futures_intrusive::channel::StateId\">StateId</a>, T)&gt;</h4></section></summary><div class=\"docblock\"><p>Attempt to retrieve a value whose <code>StateId</code> is greater than the one provided.</p>\n<p>Returns <code>None</code> if no value is found in the channel, or if the current <code>StateId</code>\nof the value is less or equal to the one provided.</p>\n</div></details></div></details>",0,"futures_intrusive::channel::state_broadcast::if_std::StateBroadcastChannel","futures_intrusive::channel::state_broadcast::LocalStateBroadcastChannel"],["<section id=\"impl-Send-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#326-329\">source</a><a href=\"#impl-Send-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html\" title=\"struct futures_intrusive::channel::GenericStateBroadcastChannel\">GenericStateBroadcastChannel</a>&lt;MutexType, T&gt;</h3></section>","Send","futures_intrusive::channel::state_broadcast::if_std::StateBroadcastChannel","futures_intrusive::channel::state_broadcast::LocalStateBroadcastChannel"],["<section id=\"impl-Sync-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/futures_intrusive/channel/state_broadcast.rs.html#331-334\">source</a><a href=\"#impl-Sync-for-GenericStateBroadcastChannel%3CMutexType,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;MutexType: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.83.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.GenericStateBroadcastChannel.html\" title=\"struct futures_intrusive::channel::GenericStateBroadcastChannel\">GenericStateBroadcastChannel</a>&lt;MutexType, T&gt;</h3></section>","Sync","futures_intrusive::channel::state_broadcast::if_std::StateBroadcastChannel","futures_intrusive::channel::state_broadcast::LocalStateBroadcastChannel"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[11457]}