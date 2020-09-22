# purpose

I am a python developer. I want to use rust programming language instead of C/C++ for performance.

I had a test comparison of the performance of regex libraries with email regexp.
 - using_re (https://docs.python.org/3/library/re.html)
 - using_re2 (https://github.com/google/re2)
 - using_cffi_re2 (https://github.com/google/re2)
 - with_regex (https://bitbucket.org/mrabarnett/mrab-regex/src/hg/)
 - with_rure (https://github.com/davidblewett/rure-python)
 - with_rust_regex (https://github.com/rust-lang/regex)
 - with_loop_on_rust (native execute rust-lang/regex)

 - hyperscan (Not included. It depends hardware.)


# execute

```console
$ git clone https://github.com/ptmono/benchmark_regex.git
$ make init-env
$ make benchmark
```


# result

```console
...
. ./_venv/bin/activate; python3.6 benchmark.py
D# 119: using_re            : 1.2516160369996214
D# 119: using_re2           : 0.6452168539999548
D# 119: using_cffi_re2      : 3.7915563010010374
D# 119: with_regex          : 2.6764859760005493
D# 119: with_rure           : 21.759336847999293
D# 119: with_rust_regex     : 56.8959412180011
D# 119: with_loop_on_rust   : 56.60304277400064
```
