#Ownership notes
Three important ownership rules
1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Knowing when data is put ont he stack or heap is important in rust