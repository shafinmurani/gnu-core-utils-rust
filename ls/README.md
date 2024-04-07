
# LS in rust

This is a Rust project that I made to gain some confidence with the rust programming language, this is a _ripoff_ of the `ls` utility in Linux.

To use it, \
Passing no arguments lists files and directories in the current directory
```
$ cargo run

./.git
./target
./src
./Cargo.toml
./Cargo.lock
./.gitignore
```
You can also pass in multiple folder names to list out multiple directories and files as well
```
$ cargo run / ../ /home

Files in /:
/tmp
/run
/home
/srv
/var
/etc
/lib64
/boot
/lib
/usr
/root
/opt
/sbin
/mnt
/bin
/strings
/dev
/.dockerenv
/sys
/proc

Files in ../:
../ls
../minigrep

Files in /home:
```

