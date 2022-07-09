// SPDX-License-Identifier: GPL-2.0
/*
 * IronJVM: JVM Implementation in Rust
 * Copyright (C) 2022 HTGAzureX1212.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

// Credit: code referenced from https://gitlab.com/frozo/noak/

use std::borrow::Borrow;
use std::borrow::Cow;
use std::borrow::ToOwned;
use std::char;
use std::fmt;
use std::fmt::Write;
use std::iter::FromIterator;
use std::mem::size_of;
use std::ops;
use std::ops::Deref;
use std::str;

use crate::jstr::error::JUtf8Error;

pub mod error;

fn validate_jstr(v: &[u8]) -> bool {
    const CONSTANTS_LARGE_ENOUGH: bool = size_of::<u64>() >= size_of::<usize>();

    macro_rules! is_block_non_ascii {
        ($block:expr) => {{
            let block = $block;
            let is_not_ascii = block & (0x80808080_80808080u64 as usize) != 0;
            // see https://jameshfisher.com/2017/01/24/bitwise-check-for-zero-byte/ for information on how this works
            let contains_zero =
                ((block.wrapping_sub(0x01010101_01010101u64 as usize)) & (!block) & (0x80808080_80808080u64 as usize))
                    != 0;
            is_not_ascii || contains_zero
        }};
    }

    let block_size = 2 * size_of::<usize>();
    let align_offset = v.as_ptr().align_offset(size_of::<usize>());

    let mut i = 0;
    while i < v.len() {
        let b1 = v[i];

        if b1 >= 0x80 {
            let width = if b1 & 0b1111_0000 == 0b1110_0000 {
                3
            } else if b1 & 0b1110_0000 == 0b1100_0000 {
                2
            } else {
                return false;
            };
            if v.len() < i + width {
                return false;
            }
            match width {
                2 => {
                    // two byte case: U+0000 and U+0080 to U+07FF
                    if v[i + 1] & 0b1100_0000 != 0b1000_0000 {
                        return false;
                    }
                    // overlong encodings which do not encode `0` are not allowed
                    if b1 & 0b0001_1110 == 0 && (b1 != 0b1100_0000 && v[i + 1] != 0b1000_0000) {
                        return false;
                    }
                    i += 2;
                }
                3 => {
                    // three byte case: U+0800 and above
                    if v[i + 1] & 0b1100_0000 != 0b1000_0000
                        || v[i + 2] & 0b1100_0000 != 0b1000_0000
                    {
                        return false;
                    }
                    // overlong encodings are not allowed
                    if b1 & 0b0000_1111 == 0 && v[i + 1] & 0b0010_0000 == 0 {
                        return false;
                    }
                    i += 3;
                }
                _ => return false,
            }
        } else {
            // ASCII case: U+0001 to 0+007F
            if !CONSTANTS_LARGE_ENOUGH
                || align_offset == usize::MAX
                || align_offset.wrapping_sub(i) % block_size != 0
            {
                // probably unaligned
                if b1 == 0 {
                    return false;
                }
                i += 1;
            } else {
                // aligned
                while i + block_size < v.len() {
                    // SAFETY:
                    // - v.as_ptr().add(i) was verified to be aligned at this point
                    // - the block is confirmed to not exceed the input slice
                    unsafe {
                        let ptr = v.as_ptr().add(i).cast::<usize>();
                        if is_block_non_ascii!(*ptr) || is_block_non_ascii!(*ptr.offset(1)) {
                            break;
                        }
                    }
                    i += block_size;
                }

                // skip the remaining ascii characters after the last block
                while i < v.len() && v[i] < 0x80 {
                    if v[i] == 0 {
                        return false;
                    }
                    i += 1;
                }
            }
        }
    }

    true
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct JStr {
    inner: [u8],
}

impl JStr {
    pub fn from_jutf8(v: &[u8]) -> Result<&Self, JUtf8Error> {
        if validate_jstr(v) {
            // SAFETY: This is safe because the byte slice is guaranteed to be valid.
            Ok(unsafe { JStr::from_jutf8_unchecked(v) })
        } else {
            Err(JUtf8Error)
        }
    }

    #[must_use]
    pub const unsafe fn from_jutf8_unchecked(v: &[u8]) -> &Self {
        // SAFETY: Relies on &JStr and &[u8] having the same layout
        std::mem::transmute(v)
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        str::from_utf8(&self.inner).ok()
    }

    #[inline]
    #[must_use]
    pub fn is_char_boundary(&self, index: usize) -> bool {
        if index == 0 || index == self.len() {
            true
        } else {
            match self.as_bytes().get(index) {
                None => false,
                Some(&b) => b & 0b1100_0000 != 0b1000_0000,
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn chars(&self) -> Chars<'_> {
        Chars { inner: &self.inner }
    }

    #[inline]
    #[must_use]
    pub fn chars_lossy(&self) -> CharsLossy<'_> {
        CharsLossy { inner: &self.inner }
    }

    #[inline]
    #[must_use]
    pub fn display(&self) -> Display<'_> {
        Display { inner: &self.inner }
    }
}

impl Default for &'static JStr {
    fn default() -> &'static JStr {
        // SAFETY: This is safe because an empty slice is always valid.
        unsafe { JStr::from_jutf8_unchecked(&[]) }
    }
}

impl ops::Index<ops::RangeFull> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, _: ops::RangeFull) -> &JStr {
        self
    }
}

impl ops::Index<ops::Range<usize>> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &JStr {
        if index.start <= index.end
            && self.is_char_boundary(index.start)
            && self.is_char_boundary(index.end)
        {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            unsafe { JStr::from_jutf8_unchecked(self.inner.get_unchecked(index)) }
        } else {
            panic!("MUtf8 index out of bounds");
        }
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, index: ops::RangeInclusive<usize>) -> &JStr {
        if *index.end() == usize::MAX {
            panic!("cannot index mutf8 to maximum integer")
        } else {
            #[allow(clippy::range_plus_one)]
            &self[*index.start()..*index.end() + 1]
        }
    }
}

impl ops::Index<ops::RangeTo<usize>> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &JStr {
        if self.is_char_boundary(index.end) {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            unsafe { JStr::from_jutf8_unchecked(self.inner.get_unchecked(index)) }
        } else {
            panic!("index out of bounds");
        }
    }
}

impl ops::Index<ops::RangeToInclusive<usize>> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, index: ops::RangeToInclusive<usize>) -> &JStr {
        if index.end == usize::MAX {
            panic!("cannot index to maximum integer")
        } else {
            #[allow(clippy::range_plus_one)]
            &self[..index.end + 1]
        }
    }
}

impl ops::Index<ops::RangeFrom<usize>> for JStr {
    type Output = JStr;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &JStr {
        if self.is_char_boundary(index.start) {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            unsafe { JStr::from_jutf8_unchecked(self.inner.get_unchecked(index)) }
        } else {
            panic!("index out of bounds");
        }
    }
}

impl PartialEq<JString> for JStr {
    #[inline]
    fn eq(&self, other: &JString) -> bool {
        *self == **other
    }
}

impl PartialEq<JStr> for JString {
    #[inline]
    fn eq(&self, other: &JStr) -> bool {
        **self == *other
    }
}

impl PartialEq<str> for JStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        let mut left = self.chars();
        let mut right = other.chars();
        loop {
            match (left.next(), right.next()) {
                (Some(Ok(l)), Some(r)) if l == r => {}
                (None, None) => return true,
                (_, _) => return false,
            }
        }
    }
}

impl PartialEq<JStr> for str {
    #[inline]
    fn eq(&self, other: &JStr) -> bool {
        *other == *self
    }
}

impl PartialEq<&'_ str> for JStr {
    #[inline]
    fn eq(&self, other: &&'_ str) -> bool {
        *self == **other
    }
}

impl PartialEq<JStr> for &'_ str {
    #[inline]
    fn eq(&self, other: &JStr) -> bool {
        *other == **self
    }
}

impl PartialEq<Cow<'_, JStr>> for JStr {
    #[inline]
    fn eq(&self, other: &Cow<'_, JStr>) -> bool {
        *self == **other
    }
}

impl PartialEq<JStr> for Cow<'_, JStr> {
    #[inline]
    fn eq(&self, other: &JStr) -> bool {
        **self == *other
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JString {
    buf: Vec<u8>,
}

impl JString {
    #[inline]
    #[must_use]
    pub fn new() -> JString {
        JString { buf: Vec::new() }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(cap: usize) -> JString {
        JString {
            buf: Vec::with_capacity(cap),
        }
    }

    /// Creates a new string from a modified UTF-8 byte vector.
    pub fn from_mutf8(buf: Vec<u8>) -> Result<JString, JUtf8Error> {
        if validate_jstr(&buf) {
            Ok(JString { buf })
        } else {
            Err(JUtf8Error)
        }
    }

    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 6];
        let size = encode_jutf8_char(ch, &mut buf);
        self.buf.extend_from_slice(&buf[..size]);
    }
}

impl Deref for JString {
    type Target = JStr;

    #[inline]
    fn deref(&self) -> &JStr {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        unsafe { JStr::from_jutf8_unchecked(&self.buf) }
    }
}

impl Borrow<JStr> for JString {
    #[inline]
    fn borrow(&self) -> &JStr {
        &**self
    }
}

impl AsRef<[u8]> for JString {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<JStr> for JString {
    fn as_ref(&self) -> &JStr {
        &**self
    }
}

impl ToOwned for JStr {
    type Owned = JString;

    #[inline]
    fn to_owned(&self) -> JString {
        JString {
            buf: self.inner.to_owned(),
        }
    }
}

impl fmt::Debug for JStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        for c in self.chars() {
            match c {
                Ok(c) => {
                    write!(f, "{}", c.escape_debug())?;
                }
                Err(n) => {
                    // Unpaired surrogates are written as `\s{..}`.
                    write!(f, "\\s{{{n:x}}}")?;
                }
            }
        }
        f.write_char('"')
    }
}

impl fmt::Debug for JString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&**self).fmt(f)
    }
}

pub struct Display<'a> {
    inner: &'a [u8],
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start = 0;
        let mut i = 0;
        while i < self.inner.len() {
            if self.inner[i] != 0b1110_1101 {
                // Three byte long
                i += 1 + i.leading_ones() as usize;
            } else {
                if i != start {
                    // SAFETY: This is safe because everything from start to i are non-zero ascii bytes.
                    f.write_str(unsafe { str::from_utf8_unchecked(&self.inner[start..i]) })?;
                }

                // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
                let (size, ch) = unsafe { decode_jutf8_char(&self.inner[i..]) };
                i += size;

                start = i;
                f.write_char(ch.unwrap_or(char::REPLACEMENT_CHARACTER))?;
            }
        }

        if i != start {
            // SAFETY: This is safe because everything from start to i are non-zero ascii bytes.
            f.write_str(unsafe { str::from_utf8_unchecked(&self.inner[start..i]) })?;
        }

        Ok(())
    }
}

impl<'a> fmt::Debug for Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        unsafe { JStr::from_jutf8_unchecked(self.inner) }.fmt(f)
    }
}

pub struct Chars<'a> {
    inner: &'a [u8],
}

impl<'a> Chars<'a> {
    #[must_use]
    pub fn as_jstr(&self) -> &'a JStr {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        unsafe { JStr::from_jutf8_unchecked(self.inner) }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = Result<char, u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            let (size, ch) = unsafe { decode_jutf8_char(self.inner) };
            self.inner = &self.inner[size..];
            Some(ch)
        }
    }
}

impl<'a> DoubleEndedIterator for Chars<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            let (size, ch) = unsafe { decode_jutf8_char_reversed(self.inner) };
            self.inner = &self.inner[..self.inner.len() - size];
            Some(ch)
        }
    }
}

impl<'a> fmt::Debug for Chars<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        let s = unsafe { JStr::from_jutf8_unchecked(self.inner) };
        f.debug_struct("Chars").field("remaining", &s).finish()
    }
}

pub struct CharsLossy<'a> {
    inner: &'a [u8],
}

impl<'a> CharsLossy<'a> {
    #[must_use]
    pub fn as_jstr(&self) -> &'a JStr {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        unsafe { JStr::from_jutf8_unchecked(self.inner) }
    }
}

impl<'a> Iterator for CharsLossy<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            let (size, ch) = unsafe { decode_jutf8_char(self.inner) };
            self.inner = &self.inner[size..];
            Some(ch.unwrap_or(char::REPLACEMENT_CHARACTER))
        }
    }
}

impl<'a> DoubleEndedIterator for CharsLossy<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
            let (size, ch) = unsafe { decode_jutf8_char_reversed(self.inner) };
            self.inner = &self.inner[..self.inner.len() - size];
            Some(ch.unwrap_or(char::REPLACEMENT_CHARACTER))
        }
    }
}

impl<'a> fmt::Debug for CharsLossy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: This is safe because the underlying buffer is guaranteed to be valid.
        let s = unsafe { JStr::from_jutf8_unchecked(self.inner) };
        f.debug_struct("CharsLossy").field("remaining", &s).finish()
    }
}

unsafe fn decode_jutf8_char(v: &[u8]) -> (usize, Result<char, u32>) {
    if v[0] & 0b1000_0000 == 0b0000_0000 {
        // single byte case
        return (1, Ok(v[0] as char));
    }

    if v[0] & 0b1110_0000 == 0b1100_0000 {
        // two byte case
        let c1 = u32::from(v[0] & 0b0001_1111) << 6;
        let c2 = u32::from(v[1] & 0b0011_1111);
        return (2, Ok(char::from_u32_unchecked(c1 | c2)));
    }

    if v[0] == 0b1110_1101 {
        if v.len() >= 6
            && v[1] & 0b1111_0000 == 0b1010_0000
            && v[3] == 0b1110_1101
            && v[4] & 0b1111_0000 == 0b1011_0000
        {
            // six byte case (paired surrogate)
            let c2 = u32::from(v[1] & 0b0000_1111) << 16;
            let c3 = u32::from(v[2] & 0b0011_1111) << 10;
            let c5 = u32::from(v[4] & 0b0000_1111) << 6;
            let c6 = u32::from(v[5] & 0b0011_1111);
            return (
                6,
                Ok(char::from_u32_unchecked(0x10000 + (c2 | c3 | c5 | c6))),
            );
        }

        // unpaired surrogates
        if v[1] & 0b1110_0000 == 0b1010_0000 {
            let c2 = u32::from(v[1] & 0b0011_1111) << 6;
            let c3 = u32::from(v[2] & 0b0011_1111);
            return (3, Err(0b1101_0000_0000_0000 | c2 | c3));
        }
    }

    // three byte case
    let c1 = u32::from(v[0] & 0b0000_1111) << 12;
    let c2 = u32::from(v[1] & 0b0011_1111) << 6;
    let c3 = u32::from(v[2] & 0b0011_1111);
    (3, Ok(char::from_u32_unchecked(c1 | c2 | c3)))
}

unsafe fn decode_jutf8_char_reversed(v: &[u8]) -> (usize, Result<char, u32>) {
    let b1 = v[v.len() - 1];
    if b1 & 0b1000_0000 == 0b0000_0000 {
        // single byte case
        return (1, Ok(b1 as char));
    }

    let b2 = v[v.len() - 2];
    if b2 & 0b1110_0000 == 0b1100_0000 {
        // two byte case
        let c1 = u32::from(b2 & 0b0001_1111) << 6;
        let c2 = u32::from(b1 & 0b0011_1111);
        return (2, Ok(char::from_u32_unchecked(c1 | c2)));
    }

    let b3 = v[v.len() - 3];
    if b3 == 0b1110_1101 {
        if v.len() >= 6 {
            let b4 = v[v.len() - 4];
            let b5 = v[v.len() - 5];
            let b6 = v[v.len() - 6];
            if b2 & 0b1111_0000 == 0b1011_0000
                && b5 & 0b1111_0000 == 0b1010_0000
                && b6 == 0b1110_1101
            {
                // six byte case
                let c2 = u32::from(b5 & 0b0000_1111) << 16;
                let c3 = u32::from(b4 & 0b0011_1111) << 10;
                let c5 = u32::from(b2 & 0b0000_1111) << 6;
                let c6 = u32::from(b1 & 0b0011_1111);
                return (
                    6,
                    Ok(char::from_u32_unchecked(0x10000 + (c2 | c3 | c5 | c6))),
                );
            }
        }
        // unpaired surrogates
        if b2 & 0b1110_0000 == 0b1010_0000 {
            let c2 = u32::from(b2 & 0b0011_1111) << 6;
            let c3 = u32::from(b1 & 0b0011_1111);
            return (3, Err(0b1101_0000_0000_0000 | c2 | c3));
        }
    }

    // three byte case
    let c1 = u32::from(b3 & 0b0000_1111) << 12;
    let c2 = u32::from(b2 & 0b0011_1111) << 6;
    let c3 = u32::from(b1 & 0b0011_1111);
    (3, Ok(char::from_u32_unchecked(c1 | c2 | c3)))
}

impl From<&JStr> for JString {
    fn from(s: &JStr) -> JString {
        s.to_owned()
    }
}

impl From<&str> for JString {
    fn from(s: &str) -> JString {
        let mut buf = JString::with_capacity(s.len());
        buf.extend(s.chars());
        buf
    }
}

impl FromIterator<char> for JString {
    fn from_iter<I>(iter: I) -> JString
    where
        I: IntoIterator<Item = char>,
    {
        let mut buf = JString::new();
        buf.extend(iter);
        buf
    }
}

impl Extend<char> for JString {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();
        self.buf.reserve(lower_bound);
        for ch in iter {
            self.push(ch);
        }
    }
}

impl<'a> Extend<&'a str> for JString {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        self.extend(iter.into_iter().flat_map(str::chars));
    }
}

fn encode_jutf8_char(ch: char, buf: &mut [u8]) -> usize {
    let ch = ch as u32;
    match ch {
        0x01..=0x7F => {
            buf[0] = ch as u8;
            1
        }
        0 | 0x80..=0x7FF => {
            buf[0] = (0b1100_0000 | (ch >> 6)) as u8;
            buf[1] = (0b1000_0000 | (ch & 0b0011_1111)) as u8;
            2
        }
        0x800..=0xFFFF => {
            buf[0] = (0b1110_0000 | (ch >> 12)) as u8;
            buf[1] = (0b1000_0000 | ((ch >> 6) & 0b0011_1111)) as u8;
            buf[2] = (0b1000_0000 | (ch & 0b0011_1111)) as u8;
            3
        }
        _ => {
            let ch = ch - 0x10000;
            buf[0] = 0b1110_1101;
            buf[1] = (0b1010_0000 | ((ch >> 16) & 0b0000_1111)) as u8;
            buf[2] = (0b1000_0000 | ((ch >> 10) & 0b0011_1111)) as u8;
            buf[3] = 0b1110_1101;
            buf[4] = (0b1011_0000 | ((ch >> 6) & 0b0000_1111)) as u8;
            buf[5] = (0b1000_0000 | (ch & 0b0011_1111)) as u8;
            6
        }
    }
}
