# libatasmart
Rust friendly interface for libatasmart-sys
[![Docs](https://docs.rs/libatasmart/badge.svg)](https://docs.rs/libatasmart)

Using libatasmart *might* require some compiling. If you're using ubuntu you can install [libatasmart-dev](https://packages.ubuntu.com/search?keywords=libatasmart) and that should be
sufficient. Otherwise head on over to http://git.0pointer.net/libatasmart.git/ and pull down the latest copy of the code.  Follow the readme instructions in there to get yourself a libatasmart shared library installed.  From there this code should work fine.

The backing C library for this code was written roughly 14yrs ago and hasn't really been updated since. If anyone is interested I think it would be worth rewriting this wrapper interface to parse the data blobs given back by the hard drives directly and then just remove the C dependency all together. 
