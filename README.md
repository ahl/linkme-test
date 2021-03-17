## linkme-test

This is a dead-simple test program for dtolnay's charming `linkme` crate. It
demonstrates a yet-to-be-filed issue on illumos. We've got a very simple
library ("lib") and very simple command ("cmd") that depends upon the library.
Both depend on `linkme`. The command creates a part of the "distributed slice" which we then expect the library to invoke. This works properly on macOS and Linux, but as of this writing (2021-03-16) it behaves incorrectly on illumos.

Under illumos the build is clean, but we see no output at runtime. A careful
inspection of the linker shows that it is quietly complaining:

```
ld: warning: reserved symbol '__start_set_linkme_DOERS' already defined in file /home/ahl/linkme-test/cmd/target/debug/deps/liblib-b2cc6c3925bf49c0.rlib(lib-b2cc6c3925bf49c0.1t2rea17ejr80ouk.rcgu.o)
ld: warning: reserved symbol '__stop_set_linkme_DOERS' already defined in file /home/ahl/linkme-test/cmd/target/debug/deps/liblib-b2cc6c3925bf49c0.rlib(lib-b2cc6c3925bf49c0.1t2rea17ejr80ouk.rcgu.o)
```

This output is not, however, shown in the terminal from which `cargo build` is
run nor is it, apparently, logged anywhere.

The resulting binary contains not one, but two ELF sections named
`__set_linkme_DOERS` and two sets of `__start...` and `__stop...` symbols. It
seems that the mechanisms of `linkme` end up procssing the section with
`sh_size` set to zero.