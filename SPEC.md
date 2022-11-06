# UGON Specification v1

Introducing: **UTF-8 Gap-based Object Notation**!

> *U gon encode all ze things!*

> In progress / under construction.

## Goals

> **TODO:** Make intro blurp shorter!

- Allow *decoders* to support only *parts* of the spec via *embedded feature flags*.
  - No flags set? It's literally JSON, but binary.
  - All flags set? Who needs safety anyway!
- Completely *self-describing* (i.e.: schemaless), just like JSON; meaning:
  - Every file must be fully resolvable to a 'dumb' tree of tags & values.
  - Files must be readable *outside* of the applications generating them.
  - All bit-patterns that a decoder *may* encounter are defined via spec.
  - Data can be appended/edited/removed without fully decoding a file.
  - If one knows the format, files can be edited via hex-editor!
- Compact data representation and speedy decoding...
  - Common cases, patterns *and* values take *way* less space.
  - Most data is usable without decoding; memcopy away!
  - Decoder hints to allow for pre-allocation and memcopy.
  - Allow blobs of both structured *and* raw data.
  - Fully streamable!

How does any of this work? Well, read on!

## Semantics

...?

### File Structure

Any valid ugon-file **must** start with a [environment setup](#environment-setup) tag which,
among other things, contains the magic-bytes identifying the ugon format.

Following the setup tag, there may be one or more [environment tags](#environment-tags) (but no setup tags!).

*After* all environment tags, there will be a single tag that forms the <span id='root'>**root**</span> of the files UGON tree.

Because of this, the root of a UGON tree is, regardless of any nested files, the first non-[environment](#environment-tags) tag of the file.

Tags in root-position *may* omit their terminator, if they have one.

### UTF-8 Gaps

A normal/well-formed UTF-8 string consists of codepoints, each encoded as a sequence of 1 to 4 bytes, with the leading-bits of the first byte determining the number of bytes.

This results in the following **Unicode Codepoint** to **UTF-8** mapping:

| Codepoint Range | Bit Patterns |
|:---------------:|:-------------|
| `U+0000 .. U+007F` | `0xxxxxxx` |
| `U+0080 .. U+07FF` | `110xxxxx 10xxxxxx` |
| `U+0800 .. U+FFFF` | `1110xxxx 10xxxxxx 10xxxxxx` |
| `U+10000 .. U+10FFFF` | `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx` |

Given these rules, the following bit patterns will *not* occur in a well-formed UTF-8 string:

| Bit Patterns | Description |
|:------------:|:------------|
| `1000xxxx` / `0x8x` | A continuation byte. |
| `1001xxxx` / `0x9x` | A continuation byte. |
| `1010xxxx` / `0xAx` | A continuation byte. |
| `1011xxxx` / `0xBx` | A continuation byte. |
| `11000000` / `0xC0` | Overlong A.   |
| `11000001` / `0xC1` | Overlong B.   |
| `11111110` / `0xFE` | Unassigned A. |
| `11111111` / `0xFF` | Unassigned B. |

We will henceforth call these patterns *gap values*.

### VarInts

The binary format for variable-length integers is as follows, with bits stored in big-endian/network order:

| Bits | Bit Pattern |
|-----:|-------------|
|  `7` Bits | `0_xxxxxxx` |
| `14` Bits | `10_xxxxxx xxxxxxxx` |
| `21` Bits | `110_xxxxx xxxxxxxx xxxxxxxx` |
| `28` Bits | `1110_xxxx xxxxxxxx xxxxxxxx xxxxxxxx` |
| `(n+4)*8` Bits | `1111_nnnn` and `n+4` payload bytes |
| — | — |
| `(0+4)*8` / `32` | `1111_0000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx` |
| `(1+4)*8` / `40` | `1111_0001 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx` |
| `(2+4)*8` / `48` | `1111_0010 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx` |
| `(4+4)*8` / `64` | `1111_0100 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx` |

If the feature-flag `v128` is *not* set, variable integers are limited to 64 bits in length.

When the feature flag `v128` *is* enabled:

| Bits | Bit Pattern |
|-----:|-------------|
| `(5+4)*8` / `52` | `1111_1000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx` |
| ... | ... |
| `(12+4)*8` / `128` | `1111_1100 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx, xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx` |

When the feature flag `vpow` (which requires `v128`) *is* enabled:
| Bits | Bit Pattern |
|-----:|-------------|
| `256` | `1111_1101 (xxxxxxxx * 32)` |
| `512` | `1111_1110 (xxxxxxxx * 64)` |
| `1024` | `1111_1111 (xxxxxxxx * 128)` |

### Keys

A key is any valid UTF-8 encoded string up to `255` bytes in length; empty keys are *never* allowed.

If the feature flag `kl24` is enabled, keys are instead limited to a maximum of `24` bytes.

#### Gap-valued Keys
*Feature flag:* `gvk`  

By default all structures that accept *strings as keys*, require that the keys are terminated by an `END`-tag.

Since one of the goals of UGON is to not waste space whenever reasonably possible, we allow the use of [gap-values](#utf-8-gaps), as so-called **gap-valued keys**, in *all* places where string-keys may be used.

That said, the following mapping shows which *gap-value* is mapped to what *character*:

| Bit-Pattern | Character |
|:----------|-----------|
| `10_00_0000` / `0x80` | `@` |
| `10_00_0001` - `10_00_1111` / `0x81 .. 0x8F` | `A` - `O` |
| `10_01_0000` - `10_01_1010` / `0x90 .. 0x9A` | `P` - `Z` |
| `10_01_1011` / `0x9B` | `#` |
| `10_01_1100` / `0x9C` | `$` |
| `10_01_1101` / `0x9D` | `%` |
| `10_01_1110` / `0x9E` | `&` |
| `10_01_1111` / `0x9F` | `_` |
| `10_10_0000` / `0xA0` | `-` |
| `10_10_0001` - `10_11_1111` / `0xA1 .. 0xAF` | `a` - `o` |
| `10_11_0000` - `10_11_1010` / `0xB0 .. 0xBA` | `p` - `z` |
| `10_11_1011` / `0xBB` | `=` |
| `10_11_1100` / `0xBC` | `/` |
| `10_11_1101` / `0xBD` | `+` |
| `10_11_1110` / `0xBE` | `*` |
| `10_11_1111` / `0xBF` | `?` |

> **Note:**  
> The gap-values that map to *alphabetic* characters
> have *the same* lowest 6 bits as their ASCII counterparts.

You may notice that the selection of special characters is strangely limited;
this is on purpose, don't ask for more.

#### Cached Keys
*Feature Flag:* `cfetch`

In any place that a key can appear, the [cache fetch tag](#cache-tags) (bit pattern `11000000`)
may be used to fetch a cached string, to be used as a key in place of itself.

### Tags

There are sixty-eight (68) possible *types* of tag, split into five (5) *families* of tags, with two (2) special-purpose tags not being part of any family:

| Bit Pattern | Description |
|:-----------:|-------------|
| `10_00_xxxx` | [**Empty** Tags](#empty-tags):<br> Each tag defines a specific constant value; there are *no* payload bytes. |
| `10_01_xxxx` | [**Const** Tags](#const-tags):<br> Each tag has a value, encoded in the *specified number* of payload bytes. |
| `10_10_xxxx` | [**Dynamic** Tags](#dynamic-tags):<br> Each tag has it's own encoding/structure and may be of *any* size. |
| `10_11_xxxx` | [**Environment** Tags](#environment-tags):<br> Tags affect parsing behaviour by modifying the current *environment*. |
| `11_00_000x` | [**Cache** Tags](#cache-tags):<br>Tags that replace themselves with values from the decoders cache. |
| `11_11_1110` | The special [`PAD`-tag.](#pad-tag) |
| `11_11_1111` | The special [`END`-tag.](#end-tag) |

#### Empty Tags

Tags that are 'empty' require no further bytes to be read; they are inlined constant values.

Due to this one-to-one correspondence, the following table defines all possible values...

| Bit Pattern / Hex Value | Constant Value |
|------------------------:|----------------|
| `10_00_0000` / `0x80` | The integer `0`. |
| `10_00_0001` / `0x81` | The integer `1`. |
| `10_00_0010` / `0x82` | The integer `2`. |
| `10_00_0011` / `0x83` | The integer `3`. |
| `10_00_0100` / `0x84` | The integer `4`. |
| `10_00_0101` / `0x85` | The integer `5`. |
| `10_00_0110` / `0x86` | The integer `6`. |
| `10_00_0111` / `0x87` | The integer `7`. |
| `10_00_1000` / `0x88` | The integer `8`. |
| `10_00_1001` / `0x89` | The integer `9`. |
| `10_00_1010` / `0x8A` | Positive infinity |
| `10_00_1011` / `0x8B` | Negative infinity |
| `10_00_1100` / `0x8C` | Boolean `false` |
| `10_00_1101` / `0x8D` | Boolean `true` |
| `10_00_1110` / `0x8E` | Non-signalling Not-a-Number (`NaN`) |
| `10_00_1111` / `0x8F` | Nil/None/Nothing (*not* the same as the `END` tag) |





#### Const Tags

Tags that are 'constant' indicate that the specified number of following bytes,
represent a single value of the specified kind, whose endianness is set by the current environment.

Since the number of payload bytes, and how to decode them,
is all that is needed to describe *const-tags*,
the following table covers the entire parsing space:

| Bit Pattern / Hex Value | Payload<br>Bytes | Decode as... |
|------------------------:|--------------:|-----------|
| `10_01_0000` / `0x90` |  `1` | A unsigned 8-bit integer / `u8`. |
| `10_01_0001` / `0x91` |  `2` | A unsigned 16-bit integer / `u16`. |
| `10_01_0010` / `0x92` |  `2` | A unsigned 24-bit integer / `u24`. |
| `10_01_0011` / `0x93` |  `4` | A unsigned 32-bit integer / `u32`. |
| `10_01_0100` / `0x94` |  `5` | A unsigned 48-bit integer / `u48`. |
| `10_01_0101` / `0x95` |  `8` | A unsigned 64-bit integer / `u64`. |
| `10_01_0110` / `0x96` | `16` | A unsigned 128-bit integer / `u128`. |
| `10_01_0111` / `0x97` |  `1` | A signed 8-bit integer / `i8`. |
| `10_01_1000` / `0x98` |  `2` | A signed 16-bit integer / `i16`. |
| `10_01_1001` / `0x99` |  `4` | A signed 32-bit integer / `i32`. |
| `10_01_1010` / `0x9A` |  `8` | A signed 64-bit integer / `i64`. |
| `10_01_1011` / `0x9B` | `16` | A signed 128-bit integer / `i128`. |
| `10_01_1100` / `0x9C` |  `2` | A 16-bit floating-point number / `f16`. |
| `10_01_1101` / `0x9D` |  `4` | A 32-bit floating-point number / `f32`. |
| `10_01_1110` / `0x9E` |  `8` | A 64-bit floating-point number / `f64`. |
| `10_01_1111` / `0x9F` | n/a | (*unassigned on purpose*) |





#### Dynamic Tags

Tags that are 'dynamic'-sized, either directly define their size thru their immediately following byte/s,
or are *streamed*, in which case the tag defines a *terminator tag* that ends it.

##### Positive Variable-Length Integer
*Bit-Pattern:* `10_10_0000`
*Terminator:* n/a  

Encoded as [variable-length integer](#varints), this tag represents an arbitrary positive integer,
in the inclusive range from `0` to `2^1024`.

##### Negative Variable-Length Integer
*Bit-Pattern:* `10_10_0001`
*Terminator:* n/a  

The format of a negative variable-length integer is *exactly the same* as [the positive one](#positive-variable-length-integer),
except that the decoded integer value is negated before being returned; the resulting range goes from `0` to `-2^1024`

##### Const Array
*Bit-Pattern:* `10_10_0010`
*Terminator:* n/a  

An **array** made of [const sized](#const-tags) values.

This structure is for both raw and structured binary data;  
i.e.: arrays of either primitives or structs.

*Payload:*
- One byte, split into *two nibbles*:
  | Bits `7,6,5,4` | Bits `3,2,1,0` |
  |:---------------|---------------:|
  | **Top** Nibble | **Low** Nibble |
- The **top nibble** declares the type of the elements,  
  by matching with the 4 lowest bits of a [const-sized tag](#const-tags).
- The **low nibble** is the *amount* of elements.
- If the amount is `0xF`: read a [varint](#varints) and use that.
- The varint-encoded amount *may* be less-or-equal than `0xF`.
- After the payloads of the nibbles follows the array payload.

###### Const Array of Structures
*Feature flag:* `aos`  

If the type-nibble is `0xF`, a struct is defined instead:

A struct is defined by a fixed-length list of tags:
- The *length* of the list is given as varint integer.
- The list consists of *pairs* of names and const-tags.
  - The const-tags are written *without* their payloads.
  - The name *must* be either:
    - A valid non-empty zero-terminated UTF-8 string.
    - A [gap-valued key](#gap-valued-keys).
  - The length of the name *must not* exceed 24 bytes.
- `PAD`-tags; each defining a zero-byte (`0x00`).

- The size of the struct is the *sum* of the  
  *unwritten payloads* and the *amount of pad-tags as bytes*.
- If the decoder implementation cannot use dynamic typing,
  it *must* decode the structs into an *array of bytes/`u8`*
  instead, while ensuring proper alignment and endianness.

##### GVEI Array
*Bit-Pattern:* `10_10_0011`  
*Terminator:* n/a  
*Feature flag:* `gvei`  

Array of [Group Varint Encoded](https://en.wikipedia.org/wiki/Variable-length_quantity#Group_Varint_Encoding) integers.

*Payload:*
- A varint determining the *amount of elements* to be read.
- The amount of *groups* is the *amount* divided by four (4).
- Followed by the given amount of integer groups.

##### (reserved)
*Bit-Pattern:* `10_10_0100`  

##### Fixed-Size String
*Bit-Pattern:* `10_10_0101`  
*Terminator:* n/a  

Fixed-Size String of UTF-8 encoded codepoints.
- The byte-length of the string is encoded as varint integer.
- Followed by the given number of bytes making up the contents of the string.

##### Terminated String
*Bit-Pattern:* `10_10_0110`  
*Terminator:* [String Terminator](#string-terminator)  

A terminated string of valid UTF-8 encoded codepoints, without any [gap-values](#utf-8-gaps) inside it.

Since the string *must* be a byte-stream of valid UTF-8, this tag is allowed to *omit itself*, effectively turning the first UTF-8 codepoint into a string tag. It is the *only* tag that can do so.

##### String Terminator
*Bit-Pattern:* `10_10_0111`  
*Terminator:* n/a  

Terminates a [string](#terminated-string); the bit-pattern was specifically chosen to be `10_10_0111`, matching the ASCII-character `§`, for easy identification in hex-editors.

##### List
*Bit-Pattern:* `10_10_1000`  
*Terminator:* [List Terminator](#list-terminator)  

A terminated list of tags.

*Merging:* The template is cloned and the payloaded tag *appended*.

##### List Terminator
*Bit-Pattern:* `10_10_1001`  
*Terminator:* n/a  

Terminates a [list](#list-terminator).

##### String-keyed Dictionary
*Bit-Pattern:* `10_10_1010`  
*Terminator:* [`END`-tag](#end-tag)  
*Feature flag:* `skey`  

A terminated dictionary of tags with string/[gap](#gap-valued-keys)-keys.

The dictionary is terminated by the `END`-tag; the terminator is optional if the dictionary is the [root](#root).

*Merging:* The template is cloned, with the payloaded tag *replacing* equal keys.

##### Integer-keyed Dictionary
*Bit-Pattern:* `10_10_1011`  
*Terminator:* [`END`-tag](#end-tag)  
*Feature flag:* `ikey`

A dictionary of tags with either [constant-](#const-tags) or [variable](#varints)-sized integer-keys.

The dictionary is terminated by the `END`-tag; the terminator is optional if the dictionary is the [root](#root).

*Merging:* The template is cloned, with the payloaded tag replacing equal keys.

##### Table of Dictionaries
*Bit-Pattern:* `10_10_1100`  
*Terminator:*  `END`-Tag  
*Feature flag:* `table`

A table of tags with named columns; equivalent to a list of dictionaries that all have the same keys.

The table is terminated by the `END`-tag; the terminator is optional if the table is the [root](#root).

An optimized form for a list of string-keyed dictionaries.

##### ?
*Bit-Pattern:* `10_10_1101`
*Terminator:* n/a  

##### ?
*Bit-Pattern:* `10_10_1110`
*Terminator:* n/a  

##### Attributes
*Bit-Pattern:* `10_10_1111`  
*Terminator:* n/a  

The Attributes tag is a length-prefixed set of *attributes*, followed by a single *content* tag.

- The *content* tag **must not** be an attribute tag itself.
- If the attribute tag is the [root](#root), the *content* tag *may* omit it's terminator, if it has one.





#### Environment Tags

The tags of the *environment* family modify and extend the *current parsing environment*.

```rust
struct UgonEnvironment {
  /// A cache of values using the LRU replacement policy.
  /// 
  /// Once full, least-recently-used values are discarded.
  lru_cache: LRUCache<UgonTag>,
  
  /// A cache of values referenced via key.
  map_cache: HashMap<String, UgonTag>,
  
  /// The endianness to decode integers from; set by header tag.
  endian: Endianness,
}
```

##### Environment Setup
*Bit-Pattern:* `10_11_0000`

The environment setup tag declares that a UGON-(sub)file begins.

*Payload:*
- Format Identifier/'Magic Bytes', depending on the contents endianness:
  - Big-Endian: `UGON` (`0x55 0x47 0x4F 0x4E`)
  - Lil'Endian: `ugon` (`0x75 0x67 0x6F 0x6E`)
- Format Version: `-V` (`0x2D V`)
  - Default is `1`/`0x31`.
  - If set to `0x00`, a [terminated string](#terminated-string)
    containing a link to the used specification follows.
- List of comma-separated *feature-flags*, terminated by a zero-byte.
  - If a flag is prefixed with `-`, it means to *disable* that feature.
    - Feature flag `*`: Enable all features this spec defines.

- *Effect:*
  - Setups a new current environment.
  - Sets the endianness used by the content.
  - Creates a new -empty- cache.

##### ?
*Bit-Pattern:* `10_11_0001`

##### ?
*Bit-Pattern:* `10_11_0010`

##### ?
*Bit-Pattern:* `10_11_0011`

##### ?
*Bit-Pattern:* `10_11_0100`

##### ?
*Bit-Pattern:* `10_11_0101`

##### ?
*Bit-Pattern:* `10_11_0110`

##### ?
*Bit-Pattern:* `10_11_0111`

##### LRU Cache Insert
*Bit-Pattern:* `10_11_1000`

- *No payload.*
- Pushes the next non-environment tag into the current environments LRU cache.
- If the value is already in the cache: Moves it to the top.

##### MAP Cache Insert
*Bit-Pattern:* `10_11_1001`  
*Feature flag:*  `cmap`

- *Payload:* A [key](#keys).
- Puts the next non-environment tag into the current environments MAP cache.
- Existing keys in the MAP cache must not be overridden.

##### ?
*Bit-Pattern:* `10_11_1010`

##### ?
*Bit-Pattern:* `10_11_1011`

##### ?
*Bit-Pattern:* `10_11_1100`

##### Byte-Length Hint
*Bit-Pattern:* `10_11_1101`

- *Payload:*  
  A single varint integer, defining some minimum size in bytes,
  of the next tag that follows.

##### Data-Amount Hint
*Bit-Pattern:* `10_11_1110`

- *Payload:*  
  A single varint integer, defining that the tag that follows
  has the given minimum amount of elements.

##### Custom Hint
*Bit-Pattern:* `10_11_1111`  
*Feature flag:* `chint`  

A tag that is to be used for parsing *hints* as inserted by an encoder/application; it *must not* modify behaviour of the UGON decoder itself.

*Payload:*
  - A [terminated string](#terminated-string), **or**, a single [gap-value](#utf-8-gaps).
  - Zero or more [environment tags](#environment) to be applied to the data.
  - A single non-environment tag holding data.





#### Cache Tags

*Bit-Pattern* `11000000`: The <span id='cache-fetch'>**cache *fetch***</span> tag.  
*Feature Flag:* `cfetch`  

- Loads the referenced value from the cache
  and pretends it was here the whole time,
  instead of the cache tag.
- *Payload:* 
  - A [cache reference](#cache-reference).



*Bit-Pattern* `11000001`: The <span id='cache-merge'>**cache *merge***</span> tag.  
*Feature Flag:* `cmerge`  

- ***Warning:*** Enabling this feature effectively makes the decoder turing-complete via lambda calculus.
- The purpose of merging is strictly to *deduplicate* data.
- Loads the referenced value from the cache
  and uses it as a 'template' value for the payloaded tag.
- The type of the template tag and the payload tag **must** be *exactly* the same.
- If a type has no defined merge behaviour, the decoder **must** throw an error.
- Merging is *always* shallow, *never* recursive.
- *Payload:* 
  - A [cache reference](#cache-reference).
  - A tag that contains data.
    - Must *not* be a cache tag.
    - Must *not* be a environment tag.

#### Pad Tag
*Bit-Pattern:* `11_11_1110`  

#### End Tag
*Bit-Pattern:* `11_11_1111`  





### Caching
> Enabled via either feature-flags `cfetch` and/or `cmerge`.

Caching exists to optimize oft-repeated keys and values, massively reducing file-size and making parsing faster, due to there being less data to shuffle/copy around.

By default *only* strings can be inserted into the cache;
when the feature flag `cdyn` is set, any non-environment tag may be inserted.

#### LRU Cache
> Enabled via either feature-flags `cfetch` and/or `cmerge`.

?

#### MAP Cache
> Enabled via feature-flag `cmap` **and** `cfetch` or `cmerge`.

A cache of values referenced via key.

?

#### Cache Reference

There are two types of cache reference, as used by the [cache tags](#cache-tags): **Numeric** and **Named**.

A *numeric* cache reference is a [varint](#varints) limited to `28` bits (limit of `268435456` entries, maximum bit-pattern of `1110_xxxx xxxxxxxx xxxxxxxx xxxxxxxx`), that is a *reverse*-index into the [LRU-cache](#lru-cache).

A *named* cache reference is a non-empty fixed-length string, overriding the numeric references bit-pattern of `1111_xxxx`, using the 4 left-over bits as the length (`bin(xxxx)+1`) of the key; the key is then used to index into the [MAP-cache](#map-cache).

This yields the following parsing space:

| Bits | Bit Pattern |
|-----:|-------------|
|  `7` Bits | `0_xxxxxxx` |
| `14` Bits | `10_xxxxxx xxxxxxxx` |
| `21` Bits | `110_xxxxx xxxxxxxx xxxxxxxx` |
| `28` Bits | `1110_xxxx xxxxxxxx xxxxxxxx xxxxxxxx` |
| `(n+1)` Bytes | `1111_nnnn`, followed by a `n+1` bytes long string. |

---

## TODO

- Caching?
- References?
- Slices?
- Templates?
