# NAME

NAME is a super-compact binary encoding format ideal for constrained no_std environments.

# Support

Being generally thought to be used in no_std environments, NAME for now does not support allocations.

Supported

 * Zero-copy deserialization of byte-strings and `str`s.

Not supported

 * `String`s, or maps.
 * Byte slices larger than u16::MAX (65_535 bytes)
 * Enums with more than 256 variants