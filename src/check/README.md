Content-Security-Policy checker
===============================

This module (will) take a parsed CSP and check URLs against it.

It'll probably use a <https://en.wikipedia.org/wiki/Trie> to perform the URL checks?
The rest is pretty basic (expose a boolean of whether sandbox is on or off, a hash set for plugin types, etc).
