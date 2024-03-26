(function() {var implementors = {
"bytes":[["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/trait.Buf.html\" title=\"trait bytes::buf::Buf\">Buf</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> for <a class=\"struct\" href=\"bytes/buf/struct.Reader.html\" title=\"struct bytes::buf::Reader\">Reader</a>&lt;B&gt;"]],
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,</div>"]],
"futures_util":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> for <a class=\"struct\" href=\"futures_util/io/struct.AllowStdIo.html\" title=\"struct futures_util::io::AllowStdIo\">AllowStdIo</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,</div>"]],
"schannel":[["impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> for <a class=\"struct\" href=\"schannel/tls_stream/struct.TlsStream.html\" title=\"struct schannel::tls_stream::TlsStream\">TlsStream</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()