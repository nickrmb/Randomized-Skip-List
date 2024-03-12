
---

<div align="center">
<pre>
________ ______________ 
___  __ \_/  ___/___  / 
__  /_/  /____ \ __  /  
_  _, _/ ____/ / _  /___
/_/ |_|  /____/  /_____/<br>
Randomized Skip List
</pre>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![cargo](https://img.shields.io/badge/Cargo-1.76.0-darkred.svg
)](https://crates.io/)

</div>

A [**Randomized Skip List**](https://en.wikipedia.org/wiki/Skip_list) is a data structure that allows Finding / Insertion / Deletion in $\mathcal{O}(\log n)$ on average. Therefore, it is a good alternative to [AVL Trees](https://en.wikipedia.org/wiki/AVL_tree).

<p align="center" width="100%">
    <img src="misc/skiplist.png" width=90%><br>
    <em>An example instance with solution.</em>
</p>

### SkipList

The standart SkipList.

| Method        | Description   |
|------------------------------|---------------|
| fn new() -> SkipList<T> | Creates a new (empty) SkipList  |
| fn insert(&mut self, val: T) | Inserts a new element into the SkipList |
| fn find(&self, val: &T) -> Option<Rc<T>> | Search the value into the SkipList. Returns a pointer of the value if it exists |
| fn delete(&mut self, val: &T) -> Option<Rc<T>> | Deletes the value from the SkipList. Returns it if it exists |
| fn iter(&self) -> NodeIterator<T> | Creates an iterator that iterates over all elements in SkipList in sorted order |
