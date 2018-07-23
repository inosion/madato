## Rust Questions

Q: Given a simple expression of a map from Collection<X> to Collection<Y> , how can you have the compiler tell you the type ?
A: assign it to an invalid type .. eg `let x:u32 = data.map().my_complex_transformation().collect().something()`
  and the compiler will complain. 

## Excellent Rust Posts 

* Strings and Vecs
  https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-or/40006220

* returning String and str
  https://www.reddit.com/r/rust/comments/2y2bd6/convention_for_returning_strings_str_or_string/

* the Borrow Checker
  http://xion.io/post/code/rust-borrowchk-tricks.html

* nice crates 
  http://xion.io/post/code/rust-little-crates.html

  https://beachape.com/blog/2017/05/24/rust-from-scala/

## WASM Items

* https://github.com/rustwasm/wasm-pack - publishing to npm registry

## Running the Node Sample
```
 make all # to build the WASM components 
npm install
npm run serve
```

## travis CI 

- https://dev.to/cad97/great-rust-ci-1fk6