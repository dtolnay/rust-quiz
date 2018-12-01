Answer: 3311
Difficulty: 1

# Hint

All traits have an independent set of methods.


# Explanation

The trait `Base` has a default method `method`. Its impl for `OnlyBase` overrides that default method. Default methods are basically sugar for "copy this method into each trait impl that doesn't explicitly define this method". Once you override the default method there is no way to call the original default method for that given type. So both static and dynamic dispatch on `OnlyBase` produce the number `3`, from calling the `method` from `impl Base for OnlyBase`.

While subtraits _can_ define methods conflicting with the base trait, these are _independent_ methods, and do not override the original. Trait inheritance does not override methods, trait inheritance is a way of saying "all implementors of this trait _must_ implement the parent trait". Both `stat()` and `dynamic()` refer to the trait `Base`, so we look for a `method()` from `impl Base for OnlyBase`, which turns out to be the default method, so both these calls produce `1`.