# Push Decoding & Pull Encoding

This crate provides abstractions for push-based decoding and pull-based encoding.
That means, the caller is responsible for obtaining the bytes to decode and feeding them into
decoder or pulling bytes from encoder and feeding them into writr.

The main advantage of this approach is that it's IO-agnostic, which implies both
**`async`-agnostic** and `no_std`. You can use the same code to deserialize from sync
and `async` readers and only need a tiny piece of code to connect the reader to a decoder. This
piece of code is provided by this crate for `std`, `tokio`, and `futures`.

# Features

* `std` - enables integration with the standard library - it's IO and error traits
* `alloc` - enables integration with the standard `alloc` crate
* `tokio` - connects decoders to Tokio IO.
* `futures_0_3` - connects decoders to futures 0.3.x IO

# Attribution

This is a simplified fork of the original [`push_decode`](https://github.com/Kixunil/push_decode) crate by Martin Habovstiak ([Kixunil](https://github.com/Kixunil)) created for educational purposes. 
