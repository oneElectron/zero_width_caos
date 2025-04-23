# A compiler's worst nightmare

Why not insert zero width characters in all text files?
This program instantly nukes your current directory by inserting zero width spaces between every single character.

This:
```
Hello World!!!
```
Becomes:
```
​H​e​l​l​o​ ​W​o​r​l​d​!​!​!​
```

Notice a difference?
Probably not. But a compiler will.

Written in 100% multithreaded and safe rust for maximum devistation.

Just for fun. dont just run it once, run it until it becomes too slow.

# How to use
```
cargo install --git https://github.com/oneElectron/zero_width_caos.git
```

run ```zwc``` it in your home folder to make sure that it works ;)
