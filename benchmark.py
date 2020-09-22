try:
    from dlibs.logger import set_logging_format_none
    from dlibs.logger import set_logging_msg_prefix_len
    set_logging_msg_prefix_len(20)
    set_logging_format_none()
    from dlibs.testing import execute_tests, ttimeit
except:
    from testing import execute_tests, ttimeit


import re
import re2
import cffi_re2
import rure
import regex

import rustlibs
from sample_txt import sample as _sample


_pattern = r'([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)'


@ttimeit(20000, _sample)
def using_re(text):
    '''
    Using python builtin module re.
    '''
    return re.findall(_pattern, text)


@ttimeit(20000, _sample)
def using_re2(text):
    '''
    There is official(?) python module
     - https://github.com/facebook/pyre2/
    But it does not contains findall method.

    I can find 'findall' method on sw-re2
     - https://github.com/StephenWattam/pyre2/
    It using cython.

    '''
    pattern = re2.compile(
        ("([a-z0-9!#$%&'*+\/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+\/=?^_`"
         "{|}~-]+)*(@|\sat\s))"))
    return pattern.findall(text)


@ttimeit(20000, _sample)
def using_cffi_re2(text):
    '''
    It using cffi
     - https://github.com/vls/cffi_re2
    '''
    pattern = cffi_re2.compile(
        ("([a-z0-9!#$%&'*+\/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+\/=?^_`"
         "{|}~-]+)*(@|\sat\s))"))
    return pattern.findall(text)


@ttimeit(20000, _sample)
def with_regex(text):
    '''
     - https://bitbucket.org/mrabarnett/mrab-regex/src/hg/
    '''
    return regex.findall(_pattern, text)


@ttimeit(20000, _sample)
def with_rure(text):
    '''
    It is python wrapper for rust regex. It using C API of rust regex.
     - https://github.com/rust-lang/regex/tree/master/regex-capi
     -
    '''
    return rure.findall(_pattern, text)


@ttimeit(20000, _sample)
def with_rust_regex(text):
    '''

    There is other binding
     - https://github.com/davidblewett/regex/tree/rure-python/regex-capi/src/python
    '''
    return rustlibs.findall_with_rust_regex(_pattern, text)


@ttimeit(1, _sample, 20000)
def with_loop_on_rust(text, count):
    return rustlibs.findall_loop_on_rust(_pattern, text, count)


if __name__ == '__main__':
    execute_tests()
