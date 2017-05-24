enum_error
==========

Provides macros to generate `std::error::Error` and `std::fmt::Display` implementations for simple enumeration types.

[![Build Status](https://travis-ci.org/tgockel/enum_error.svg?branch=master)](https://travis-ci.org/tgockel/enum_error)

Usage
-----

In `Cargo.toml`:

    [dependencies]
    enum_string = { git = "https://github.com/tgockel/enum_error" }

In your project:

    #[macro_use]
    extern crate enum_error;

    #[derive(Debug, EnumDisplay, EnumError)]
    enum BasicError {
        A,
        B,
        C,
    }

Now, you can happily use `BasicError` values wherever `std::error::Error` is appropriate.

License
-------

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
the License. You may obtain a copy of the License at

  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
