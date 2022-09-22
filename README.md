# slink

Create symlinks without hussle.

Do you know how to use `ln -s`? Do you really know which comes first the source or the target file? Do you know what's source and what's target?

Me neither.

This is exactly why `slink` exists!

In action:

```bash
$ ls -l
total 0
-rw-r--r--  1 rumpl  staff  0 22 Sep 12:17 a.txt
$ slink a.txt b.txt
$ ls -l
total 0
-rw-r--r--  1 rumpl  staff  0 22 Sep 12:17 a.txt
lrwxr-xr-x  1 rumpl  staff  5 22 Sep 12:17 b.txt -> a.txt
$ rm b.txt
$ slink b.txt a.txt
$ ls -l
$ ls -l
total 0
-rw-r--r--  1 rumpl  staff  0 22 Sep 12:17 a.txt
lrwxr-xr-x  1 rumpl  staff  5 22 Sep 12:17 b.txt -> a.txt
```

That's right, no need to know which comes first, `slink` will always do what you want, no questions asked.

