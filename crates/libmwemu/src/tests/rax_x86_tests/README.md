This x86 specific test-set are created by Kenan Sulayman https://github.com/19h/ 
and is part of his RAX project:

`https://github.com/19h/rax/tree/master/tests/x86_64`

The tests have been converted to use libmwemu's api using some regex I prepared on regex.vim


To trigger this tests use:
```
cargo test --features rax_x86_tests rax_x86_tests
```
