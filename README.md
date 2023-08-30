# char\_index
A crate for ~O(1) charwise indexing into a string, without normally using as much memory as a `Vec<char>`.

# Benchmarks
1000 "e"'s with 3 extra non ascii chars, shuffled
|Implementation|Memory Use|Index 200th codepoint|
|-|-|-|
|Vec<char>|4012B|0.6ns|
|IndexedChars|2009B|4ns|
|String|1006B|126ns|

# `no_std`
This crate is fully `no_std`, however it does rely on alloc.  
A std feature may be added at a later date, but it is currently unknown what that would include.

# License
This crate is licensed under MPL-2.0, this is a weak copyleft license intended to keep any modifications 
to the core library open source, without affecting other requirements greatly. This is not legal advice.
