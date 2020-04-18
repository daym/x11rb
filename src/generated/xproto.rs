// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::Event as _;
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::cookie::ListFontsWithInfoCookie;
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Char2b {
    pub byte1: u8,
    pub byte2: u8,
}
impl TryParse for Char2b {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (byte1, remaining) = u8::try_parse(remaining)?;
        let (byte2, remaining) = u8::try_parse(remaining)?;
        let result = Char2b { byte1, byte2 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Char2b {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Char2b {
    type Bytes = [u8; 2];
    fn serialize(&self) -> Self::Bytes {
        let byte1_bytes = self.byte1.serialize();
        let byte2_bytes = self.byte2.serialize();
        [
            byte1_bytes[0],
            byte2_bytes[0],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(2);
        self.byte1.serialize_into(bytes);
        self.byte2.serialize_into(bytes);
    }
}

pub type Window = u32;

pub type Pixmap = u32;

pub type Cursor = u32;

pub type Font = u32;

pub type Gcontext = u32;

pub type Colormap = u32;

pub type Atom = u32;

pub type Drawable = u32;

pub type Fontable = u32;

pub type Bool32 = u32;

pub type Visualid = u32;

pub type Timestamp = u32;

pub type Keysym = u32;

pub type Keycode = u8;

pub type Keycode32 = u32;

pub type Button = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
impl TryParse for Point {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let result = Point { x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Point {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Point {
    type Bytes = [u8; 4];
    fn serialize(&self) -> Self::Bytes {
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
impl TryParse for Rectangle {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let result = Rectangle { x, y, width, height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Rectangle {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Rectangle {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arc {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}
impl TryParse for Arc {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (angle1, remaining) = i16::try_parse(remaining)?;
        let (angle2, remaining) = i16::try_parse(remaining)?;
        let result = Arc { x, y, width, height, angle1, angle2 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Arc {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Arc {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let angle1_bytes = self.angle1.serialize();
        let angle2_bytes = self.angle2.serialize();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            angle1_bytes[0],
            angle1_bytes[1],
            angle2_bytes[0],
            angle2_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
        self.width.serialize_into(bytes);
        self.height.serialize_into(bytes);
        self.angle1.serialize_into(bytes);
        self.angle2.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}
impl TryParse for Format {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bits_per_pixel, remaining) = u8::try_parse(remaining)?;
        let (scanline_pad, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let result = Format { depth, bits_per_pixel, scanline_pad };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Format {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Format {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let depth_bytes = self.depth.serialize();
        let bits_per_pixel_bytes = self.bits_per_pixel.serialize();
        let scanline_pad_bytes = self.scanline_pad.serialize();
        [
            depth_bytes[0],
            bits_per_pixel_bytes[0],
            scanline_pad_bytes[0],
            0,
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.depth.serialize_into(bytes);
        self.bits_per_pixel.serialize_into(bytes);
        self.scanline_pad.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VisualClass {
    StaticGray = 0,
    GrayScale = 1,
    StaticColor = 2,
    PseudoColor = 3,
    TrueColor = 4,
    DirectColor = 5,
}
impl From<VisualClass> for u8 {
    fn from(input: VisualClass) -> Self {
        match input {
            VisualClass::StaticGray => 0,
            VisualClass::GrayScale => 1,
            VisualClass::StaticColor => 2,
            VisualClass::PseudoColor => 3,
            VisualClass::TrueColor => 4,
            VisualClass::DirectColor => 5,
        }
    }
}
impl From<VisualClass> for Option<u8> {
    fn from(input: VisualClass) -> Self {
        Some(u8::from(input))
    }
}
impl From<VisualClass> for u16 {
    fn from(input: VisualClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VisualClass> for Option<u16> {
    fn from(input: VisualClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<VisualClass> for u32 {
    fn from(input: VisualClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VisualClass> for Option<u32> {
    fn from(input: VisualClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for VisualClass {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VisualClass::StaticGray),
            1 => Ok(VisualClass::GrayScale),
            2 => Ok(VisualClass::StaticColor),
            3 => Ok(VisualClass::PseudoColor),
            4 => Ok(VisualClass::TrueColor),
            5 => Ok(VisualClass::DirectColor),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for VisualClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for VisualClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Visualtype {
    pub visual_id: Visualid,
    pub class: VisualClass,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}
impl TryParse for Visualtype {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (visual_id, remaining) = Visualid::try_parse(remaining)?;
        let (class, remaining) = u8::try_parse(remaining)?;
        let (bits_per_rgb_value, remaining) = u8::try_parse(remaining)?;
        let (colormap_entries, remaining) = u16::try_parse(remaining)?;
        let (red_mask, remaining) = u32::try_parse(remaining)?;
        let (green_mask, remaining) = u32::try_parse(remaining)?;
        let (blue_mask, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let class = class.try_into()?;
        let result = Visualtype { visual_id, class, bits_per_rgb_value, colormap_entries, red_mask, green_mask, blue_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Visualtype {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Visualtype {
    type Bytes = [u8; 24];
    fn serialize(&self) -> Self::Bytes {
        let visual_id_bytes = self.visual_id.serialize();
        let class_bytes = u8::from(self.class).serialize();
        let bits_per_rgb_value_bytes = self.bits_per_rgb_value.serialize();
        let colormap_entries_bytes = self.colormap_entries.serialize();
        let red_mask_bytes = self.red_mask.serialize();
        let green_mask_bytes = self.green_mask.serialize();
        let blue_mask_bytes = self.blue_mask.serialize();
        [
            visual_id_bytes[0],
            visual_id_bytes[1],
            visual_id_bytes[2],
            visual_id_bytes[3],
            class_bytes[0],
            bits_per_rgb_value_bytes[0],
            colormap_entries_bytes[0],
            colormap_entries_bytes[1],
            red_mask_bytes[0],
            red_mask_bytes[1],
            red_mask_bytes[2],
            red_mask_bytes[3],
            green_mask_bytes[0],
            green_mask_bytes[1],
            green_mask_bytes[2],
            green_mask_bytes[3],
            blue_mask_bytes[0],
            blue_mask_bytes[1],
            blue_mask_bytes[2],
            blue_mask_bytes[3],
            0,
            0,
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(24);
        self.visual_id.serialize_into(bytes);
        u8::from(self.class).serialize_into(bytes);
        self.bits_per_rgb_value.serialize_into(bytes);
        self.colormap_entries.serialize_into(bytes);
        self.red_mask.serialize_into(bytes);
        self.green_mask.serialize_into(bytes);
        self.blue_mask.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<Visualtype>,
}
impl TryParse for Depth {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (visuals_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (visuals, remaining) = crate::x11_utils::parse_list::<Visualtype>(remaining, visuals_len as usize)?;
        let result = Depth { depth, visuals };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Depth {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Depth {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        let visuals_len = self.visuals.len() as u16;
        visuals_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.visuals.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EventMask {
    NoEvent = 0,
    KeyPress = 1 << 0,
    KeyRelease = 1 << 1,
    ButtonPress = 1 << 2,
    ButtonRelease = 1 << 3,
    EnterWindow = 1 << 4,
    LeaveWindow = 1 << 5,
    PointerMotion = 1 << 6,
    PointerMotionHint = 1 << 7,
    Button1Motion = 1 << 8,
    Button2Motion = 1 << 9,
    Button3Motion = 1 << 10,
    Button4Motion = 1 << 11,
    Button5Motion = 1 << 12,
    ButtonMotion = 1 << 13,
    KeymapState = 1 << 14,
    Exposure = 1 << 15,
    VisibilityChange = 1 << 16,
    StructureNotify = 1 << 17,
    ResizeRedirect = 1 << 18,
    SubstructureNotify = 1 << 19,
    SubstructureRedirect = 1 << 20,
    FocusChange = 1 << 21,
    PropertyChange = 1 << 22,
    ColorMapChange = 1 << 23,
    OwnerGrabButton = 1 << 24,
}
impl From<EventMask> for u32 {
    fn from(input: EventMask) -> Self {
        match input {
            EventMask::NoEvent => 0,
            EventMask::KeyPress => 1 << 0,
            EventMask::KeyRelease => 1 << 1,
            EventMask::ButtonPress => 1 << 2,
            EventMask::ButtonRelease => 1 << 3,
            EventMask::EnterWindow => 1 << 4,
            EventMask::LeaveWindow => 1 << 5,
            EventMask::PointerMotion => 1 << 6,
            EventMask::PointerMotionHint => 1 << 7,
            EventMask::Button1Motion => 1 << 8,
            EventMask::Button2Motion => 1 << 9,
            EventMask::Button3Motion => 1 << 10,
            EventMask::Button4Motion => 1 << 11,
            EventMask::Button5Motion => 1 << 12,
            EventMask::ButtonMotion => 1 << 13,
            EventMask::KeymapState => 1 << 14,
            EventMask::Exposure => 1 << 15,
            EventMask::VisibilityChange => 1 << 16,
            EventMask::StructureNotify => 1 << 17,
            EventMask::ResizeRedirect => 1 << 18,
            EventMask::SubstructureNotify => 1 << 19,
            EventMask::SubstructureRedirect => 1 << 20,
            EventMask::FocusChange => 1 << 21,
            EventMask::PropertyChange => 1 << 22,
            EventMask::ColorMapChange => 1 << 23,
            EventMask::OwnerGrabButton => 1 << 24,
        }
    }
}
impl From<EventMask> for Option<u32> {
    fn from(input: EventMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for EventMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventMask::NoEvent),
            1 => Ok(EventMask::KeyPress),
            2 => Ok(EventMask::KeyRelease),
            4 => Ok(EventMask::ButtonPress),
            8 => Ok(EventMask::ButtonRelease),
            16 => Ok(EventMask::EnterWindow),
            32 => Ok(EventMask::LeaveWindow),
            64 => Ok(EventMask::PointerMotion),
            128 => Ok(EventMask::PointerMotionHint),
            256 => Ok(EventMask::Button1Motion),
            512 => Ok(EventMask::Button2Motion),
            1024 => Ok(EventMask::Button3Motion),
            2048 => Ok(EventMask::Button4Motion),
            4096 => Ok(EventMask::Button5Motion),
            8192 => Ok(EventMask::ButtonMotion),
            16384 => Ok(EventMask::KeymapState),
            32768 => Ok(EventMask::Exposure),
            65536 => Ok(EventMask::VisibilityChange),
            131_072 => Ok(EventMask::StructureNotify),
            262_144 => Ok(EventMask::ResizeRedirect),
            524_288 => Ok(EventMask::SubstructureNotify),
            1_048_576 => Ok(EventMask::SubstructureRedirect),
            2_097_152 => Ok(EventMask::FocusChange),
            4_194_304 => Ok(EventMask::PropertyChange),
            8_388_608 => Ok(EventMask::ColorMapChange),
            16_777_216 => Ok(EventMask::OwnerGrabButton),
            _ => Err(ParseError::ParseError),
        }
    }
}
bitmask_binop!(EventMask, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BackingStore {
    NotUseful = 0,
    WhenMapped = 1,
    Always = 2,
}
impl From<BackingStore> for u8 {
    fn from(input: BackingStore) -> Self {
        match input {
            BackingStore::NotUseful => 0,
            BackingStore::WhenMapped => 1,
            BackingStore::Always => 2,
        }
    }
}
impl From<BackingStore> for Option<u8> {
    fn from(input: BackingStore) -> Self {
        Some(u8::from(input))
    }
}
impl From<BackingStore> for u16 {
    fn from(input: BackingStore) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackingStore> for Option<u16> {
    fn from(input: BackingStore) -> Self {
        Some(u16::from(input))
    }
}
impl From<BackingStore> for u32 {
    fn from(input: BackingStore) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackingStore> for Option<u32> {
    fn from(input: BackingStore) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BackingStore::NotUseful),
            1 => Ok(BackingStore::WhenMapped),
            2 => Ok(BackingStore::Always),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub root: Window,
    pub default_colormap: Colormap,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: Visualid,
    pub backing_stores: BackingStore,
    pub save_unders: bool,
    pub root_depth: u8,
    pub allowed_depths: Vec<Depth>,
}
impl TryParse for Screen {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (root, remaining) = Window::try_parse(remaining)?;
        let (default_colormap, remaining) = Colormap::try_parse(remaining)?;
        let (white_pixel, remaining) = u32::try_parse(remaining)?;
        let (black_pixel, remaining) = u32::try_parse(remaining)?;
        let (current_input_masks, remaining) = u32::try_parse(remaining)?;
        let (width_in_pixels, remaining) = u16::try_parse(remaining)?;
        let (height_in_pixels, remaining) = u16::try_parse(remaining)?;
        let (width_in_millimeters, remaining) = u16::try_parse(remaining)?;
        let (height_in_millimeters, remaining) = u16::try_parse(remaining)?;
        let (min_installed_maps, remaining) = u16::try_parse(remaining)?;
        let (max_installed_maps, remaining) = u16::try_parse(remaining)?;
        let (root_visual, remaining) = Visualid::try_parse(remaining)?;
        let (backing_stores, remaining) = u8::try_parse(remaining)?;
        let (save_unders, remaining) = bool::try_parse(remaining)?;
        let (root_depth, remaining) = u8::try_parse(remaining)?;
        let (allowed_depths_len, remaining) = u8::try_parse(remaining)?;
        let (allowed_depths, remaining) = crate::x11_utils::parse_list::<Depth>(remaining, allowed_depths_len as usize)?;
        let backing_stores = backing_stores.try_into()?;
        let result = Screen { root, default_colormap, white_pixel, black_pixel, current_input_masks, width_in_pixels, height_in_pixels, width_in_millimeters, height_in_millimeters, min_installed_maps, max_installed_maps, root_visual, backing_stores, save_unders, root_depth, allowed_depths };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Screen {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Screen {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(40);
        self.root.serialize_into(bytes);
        self.default_colormap.serialize_into(bytes);
        self.white_pixel.serialize_into(bytes);
        self.black_pixel.serialize_into(bytes);
        self.current_input_masks.serialize_into(bytes);
        self.width_in_pixels.serialize_into(bytes);
        self.height_in_pixels.serialize_into(bytes);
        self.width_in_millimeters.serialize_into(bytes);
        self.height_in_millimeters.serialize_into(bytes);
        self.min_installed_maps.serialize_into(bytes);
        self.max_installed_maps.serialize_into(bytes);
        self.root_visual.serialize_into(bytes);
        u8::from(self.backing_stores).serialize_into(bytes);
        self.save_unders.serialize_into(bytes);
        self.root_depth.serialize_into(bytes);
        let allowed_depths_len = self.allowed_depths.len() as u8;
        allowed_depths_len.serialize_into(bytes);
        self.allowed_depths.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupRequest {
    pub byte_order: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name: Vec<u8>,
    pub authorization_protocol_data: Vec<u8>,
}
impl TryParse for SetupRequest {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (byte_order, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (protocol_major_version, remaining) = u16::try_parse(remaining)?;
        let (protocol_minor_version, remaining) = u16::try_parse(remaining)?;
        let (authorization_protocol_name_len, remaining) = u16::try_parse(remaining)?;
        let (authorization_protocol_data_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (authorization_protocol_name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, authorization_protocol_name_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (authorization_protocol_data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, authorization_protocol_data_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = SetupRequest { byte_order, protocol_major_version, protocol_minor_version, authorization_protocol_name, authorization_protocol_data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetupRequest {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetupRequest {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.byte_order.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.protocol_major_version.serialize_into(bytes);
        self.protocol_minor_version.serialize_into(bytes);
        let authorization_protocol_name_len = self.authorization_protocol_name.len() as u16;
        authorization_protocol_name_len.serialize_into(bytes);
        let authorization_protocol_data_len = self.authorization_protocol_data.len() as u16;
        authorization_protocol_data_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
        self.authorization_protocol_name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.authorization_protocol_data.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupFailed {
    pub status: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub reason: Vec<u8>,
}
impl TryParse for SetupFailed {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (status, remaining) = u8::try_parse(remaining)?;
        let (reason_len, remaining) = u8::try_parse(remaining)?;
        let (protocol_major_version, remaining) = u16::try_parse(remaining)?;
        let (protocol_minor_version, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u16::try_parse(remaining)?;
        let (reason, remaining) = crate::x11_utils::parse_list::<u8>(remaining, reason_len as usize)?;
        let result = SetupFailed { status, protocol_major_version, protocol_minor_version, length, reason };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetupFailed {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetupFailed {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.status.serialize_into(bytes);
        let reason_len = self.reason.len() as u8;
        reason_len.serialize_into(bytes);
        self.protocol_major_version.serialize_into(bytes);
        self.protocol_minor_version.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.reason.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupAuthenticate {
    pub status: u8,
    pub reason: Vec<u8>,
}
impl TryParse for SetupAuthenticate {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(5..).ok_or(ParseError::ParseError)?;
        let (length, remaining) = u16::try_parse(remaining)?;
        let (reason, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = SetupAuthenticate { status, reason };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetupAuthenticate {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for SetupAuthenticate {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 5]);
        let length = self.reason.len() as u16 / 4;
        length.serialize_into(bytes);
        self.reason.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageOrder {
    LSBFirst = 0,
    MSBFirst = 1,
}
impl From<ImageOrder> for u8 {
    fn from(input: ImageOrder) -> Self {
        match input {
            ImageOrder::LSBFirst => 0,
            ImageOrder::MSBFirst => 1,
        }
    }
}
impl From<ImageOrder> for Option<u8> {
    fn from(input: ImageOrder) -> Self {
        Some(u8::from(input))
    }
}
impl From<ImageOrder> for u16 {
    fn from(input: ImageOrder) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageOrder> for Option<u16> {
    fn from(input: ImageOrder) -> Self {
        Some(u16::from(input))
    }
}
impl From<ImageOrder> for u32 {
    fn from(input: ImageOrder) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageOrder> for Option<u32> {
    fn from(input: ImageOrder) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ImageOrder {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageOrder::LSBFirst),
            1 => Ok(ImageOrder::MSBFirst),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ImageOrder {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ImageOrder {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setup {
    pub status: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: ImageOrder,
    pub bitmap_format_bit_order: ImageOrder,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: Keycode,
    pub max_keycode: Keycode,
    pub vendor: Vec<u8>,
    pub pixmap_formats: Vec<Format>,
    pub roots: Vec<Screen>,
}
impl TryParse for Setup {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (status, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (protocol_major_version, remaining) = u16::try_parse(remaining)?;
        let (protocol_minor_version, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u16::try_parse(remaining)?;
        let (release_number, remaining) = u32::try_parse(remaining)?;
        let (resource_id_base, remaining) = u32::try_parse(remaining)?;
        let (resource_id_mask, remaining) = u32::try_parse(remaining)?;
        let (motion_buffer_size, remaining) = u32::try_parse(remaining)?;
        let (vendor_len, remaining) = u16::try_parse(remaining)?;
        let (maximum_request_length, remaining) = u16::try_parse(remaining)?;
        let (roots_len, remaining) = u8::try_parse(remaining)?;
        let (pixmap_formats_len, remaining) = u8::try_parse(remaining)?;
        let (image_byte_order, remaining) = u8::try_parse(remaining)?;
        let (bitmap_format_bit_order, remaining) = u8::try_parse(remaining)?;
        let (bitmap_format_scanline_unit, remaining) = u8::try_parse(remaining)?;
        let (bitmap_format_scanline_pad, remaining) = u8::try_parse(remaining)?;
        let (min_keycode, remaining) = Keycode::try_parse(remaining)?;
        let (max_keycode, remaining) = Keycode::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (vendor, remaining) = crate::x11_utils::parse_list::<u8>(remaining, vendor_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (pixmap_formats, remaining) = crate::x11_utils::parse_list::<Format>(remaining, pixmap_formats_len as usize)?;
        let (roots, remaining) = crate::x11_utils::parse_list::<Screen>(remaining, roots_len as usize)?;
        let image_byte_order = image_byte_order.try_into()?;
        let bitmap_format_bit_order = bitmap_format_bit_order.try_into()?;
        let result = Setup { status, protocol_major_version, protocol_minor_version, length, release_number, resource_id_base, resource_id_mask, motion_buffer_size, maximum_request_length, image_byte_order, bitmap_format_bit_order, bitmap_format_scanline_unit, bitmap_format_scanline_pad, min_keycode, max_keycode, vendor, pixmap_formats, roots };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Setup {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Setup {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(40);
        self.status.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        self.protocol_major_version.serialize_into(bytes);
        self.protocol_minor_version.serialize_into(bytes);
        self.length.serialize_into(bytes);
        self.release_number.serialize_into(bytes);
        self.resource_id_base.serialize_into(bytes);
        self.resource_id_mask.serialize_into(bytes);
        self.motion_buffer_size.serialize_into(bytes);
        let vendor_len = self.vendor.len() as u16;
        vendor_len.serialize_into(bytes);
        self.maximum_request_length.serialize_into(bytes);
        let roots_len = self.roots.len() as u8;
        roots_len.serialize_into(bytes);
        let pixmap_formats_len = self.pixmap_formats.len() as u8;
        pixmap_formats_len.serialize_into(bytes);
        u8::from(self.image_byte_order).serialize_into(bytes);
        u8::from(self.bitmap_format_bit_order).serialize_into(bytes);
        self.bitmap_format_scanline_unit.serialize_into(bytes);
        self.bitmap_format_scanline_pad.serialize_into(bytes);
        self.min_keycode.serialize_into(bytes);
        self.max_keycode.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.vendor.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        self.pixmap_formats.serialize_into(bytes);
        self.roots.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ModMask {
    Shift = 1 << 0,
    Lock = 1 << 1,
    Control = 1 << 2,
    M1 = 1 << 3,
    M2 = 1 << 4,
    M3 = 1 << 5,
    M4 = 1 << 6,
    M5 = 1 << 7,
    Any = 1 << 15,
}
impl From<ModMask> for u16 {
    fn from(input: ModMask) -> Self {
        match input {
            ModMask::Shift => 1 << 0,
            ModMask::Lock => 1 << 1,
            ModMask::Control => 1 << 2,
            ModMask::M1 => 1 << 3,
            ModMask::M2 => 1 << 4,
            ModMask::M3 => 1 << 5,
            ModMask::M4 => 1 << 6,
            ModMask::M5 => 1 << 7,
            ModMask::Any => 1 << 15,
        }
    }
}
impl From<ModMask> for Option<u16> {
    fn from(input: ModMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ModMask> for u32 {
    fn from(input: ModMask) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<ModMask> for Option<u32> {
    fn from(input: ModMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for ModMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ModMask::Shift),
            2 => Ok(ModMask::Lock),
            4 => Ok(ModMask::Control),
            8 => Ok(ModMask::M1),
            16 => Ok(ModMask::M2),
            32 => Ok(ModMask::M3),
            64 => Ok(ModMask::M4),
            128 => Ok(ModMask::M5),
            32768 => Ok(ModMask::Any),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for ModMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ModMask, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum KeyButMask {
    Shift = 1 << 0,
    Lock = 1 << 1,
    Control = 1 << 2,
    Mod1 = 1 << 3,
    Mod2 = 1 << 4,
    Mod3 = 1 << 5,
    Mod4 = 1 << 6,
    Mod5 = 1 << 7,
    Button1 = 1 << 8,
    Button2 = 1 << 9,
    Button3 = 1 << 10,
    Button4 = 1 << 11,
    Button5 = 1 << 12,
}
impl From<KeyButMask> for u16 {
    fn from(input: KeyButMask) -> Self {
        match input {
            KeyButMask::Shift => 1 << 0,
            KeyButMask::Lock => 1 << 1,
            KeyButMask::Control => 1 << 2,
            KeyButMask::Mod1 => 1 << 3,
            KeyButMask::Mod2 => 1 << 4,
            KeyButMask::Mod3 => 1 << 5,
            KeyButMask::Mod4 => 1 << 6,
            KeyButMask::Mod5 => 1 << 7,
            KeyButMask::Button1 => 1 << 8,
            KeyButMask::Button2 => 1 << 9,
            KeyButMask::Button3 => 1 << 10,
            KeyButMask::Button4 => 1 << 11,
            KeyButMask::Button5 => 1 << 12,
        }
    }
}
impl From<KeyButMask> for Option<u16> {
    fn from(input: KeyButMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<KeyButMask> for u32 {
    fn from(input: KeyButMask) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<KeyButMask> for Option<u32> {
    fn from(input: KeyButMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for KeyButMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KeyButMask::Shift),
            2 => Ok(KeyButMask::Lock),
            4 => Ok(KeyButMask::Control),
            8 => Ok(KeyButMask::Mod1),
            16 => Ok(KeyButMask::Mod2),
            32 => Ok(KeyButMask::Mod3),
            64 => Ok(KeyButMask::Mod4),
            128 => Ok(KeyButMask::Mod5),
            256 => Ok(KeyButMask::Button1),
            512 => Ok(KeyButMask::Button2),
            1024 => Ok(KeyButMask::Button3),
            2048 => Ok(KeyButMask::Button4),
            4096 => Ok(KeyButMask::Button5),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for KeyButMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(KeyButMask, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WindowEnum {
    None = 0,
}
impl From<WindowEnum> for u8 {
    fn from(input: WindowEnum) -> Self {
        match input {
            WindowEnum::None => 0,
        }
    }
}
impl From<WindowEnum> for Option<u8> {
    fn from(input: WindowEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<WindowEnum> for u16 {
    fn from(input: WindowEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<WindowEnum> for Option<u16> {
    fn from(input: WindowEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<WindowEnum> for u32 {
    fn from(input: WindowEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<WindowEnum> for Option<u32> {
    fn from(input: WindowEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for WindowEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WindowEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for WindowEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for WindowEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the KeyPress event
pub const KEY_PRESS_EVENT: u8 = 2;
/// a key was pressed/released.
///
///
/// # Fields
///
/// * `detail` - The keycode (a number representing a physical key on the keyboard) of the key
/// which was pressed.
/// * `event_x` - If `same_screen` is true, this is the X coordinate relative to the `event`
/// window's origin. Otherwise, `event_x` will be set to zero.
/// * `event_y` - If `same_screen` is true, this is the Y coordinate relative to the `event`
/// window's origin. Otherwise, `event_y` will be set to zero.
/// * `root` - The root window of `child`.
/// * `root_x` - The X coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `root_y` - The Y coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `same_screen` - Whether the `event` window is on the same screen as the `root` window.
/// * `state` - The logical state of the pointer buttons and modifier keys just prior to the
/// event.
/// * `time` - Time when the event was generated (in milliseconds).
///
/// # See
///
/// * GrabKey: request
/// * GrabKeyboard: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyPressEvent {
    pub response_type: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl KeyPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = Keycode::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = KeyPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&KeyPressEvent> for [u8; 32] {
    fn from(input: &KeyPressEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], 0
        ]
    }
}
impl From<KeyPressEvent> for [u8; 32] {
    fn from(input: KeyPressEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the KeyRelease event
pub const KEY_RELEASE_EVENT: u8 = 3;
/// a key was pressed/released.
///
///
/// # Fields
///
/// * `detail` - The keycode (a number representing a physical key on the keyboard) of the key
/// which was pressed.
/// * `event_x` - If `same_screen` is true, this is the X coordinate relative to the `event`
/// window's origin. Otherwise, `event_x` will be set to zero.
/// * `event_y` - If `same_screen` is true, this is the Y coordinate relative to the `event`
/// window's origin. Otherwise, `event_y` will be set to zero.
/// * `root` - The root window of `child`.
/// * `root_x` - The X coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `root_y` - The Y coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `same_screen` - Whether the `event` window is on the same screen as the `root` window.
/// * `state` - The logical state of the pointer buttons and modifier keys just prior to the
/// event.
/// * `time` - Time when the event was generated (in milliseconds).
///
/// # See
///
/// * GrabKey: request
/// * GrabKeyboard: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyReleaseEvent {
    pub response_type: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl KeyReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = Keycode::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = KeyReleaseEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for KeyReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&KeyReleaseEvent> for [u8; 32] {
    fn from(input: &KeyReleaseEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], 0
        ]
    }
}
impl From<KeyReleaseEvent> for [u8; 32] {
    fn from(input: KeyReleaseEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ButtonMask {
    M1 = 1 << 8,
    M2 = 1 << 9,
    M3 = 1 << 10,
    M4 = 1 << 11,
    M5 = 1 << 12,
    Any = 1 << 15,
}
impl From<ButtonMask> for u16 {
    fn from(input: ButtonMask) -> Self {
        match input {
            ButtonMask::M1 => 1 << 8,
            ButtonMask::M2 => 1 << 9,
            ButtonMask::M3 => 1 << 10,
            ButtonMask::M4 => 1 << 11,
            ButtonMask::M5 => 1 << 12,
            ButtonMask::Any => 1 << 15,
        }
    }
}
impl From<ButtonMask> for Option<u16> {
    fn from(input: ButtonMask) -> Self {
        Some(u16::from(input))
    }
}
impl From<ButtonMask> for u32 {
    fn from(input: ButtonMask) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<ButtonMask> for Option<u32> {
    fn from(input: ButtonMask) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for ButtonMask {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            256 => Ok(ButtonMask::M1),
            512 => Ok(ButtonMask::M2),
            1024 => Ok(ButtonMask::M3),
            2048 => Ok(ButtonMask::M4),
            4096 => Ok(ButtonMask::M5),
            32768 => Ok(ButtonMask::Any),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for ButtonMask {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ButtonMask, u16);

/// Opcode for the ButtonPress event
pub const BUTTON_PRESS_EVENT: u8 = 4;
/// a mouse button was pressed/released.
///
///
/// # Fields
///
/// * `detail` - The keycode (a number representing a physical key on the keyboard) of the key
/// which was pressed.
/// * `event_x` - If `same_screen` is true, this is the X coordinate relative to the `event`
/// window's origin. Otherwise, `event_x` will be set to zero.
/// * `event_y` - If `same_screen` is true, this is the Y coordinate relative to the `event`
/// window's origin. Otherwise, `event_y` will be set to zero.
/// * `root` - The root window of `child`.
/// * `root_x` - The X coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `root_y` - The Y coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `same_screen` - Whether the `event` window is on the same screen as the `root` window.
/// * `state` - The logical state of the pointer buttons and modifier keys just prior to the
/// event.
/// * `time` - Time when the event was generated (in milliseconds).
///
/// # See
///
/// * GrabButton: request
/// * GrabPointer: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonPressEvent {
    pub response_type: u8,
    pub detail: Button,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl ButtonPressEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = Button::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ButtonPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ButtonPressEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ButtonPressEvent> for [u8; 32] {
    fn from(input: &ButtonPressEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], 0
        ]
    }
}
impl From<ButtonPressEvent> for [u8; 32] {
    fn from(input: ButtonPressEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ButtonRelease event
pub const BUTTON_RELEASE_EVENT: u8 = 5;
/// a mouse button was pressed/released.
///
///
/// # Fields
///
/// * `detail` - The keycode (a number representing a physical key on the keyboard) of the key
/// which was pressed.
/// * `event_x` - If `same_screen` is true, this is the X coordinate relative to the `event`
/// window's origin. Otherwise, `event_x` will be set to zero.
/// * `event_y` - If `same_screen` is true, this is the Y coordinate relative to the `event`
/// window's origin. Otherwise, `event_y` will be set to zero.
/// * `root` - The root window of `child`.
/// * `root_x` - The X coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `root_y` - The Y coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `same_screen` - Whether the `event` window is on the same screen as the `root` window.
/// * `state` - The logical state of the pointer buttons and modifier keys just prior to the
/// event.
/// * `time` - Time when the event was generated (in milliseconds).
///
/// # See
///
/// * GrabButton: request
/// * GrabPointer: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonReleaseEvent {
    pub response_type: u8,
    pub detail: Button,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl ButtonReleaseEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = Button::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ButtonReleaseEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ButtonReleaseEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ButtonReleaseEvent> for [u8; 32] {
    fn from(input: &ButtonReleaseEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = input.detail.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], 0
        ]
    }
}
impl From<ButtonReleaseEvent> for [u8; 32] {
    fn from(input: ButtonReleaseEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Motion {
    Normal = 0,
    Hint = 1,
}
impl From<Motion> for u8 {
    fn from(input: Motion) -> Self {
        match input {
            Motion::Normal => 0,
            Motion::Hint => 1,
        }
    }
}
impl From<Motion> for Option<u8> {
    fn from(input: Motion) -> Self {
        Some(u8::from(input))
    }
}
impl From<Motion> for u16 {
    fn from(input: Motion) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Motion> for Option<u16> {
    fn from(input: Motion) -> Self {
        Some(u16::from(input))
    }
}
impl From<Motion> for u32 {
    fn from(input: Motion) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Motion> for Option<u32> {
    fn from(input: Motion) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Motion {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Motion::Normal),
            1 => Ok(Motion::Hint),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Motion {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Motion {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the MotionNotify event
pub const MOTION_NOTIFY_EVENT: u8 = 6;
/// a key was pressed.
///
///
/// # Fields
///
/// * `detail` - The keycode (a number representing a physical key on the keyboard) of the key
/// which was pressed.
/// * `event_x` - If `same_screen` is true, this is the X coordinate relative to the `event`
/// window's origin. Otherwise, `event_x` will be set to zero.
/// * `event_y` - If `same_screen` is true, this is the Y coordinate relative to the `event`
/// window's origin. Otherwise, `event_y` will be set to zero.
/// * `root` - The root window of `child`.
/// * `root_x` - The X coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `root_y` - The Y coordinate of the pointer relative to the `root` window at the time of
/// the event.
/// * `same_screen` - Whether the `event` window is on the same screen as the `root` window.
/// * `state` - The logical state of the pointer buttons and modifier keys just prior to the
/// event.
/// * `time` - Time when the event was generated (in milliseconds).
///
/// # See
///
/// * GrabKey: request
/// * GrabKeyboard: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionNotifyEvent {
    pub response_type: u8,
    pub detail: Motion,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl MotionNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let detail = detail.try_into()?;
        let result = MotionNotifyEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for MotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for MotionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&MotionNotifyEvent> for [u8; 32] {
    fn from(input: &MotionNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = u8::from(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let same_screen = input.same_screen.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], same_screen[0], 0
        ]
    }
}
impl From<MotionNotifyEvent> for [u8; 32] {
    fn from(input: MotionNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NotifyDetail {
    Ancestor = 0,
    Virtual = 1,
    Inferior = 2,
    Nonlinear = 3,
    NonlinearVirtual = 4,
    Pointer = 5,
    PointerRoot = 6,
    None = 7,
}
impl From<NotifyDetail> for u8 {
    fn from(input: NotifyDetail) -> Self {
        match input {
            NotifyDetail::Ancestor => 0,
            NotifyDetail::Virtual => 1,
            NotifyDetail::Inferior => 2,
            NotifyDetail::Nonlinear => 3,
            NotifyDetail::NonlinearVirtual => 4,
            NotifyDetail::Pointer => 5,
            NotifyDetail::PointerRoot => 6,
            NotifyDetail::None => 7,
        }
    }
}
impl From<NotifyDetail> for Option<u8> {
    fn from(input: NotifyDetail) -> Self {
        Some(u8::from(input))
    }
}
impl From<NotifyDetail> for u16 {
    fn from(input: NotifyDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyDetail> for Option<u16> {
    fn from(input: NotifyDetail) -> Self {
        Some(u16::from(input))
    }
}
impl From<NotifyDetail> for u32 {
    fn from(input: NotifyDetail) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyDetail> for Option<u32> {
    fn from(input: NotifyDetail) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NotifyDetail::Ancestor),
            1 => Ok(NotifyDetail::Virtual),
            2 => Ok(NotifyDetail::Inferior),
            3 => Ok(NotifyDetail::Nonlinear),
            4 => Ok(NotifyDetail::NonlinearVirtual),
            5 => Ok(NotifyDetail::Pointer),
            6 => Ok(NotifyDetail::PointerRoot),
            7 => Ok(NotifyDetail::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NotifyDetail {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NotifyMode {
    Normal = 0,
    Grab = 1,
    Ungrab = 2,
    WhileGrabbed = 3,
}
impl From<NotifyMode> for u8 {
    fn from(input: NotifyMode) -> Self {
        match input {
            NotifyMode::Normal => 0,
            NotifyMode::Grab => 1,
            NotifyMode::Ungrab => 2,
            NotifyMode::WhileGrabbed => 3,
        }
    }
}
impl From<NotifyMode> for Option<u8> {
    fn from(input: NotifyMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<NotifyMode> for u16 {
    fn from(input: NotifyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMode> for Option<u16> {
    fn from(input: NotifyMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<NotifyMode> for u32 {
    fn from(input: NotifyMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<NotifyMode> for Option<u32> {
    fn from(input: NotifyMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NotifyMode::Normal),
            1 => Ok(NotifyMode::Grab),
            2 => Ok(NotifyMode::Ungrab),
            3 => Ok(NotifyMode::WhileGrabbed),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for NotifyMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the EnterNotify event
pub const ENTER_NOTIFY_EVENT: u8 = 7;
/// the pointer is in a different window.
///
///
/// # Fields
///
/// * `child` - If the `event` window has subwindows and the final pointer position is in one
/// of them, then `child` is set to that subwindow, `XCB_WINDOW_NONE` otherwise.
/// * `event` - The window on which the event was generated.
/// * `event_x` - If `event` is on the same screen as `root`, this is the pointer X coordinate
/// relative to the event window's origin.
/// * `event_y` - If `event` is on the same screen as `root`, this is the pointer Y coordinate
/// relative to the event window's origin.
/// * `root` - The root window for the final cursor position.
/// * `root_x` - The pointer X coordinate relative to `root`'s origin at the time of the event.
/// * `root_y` - The pointer Y coordinate relative to `root`'s origin at the time of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnterNotifyEvent {
    pub response_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub mode: NotifyMode,
    pub same_screen_focus: u8,
}
impl EnterNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (same_screen_focus, remaining) = u8::try_parse(remaining)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = EnterNotifyEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, mode, same_screen_focus };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for EnterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for EnterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for EnterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&EnterNotifyEvent> for [u8; 32] {
    fn from(input: &EnterNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = u8::from(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let mode = u8::from(input.mode).serialize();
        let same_screen_focus = input.same_screen_focus.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], mode[0], same_screen_focus[0]
        ]
    }
}
impl From<EnterNotifyEvent> for [u8; 32] {
    fn from(input: EnterNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the LeaveNotify event
pub const LEAVE_NOTIFY_EVENT: u8 = 8;
/// the pointer is in a different window.
///
///
/// # Fields
///
/// * `child` - If the `event` window has subwindows and the final pointer position is in one
/// of them, then `child` is set to that subwindow, `XCB_WINDOW_NONE` otherwise.
/// * `event` - The window on which the event was generated.
/// * `event_x` - If `event` is on the same screen as `root`, this is the pointer X coordinate
/// relative to the event window's origin.
/// * `event_y` - If `event` is on the same screen as `root`, this is the pointer Y coordinate
/// relative to the event window's origin.
/// * `root` - The root window for the final cursor position.
/// * `root_x` - The pointer X coordinate relative to `root`'s origin at the time of the event.
/// * `root_y` - The pointer Y coordinate relative to `root`'s origin at the time of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaveNotifyEvent {
    pub response_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub mode: NotifyMode,
    pub same_screen_focus: u8,
}
impl LeaveNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (same_screen_focus, remaining) = u8::try_parse(remaining)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = LeaveNotifyEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, mode, same_screen_focus };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LeaveNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for LeaveNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for LeaveNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&LeaveNotifyEvent> for [u8; 32] {
    fn from(input: &LeaveNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = u8::from(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let root = input.root.serialize();
        let event = input.event.serialize();
        let child = input.child.serialize();
        let root_x = input.root_x.serialize();
        let root_y = input.root_y.serialize();
        let event_x = input.event_x.serialize();
        let event_y = input.event_y.serialize();
        let state = input.state.serialize();
        let mode = u8::from(input.mode).serialize();
        let same_screen_focus = input.same_screen_focus.serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], mode[0], same_screen_focus[0]
        ]
    }
}
impl From<LeaveNotifyEvent> for [u8; 32] {
    fn from(input: LeaveNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the FocusIn event
pub const FOCUS_IN_EVENT: u8 = 9;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `event` - The window on which the focus event was generated. This is the window used by
/// the X server to report the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusInEvent {
    pub response_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub event: Window,
    pub mode: NotifyMode,
}
impl FocusInEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = FocusInEvent { response_type, detail, sequence, event, mode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for FocusInEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&FocusInEvent> for [u8; 32] {
    fn from(input: &FocusInEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = u8::from(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let mode = u8::from(input.mode).serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            mode[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<FocusInEvent> for [u8; 32] {
    fn from(input: FocusInEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the FocusOut event
pub const FOCUS_OUT_EVENT: u8 = 10;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `event` - The window on which the focus event was generated. This is the window used by
/// the X server to report the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusOutEvent {
    pub response_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub event: Window,
    pub mode: NotifyMode,
}
impl FocusOutEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let detail = detail.try_into()?;
        let mode = mode.try_into()?;
        let result = FocusOutEvent { response_type, detail, sequence, event, mode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for FocusOutEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&FocusOutEvent> for [u8; 32] {
    fn from(input: &FocusOutEvent) -> Self {
        let response_type = input.response_type.serialize();
        let detail = u8::from(input.detail).serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let mode = u8::from(input.mode).serialize();
        [
            response_type[0], detail[0], sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            mode[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<FocusOutEvent> for [u8; 32] {
    fn from(input: FocusOutEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the KeymapNotify event
pub const KEYMAP_NOTIFY_EVENT: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeymapNotifyEvent {
    pub response_type: u8,
    pub keys: [u8; 31],
}
impl KeymapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let (keys_4, remaining) = u8::try_parse(remaining)?;
        let (keys_5, remaining) = u8::try_parse(remaining)?;
        let (keys_6, remaining) = u8::try_parse(remaining)?;
        let (keys_7, remaining) = u8::try_parse(remaining)?;
        let (keys_8, remaining) = u8::try_parse(remaining)?;
        let (keys_9, remaining) = u8::try_parse(remaining)?;
        let (keys_10, remaining) = u8::try_parse(remaining)?;
        let (keys_11, remaining) = u8::try_parse(remaining)?;
        let (keys_12, remaining) = u8::try_parse(remaining)?;
        let (keys_13, remaining) = u8::try_parse(remaining)?;
        let (keys_14, remaining) = u8::try_parse(remaining)?;
        let (keys_15, remaining) = u8::try_parse(remaining)?;
        let (keys_16, remaining) = u8::try_parse(remaining)?;
        let (keys_17, remaining) = u8::try_parse(remaining)?;
        let (keys_18, remaining) = u8::try_parse(remaining)?;
        let (keys_19, remaining) = u8::try_parse(remaining)?;
        let (keys_20, remaining) = u8::try_parse(remaining)?;
        let (keys_21, remaining) = u8::try_parse(remaining)?;
        let (keys_22, remaining) = u8::try_parse(remaining)?;
        let (keys_23, remaining) = u8::try_parse(remaining)?;
        let (keys_24, remaining) = u8::try_parse(remaining)?;
        let (keys_25, remaining) = u8::try_parse(remaining)?;
        let (keys_26, remaining) = u8::try_parse(remaining)?;
        let (keys_27, remaining) = u8::try_parse(remaining)?;
        let (keys_28, remaining) = u8::try_parse(remaining)?;
        let (keys_29, remaining) = u8::try_parse(remaining)?;
        let (keys_30, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
            keys_4,
            keys_5,
            keys_6,
            keys_7,
            keys_8,
            keys_9,
            keys_10,
            keys_11,
            keys_12,
            keys_13,
            keys_14,
            keys_15,
            keys_16,
            keys_17,
            keys_18,
            keys_19,
            keys_20,
            keys_21,
            keys_22,
            keys_23,
            keys_24,
            keys_25,
            keys_26,
            keys_27,
            keys_28,
            keys_29,
            keys_30,
        ];
        let result = KeymapNotifyEvent { response_type, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeymapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for KeymapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for KeymapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&KeymapNotifyEvent> for [u8; 32] {
    fn from(input: &KeymapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        [
            response_type[0], input.keys[0], input.keys[1], input.keys[2], input.keys[3], input.keys[4], input.keys[5], input.keys[6],
            input.keys[7], input.keys[8], input.keys[9], input.keys[10], input.keys[11], input.keys[12], input.keys[13], input.keys[14],
            input.keys[15], input.keys[16], input.keys[17], input.keys[18], input.keys[19], input.keys[20], input.keys[21], input.keys[22],
            input.keys[23], input.keys[24], input.keys[25], input.keys[26], input.keys[27], input.keys[28], input.keys[29], input.keys[30]
        ]
    }
}
impl From<KeymapNotifyEvent> for [u8; 32] {
    fn from(input: KeymapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Expose event
pub const EXPOSE_EVENT: u8 = 12;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `count` - The amount of `Expose` events following this one. Simple applications that do
/// not want to optimize redisplay by distinguishing between subareas of its window
/// can just ignore all Expose events with nonzero counts and perform full
/// redisplays on events with zero counts.
/// * `height` - The height of the exposed rectangle.
/// * `width` - The width of the exposed rectangle.
/// * `window` - The exposed (damaged) window.
/// * `x` - The X coordinate of the left-upper corner of the exposed rectangle, relative to
/// the `window`'s origin.
/// * `y` - The Y coordinate of the left-upper corner of the exposed rectangle, relative to
/// the `window`'s origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExposeEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}
impl ExposeEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = u16::try_parse(remaining)?;
        let (y, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (count, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = ExposeEvent { response_type, sequence, window, x, y, width, height, count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ExposeEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ExposeEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ExposeEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ExposeEvent> for [u8; 32] {
    fn from(input: &ExposeEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let count = input.count.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            x[0], x[1], y[0], y[1], width[0], width[1], height[0], height[1],
            count[0], count[1], 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ExposeEvent> for [u8; 32] {
    fn from(input: ExposeEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GraphicsExposure event
pub const GRAPHICS_EXPOSURE_EVENT: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsExposureEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
}
impl GraphicsExposureEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (x, remaining) = u16::try_parse(remaining)?;
        let (y, remaining) = u16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (count, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = GraphicsExposureEvent { response_type, sequence, drawable, x, y, width, height, minor_opcode, count, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GraphicsExposureEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for GraphicsExposureEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for GraphicsExposureEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&GraphicsExposureEvent> for [u8; 32] {
    fn from(input: &GraphicsExposureEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let drawable = input.drawable.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let count = input.count.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], drawable[0], drawable[1], drawable[2], drawable[3],
            x[0], x[1], y[0], y[1], width[0], width[1], height[0], height[1],
            minor_opcode[0], minor_opcode[1], count[0], count[1], major_opcode[0], 0, 0, 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GraphicsExposureEvent> for [u8; 32] {
    fn from(input: GraphicsExposureEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the NoExposure event
pub const NO_EXPOSURE_EVENT: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoExposureEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl NoExposureEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (drawable, remaining) = Drawable::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = NoExposureEvent { response_type, sequence, drawable, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NoExposureEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for NoExposureEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for NoExposureEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&NoExposureEvent> for [u8; 32] {
    fn from(input: &NoExposureEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let drawable = input.drawable.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], drawable[0], drawable[1], drawable[2], drawable[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<NoExposureEvent> for [u8; 32] {
    fn from(input: NoExposureEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Unobscured = 0,
    PartiallyObscured = 1,
    FullyObscured = 2,
}
impl From<Visibility> for u8 {
    fn from(input: Visibility) -> Self {
        match input {
            Visibility::Unobscured => 0,
            Visibility::PartiallyObscured => 1,
            Visibility::FullyObscured => 2,
        }
    }
}
impl From<Visibility> for Option<u8> {
    fn from(input: Visibility) -> Self {
        Some(u8::from(input))
    }
}
impl From<Visibility> for u16 {
    fn from(input: Visibility) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Visibility> for Option<u16> {
    fn from(input: Visibility) -> Self {
        Some(u16::from(input))
    }
}
impl From<Visibility> for u32 {
    fn from(input: Visibility) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Visibility> for Option<u32> {
    fn from(input: Visibility) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Visibility {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Visibility::Unobscured),
            1 => Ok(Visibility::PartiallyObscured),
            2 => Ok(Visibility::FullyObscured),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Visibility {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Visibility {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the VisibilityNotify event
pub const VISIBILITY_NOTIFY_EVENT: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisibilityNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub state: Visibility,
}
impl VisibilityNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let result = VisibilityNotifyEvent { response_type, sequence, window, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for VisibilityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for VisibilityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for VisibilityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&VisibilityNotifyEvent> for [u8; 32] {
    fn from(input: &VisibilityNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let state = u8::from(input.state).serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            state[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<VisibilityNotifyEvent> for [u8; 32] {
    fn from(input: VisibilityNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the CreateNotify event
pub const CREATE_NOTIFY_EVENT: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}
impl CreateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (parent, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (border_width, remaining) = u16::try_parse(remaining)?;
        let (override_redirect, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = CreateNotifyEvent { response_type, sequence, parent, window, x, y, width, height, border_width, override_redirect };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CreateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CreateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CreateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CreateNotifyEvent> for [u8; 32] {
    fn from(input: &CreateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let parent = input.parent.serialize();
        let window = input.window.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let border_width = input.border_width.serialize();
        let override_redirect = input.override_redirect.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], parent[0], parent[1], parent[2], parent[3],
            window[0], window[1], window[2], window[3], x[0], x[1], y[0], y[1],
            width[0], width[1], height[0], height[1], border_width[0], border_width[1], override_redirect[0], 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<CreateNotifyEvent> for [u8; 32] {
    fn from(input: CreateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the DestroyNotify event
pub const DESTROY_NOTIFY_EVENT: u8 = 17;
/// a window is destroyed.
///
///
/// # Fields
///
/// * `event` - The reconfigured window or its parent, depending on whether `StructureNotify`
/// or `SubstructureNotify` was selected.
/// * `window` - The window that is destroyed.
///
/// # See
///
/// * DestroyWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
}
impl DestroyNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let result = DestroyNotifyEvent { response_type, sequence, event, window };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DestroyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for DestroyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for DestroyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&DestroyNotifyEvent> for [u8; 32] {
    fn from(input: &DestroyNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DestroyNotifyEvent> for [u8; 32] {
    fn from(input: DestroyNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the UnmapNotify event
pub const UNMAP_NOTIFY_EVENT: u8 = 18;
/// a window is unmapped.
///
///
/// # Fields
///
/// * `event` - The reconfigured window or its parent, depending on whether `StructureNotify`
/// or `SubstructureNotify` was selected.
/// * `from_configure` - Set to 1 if the event was generated as a result of a resizing of the window's
/// parent when `window` had a win_gravity of `UnmapGravity`.
/// * `window` - The window that was unmapped.
///
/// # See
///
/// * UnmapWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnmapNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub from_configure: bool,
}
impl UnmapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (from_configure, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = UnmapNotifyEvent { response_type, sequence, event, window, from_configure };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for UnmapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for UnmapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for UnmapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&UnmapNotifyEvent> for [u8; 32] {
    fn from(input: &UnmapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let from_configure = input.from_configure.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], from_configure[0], 0, 0, 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<UnmapNotifyEvent> for [u8; 32] {
    fn from(input: UnmapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the MapNotify event
pub const MAP_NOTIFY_EVENT: u8 = 19;
/// a window was mapped.
///
///
/// # Fields
///
/// * `event` - The window which was mapped or its parent, depending on whether
/// `StructureNotify` or `SubstructureNotify` was selected.
/// * `override_redirect` - Window managers should ignore this window if `override_redirect` is 1.
/// * `window` - The window that was mapped.
///
/// # See
///
/// * MapWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub override_redirect: bool,
}
impl MapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (override_redirect, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = MapNotifyEvent { response_type, sequence, event, window, override_redirect };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for MapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for MapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&MapNotifyEvent> for [u8; 32] {
    fn from(input: &MapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let override_redirect = input.override_redirect.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], override_redirect[0], 0, 0, 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<MapNotifyEvent> for [u8; 32] {
    fn from(input: MapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the MapRequest event
pub const MAP_REQUEST_EVENT: u8 = 20;
/// window wants to be mapped.
///
///
/// # Fields
///
/// * `parent` - The parent of `window`.
/// * `window` - The window to be mapped.
///
/// # See
///
/// * MapWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapRequestEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
}
impl MapRequestEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (parent, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let result = MapRequestEvent { response_type, sequence, parent, window };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MapRequestEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for MapRequestEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for MapRequestEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&MapRequestEvent> for [u8; 32] {
    fn from(input: &MapRequestEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let parent = input.parent.serialize();
        let window = input.window.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], parent[0], parent[1], parent[2], parent[3],
            window[0], window[1], window[2], window[3], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<MapRequestEvent> for [u8; 32] {
    fn from(input: MapRequestEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ReparentNotify event
pub const REPARENT_NOTIFY_EVENT: u8 = 21;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReparentNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: i16,
    pub y: i16,
    pub override_redirect: bool,
}
impl ReparentNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (parent, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (override_redirect, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let result = ReparentNotifyEvent { response_type, sequence, event, window, parent, x, y, override_redirect };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ReparentNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ReparentNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ReparentNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ReparentNotifyEvent> for [u8; 32] {
    fn from(input: &ReparentNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let parent = input.parent.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let override_redirect = input.override_redirect.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], parent[0], parent[1], parent[2], parent[3],
            x[0], x[1], y[0], y[1], override_redirect[0], 0, 0, 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ReparentNotifyEvent> for [u8; 32] {
    fn from(input: ReparentNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ConfigureNotify event
pub const CONFIGURE_NOTIFY_EVENT: u8 = 22;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `above_sibling` - If `XCB_NONE`, the `window` is on the bottom of the stack with respect to
/// sibling windows. However, if set to a sibling window, the `window` is placed on
/// top of this sibling window.
/// * `border_width` - The border width of `window`.
/// * `event` - The reconfigured window or its parent, depending on whether `StructureNotify`
/// or `SubstructureNotify` was selected.
/// * `height` - The inside height of `window`, not including the border.
/// * `override_redirect` - Window managers should ignore this window if `override_redirect` is 1.
/// * `width` - The inside width of `window`, not including the border.
/// * `window` - The window whose size, position, border, and/or stacking order was changed.
/// * `x` - The X coordinate of the upper-left outside corner of `window`, relative to the
/// parent window's origin.
/// * `y` - The Y coordinate of the upper-left outside corner of `window`, relative to the
/// parent window's origin.
///
/// # See
///
/// * FreeColormap: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigureNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub above_sibling: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}
impl ConfigureNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (above_sibling, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (border_width, remaining) = u16::try_parse(remaining)?;
        let (override_redirect, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ConfigureNotifyEvent { response_type, sequence, event, window, above_sibling, x, y, width, height, border_width, override_redirect };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ConfigureNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ConfigureNotifyEvent> for [u8; 32] {
    fn from(input: &ConfigureNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let above_sibling = input.above_sibling.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let border_width = input.border_width.serialize();
        let override_redirect = input.override_redirect.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], above_sibling[0], above_sibling[1], above_sibling[2], above_sibling[3],
            x[0], x[1], y[0], y[1], width[0], width[1], height[0], height[1],
            border_width[0], border_width[1], override_redirect[0], 0, /* trailing padding */ 0, 0, 0, 0
        ]
    }
}
impl From<ConfigureNotifyEvent> for [u8; 32] {
    fn from(input: ConfigureNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ConfigureRequest event
pub const CONFIGURE_REQUEST_EVENT: u8 = 23;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigureRequestEvent {
    pub response_type: u8,
    pub stack_mode: StackMode,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub sibling: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: u16,
}
impl ConfigureRequestEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (stack_mode, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (parent, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (sibling, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (border_width, remaining) = u16::try_parse(remaining)?;
        let (value_mask, remaining) = u16::try_parse(remaining)?;
        let stack_mode = stack_mode.try_into()?;
        let result = ConfigureRequestEvent { response_type, stack_mode, sequence, parent, window, sibling, x, y, width, height, border_width, value_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ConfigureRequestEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ConfigureRequestEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ConfigureRequestEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ConfigureRequestEvent> for [u8; 32] {
    fn from(input: &ConfigureRequestEvent) -> Self {
        let response_type = input.response_type.serialize();
        let stack_mode = u8::from(input.stack_mode).serialize();
        let sequence = input.sequence.serialize();
        let parent = input.parent.serialize();
        let window = input.window.serialize();
        let sibling = input.sibling.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        let border_width = input.border_width.serialize();
        let value_mask = input.value_mask.serialize();
        [
            response_type[0], stack_mode[0], sequence[0], sequence[1], parent[0], parent[1], parent[2], parent[3],
            window[0], window[1], window[2], window[3], sibling[0], sibling[1], sibling[2], sibling[3],
            x[0], x[1], y[0], y[1], width[0], width[1], height[0], height[1],
            border_width[0], border_width[1], value_mask[0], value_mask[1], /* trailing padding */ 0, 0, 0, 0
        ]
    }
}
impl From<ConfigureRequestEvent> for [u8; 32] {
    fn from(input: ConfigureRequestEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GravityNotify event
pub const GRAVITY_NOTIFY_EVENT: u8 = 24;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GravityNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
}
impl GravityNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let result = GravityNotifyEvent { response_type, sequence, event, window, x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GravityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for GravityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for GravityNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&GravityNotifyEvent> for [u8; 32] {
    fn from(input: &GravityNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let x = input.x.serialize();
        let y = input.y.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], x[0], x[1], y[0], y[1],
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GravityNotifyEvent> for [u8; 32] {
    fn from(input: GravityNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ResizeRequest event
pub const RESIZE_REQUEST_EVENT: u8 = 25;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResizeRequestEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub width: u16,
    pub height: u16,
}
impl ResizeRequestEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let result = ResizeRequestEvent { response_type, sequence, window, width, height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ResizeRequestEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ResizeRequestEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ResizeRequestEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ResizeRequestEvent> for [u8; 32] {
    fn from(input: &ResizeRequestEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let width = input.width.serialize();
        let height = input.height.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            width[0], width[1], height[0], height[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ResizeRequestEvent> for [u8; 32] {
    fn from(input: ResizeRequestEvent) -> Self {
        Self::from(&input)
    }
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `OnBottom` - The window is now below all siblings.
/// * `OnTop` - The window is now on top of all siblings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Place {
    OnTop = 0,
    OnBottom = 1,
}
impl From<Place> for u8 {
    fn from(input: Place) -> Self {
        match input {
            Place::OnTop => 0,
            Place::OnBottom => 1,
        }
    }
}
impl From<Place> for Option<u8> {
    fn from(input: Place) -> Self {
        Some(u8::from(input))
    }
}
impl From<Place> for u16 {
    fn from(input: Place) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Place> for Option<u16> {
    fn from(input: Place) -> Self {
        Some(u16::from(input))
    }
}
impl From<Place> for u32 {
    fn from(input: Place) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Place> for Option<u32> {
    fn from(input: Place) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Place {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Place::OnTop),
            1 => Ok(Place::OnBottom),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Place {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Place {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CirculateNotify event
pub const CIRCULATE_NOTIFY_EVENT: u8 = 26;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `event` - Either the restacked window or its parent, depending on whether
/// `StructureNotify` or `SubstructureNotify` was selected.
/// * `window` - The restacked window.
///
/// # See
///
/// * CirculateWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CirculateNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub place: Place,
}
impl CirculateNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (place, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let place = place.try_into()?;
        let result = CirculateNotifyEvent { response_type, sequence, event, window, place };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CirculateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CirculateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CirculateNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CirculateNotifyEvent> for [u8; 32] {
    fn from(input: &CirculateNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let place = u8::from(input.place).serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], 0, 0, 0, 0,
            place[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<CirculateNotifyEvent> for [u8; 32] {
    fn from(input: CirculateNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the CirculateRequest event
pub const CIRCULATE_REQUEST_EVENT: u8 = 27;
/// NOT YET DOCUMENTED.
///
///
/// # Fields
///
/// * `event` - Either the restacked window or its parent, depending on whether
/// `StructureNotify` or `SubstructureNotify` was selected.
/// * `window` - The restacked window.
///
/// # See
///
/// * CirculateWindow: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CirculateRequestEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub place: Place,
}
impl CirculateRequestEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (place, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let place = place.try_into()?;
        let result = CirculateRequestEvent { response_type, sequence, event, window, place };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CirculateRequestEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CirculateRequestEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CirculateRequestEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CirculateRequestEvent> for [u8; 32] {
    fn from(input: &CirculateRequestEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let event = input.event.serialize();
        let window = input.window.serialize();
        let place = u8::from(input.place).serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], event[0], event[1], event[2], event[3],
            window[0], window[1], window[2], window[3], 0, 0, 0, 0,
            place[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<CirculateRequestEvent> for [u8; 32] {
    fn from(input: CirculateRequestEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Property {
    NewValue = 0,
    Delete = 1,
}
impl From<Property> for u8 {
    fn from(input: Property) -> Self {
        match input {
            Property::NewValue => 0,
            Property::Delete => 1,
        }
    }
}
impl From<Property> for Option<u8> {
    fn from(input: Property) -> Self {
        Some(u8::from(input))
    }
}
impl From<Property> for u16 {
    fn from(input: Property) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Property> for Option<u16> {
    fn from(input: Property) -> Self {
        Some(u16::from(input))
    }
}
impl From<Property> for u32 {
    fn from(input: Property) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Property> for Option<u32> {
    fn from(input: Property) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Property {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Property::NewValue),
            1 => Ok(Property::Delete),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Property {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Property {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the PropertyNotify event
pub const PROPERTY_NOTIFY_EVENT: u8 = 28;
/// a window property changed.
///
///
/// # Fields
///
/// * `atom` - The property's atom, to indicate which property was changed.
/// * `time` - A timestamp of the server time when the property was changed.
/// * `window` - The window whose associated property was changed.
///
/// # See
///
/// * ChangeProperty: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub atom: Atom,
    pub time: Timestamp,
    pub state: Property,
}
impl PropertyNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (atom, remaining) = Atom::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let result = PropertyNotifyEvent { response_type, sequence, window, atom, time, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for PropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for PropertyNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&PropertyNotifyEvent> for [u8; 32] {
    fn from(input: &PropertyNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let atom = input.atom.serialize();
        let time = input.time.serialize();
        let state = u8::from(input.state).serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            atom[0], atom[1], atom[2], atom[3], time[0], time[1], time[2], time[3],
            state[0], 0, 0, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PropertyNotifyEvent> for [u8; 32] {
    fn from(input: PropertyNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the SelectionClear event
pub const SELECTION_CLEAR_EVENT: u8 = 29;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionClearEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub owner: Window,
    pub selection: Atom,
}
impl SelectionClearEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (owner, remaining) = Window::try_parse(remaining)?;
        let (selection, remaining) = Atom::try_parse(remaining)?;
        let result = SelectionClearEvent { response_type, sequence, time, owner, selection };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SelectionClearEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for SelectionClearEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for SelectionClearEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&SelectionClearEvent> for [u8; 32] {
    fn from(input: &SelectionClearEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let owner = input.owner.serialize();
        let selection = input.selection.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            owner[0], owner[1], owner[2], owner[3], selection[0], selection[1], selection[2], selection[3],
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<SelectionClearEvent> for [u8; 32] {
    fn from(input: SelectionClearEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Time {
    CurrentTime = 0,
}
impl From<Time> for u8 {
    fn from(input: Time) -> Self {
        match input {
            Time::CurrentTime => 0,
        }
    }
}
impl From<Time> for Option<u8> {
    fn from(input: Time) -> Self {
        Some(u8::from(input))
    }
}
impl From<Time> for u16 {
    fn from(input: Time) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Time> for Option<u16> {
    fn from(input: Time) -> Self {
        Some(u16::from(input))
    }
}
impl From<Time> for u32 {
    fn from(input: Time) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Time> for Option<u32> {
    fn from(input: Time) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Time {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Time::CurrentTime),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Time {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Time {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AtomEnum {
    None,
    Any,
    PRIMARY,
    SECONDARY,
    ARC,
    ATOM,
    BITMAP,
    CARDINAL,
    COLORMAP,
    CURSOR,
    CUT_BUFFER0,
    CUT_BUFFER1,
    CUT_BUFFER2,
    CUT_BUFFER3,
    CUT_BUFFER4,
    CUT_BUFFER5,
    CUT_BUFFER6,
    CUT_BUFFER7,
    DRAWABLE,
    FONT,
    INTEGER,
    PIXMAP,
    POINT,
    RECTANGLE,
    RESOURCE_MANAGER,
    RGB_COLOR_MAP,
    RGB_BEST_MAP,
    RGB_BLUE_MAP,
    RGB_DEFAULT_MAP,
    RGB_GRAY_MAP,
    RGB_GREEN_MAP,
    RGB_RED_MAP,
    STRING,
    VISUALID,
    WINDOW,
    WM_COMMAND,
    WM_HINTS,
    WM_CLIENT_MACHINE,
    WM_ICON_NAME,
    WM_ICON_SIZE,
    WM_NAME,
    WM_NORMAL_HINTS,
    WM_SIZE_HINTS,
    WM_ZOOM_HINTS,
    MIN_SPACE,
    NORM_SPACE,
    MAX_SPACE,
    END_SPACE,
    SUPERSCRIPT_X,
    SUPERSCRIPT_Y,
    SUBSCRIPT_X,
    SUBSCRIPT_Y,
    UNDERLINE_POSITION,
    UNDERLINE_THICKNESS,
    STRIKEOUT_ASCENT,
    STRIKEOUT_DESCENT,
    ITALIC_ANGLE,
    X_HEIGHT,
    QUAD_WIDTH,
    WEIGHT,
    POINT_SIZE,
    RESOLUTION,
    COPYRIGHT,
    NOTICE,
    FONT_NAME,
    FAMILY_NAME,
    FULL_NAME,
    CAP_HEIGHT,
    WM_CLASS,
    WM_TRANSIENT_FOR,
}
impl From<AtomEnum> for u8 {
    fn from(input: AtomEnum) -> Self {
        match input {
            AtomEnum::None => 0,
            AtomEnum::Any => 0,
            AtomEnum::PRIMARY => 1,
            AtomEnum::SECONDARY => 2,
            AtomEnum::ARC => 3,
            AtomEnum::ATOM => 4,
            AtomEnum::BITMAP => 5,
            AtomEnum::CARDINAL => 6,
            AtomEnum::COLORMAP => 7,
            AtomEnum::CURSOR => 8,
            AtomEnum::CUT_BUFFER0 => 9,
            AtomEnum::CUT_BUFFER1 => 10,
            AtomEnum::CUT_BUFFER2 => 11,
            AtomEnum::CUT_BUFFER3 => 12,
            AtomEnum::CUT_BUFFER4 => 13,
            AtomEnum::CUT_BUFFER5 => 14,
            AtomEnum::CUT_BUFFER6 => 15,
            AtomEnum::CUT_BUFFER7 => 16,
            AtomEnum::DRAWABLE => 17,
            AtomEnum::FONT => 18,
            AtomEnum::INTEGER => 19,
            AtomEnum::PIXMAP => 20,
            AtomEnum::POINT => 21,
            AtomEnum::RECTANGLE => 22,
            AtomEnum::RESOURCE_MANAGER => 23,
            AtomEnum::RGB_COLOR_MAP => 24,
            AtomEnum::RGB_BEST_MAP => 25,
            AtomEnum::RGB_BLUE_MAP => 26,
            AtomEnum::RGB_DEFAULT_MAP => 27,
            AtomEnum::RGB_GRAY_MAP => 28,
            AtomEnum::RGB_GREEN_MAP => 29,
            AtomEnum::RGB_RED_MAP => 30,
            AtomEnum::STRING => 31,
            AtomEnum::VISUALID => 32,
            AtomEnum::WINDOW => 33,
            AtomEnum::WM_COMMAND => 34,
            AtomEnum::WM_HINTS => 35,
            AtomEnum::WM_CLIENT_MACHINE => 36,
            AtomEnum::WM_ICON_NAME => 37,
            AtomEnum::WM_ICON_SIZE => 38,
            AtomEnum::WM_NAME => 39,
            AtomEnum::WM_NORMAL_HINTS => 40,
            AtomEnum::WM_SIZE_HINTS => 41,
            AtomEnum::WM_ZOOM_HINTS => 42,
            AtomEnum::MIN_SPACE => 43,
            AtomEnum::NORM_SPACE => 44,
            AtomEnum::MAX_SPACE => 45,
            AtomEnum::END_SPACE => 46,
            AtomEnum::SUPERSCRIPT_X => 47,
            AtomEnum::SUPERSCRIPT_Y => 48,
            AtomEnum::SUBSCRIPT_X => 49,
            AtomEnum::SUBSCRIPT_Y => 50,
            AtomEnum::UNDERLINE_POSITION => 51,
            AtomEnum::UNDERLINE_THICKNESS => 52,
            AtomEnum::STRIKEOUT_ASCENT => 53,
            AtomEnum::STRIKEOUT_DESCENT => 54,
            AtomEnum::ITALIC_ANGLE => 55,
            AtomEnum::X_HEIGHT => 56,
            AtomEnum::QUAD_WIDTH => 57,
            AtomEnum::WEIGHT => 58,
            AtomEnum::POINT_SIZE => 59,
            AtomEnum::RESOLUTION => 60,
            AtomEnum::COPYRIGHT => 61,
            AtomEnum::NOTICE => 62,
            AtomEnum::FONT_NAME => 63,
            AtomEnum::FAMILY_NAME => 64,
            AtomEnum::FULL_NAME => 65,
            AtomEnum::CAP_HEIGHT => 66,
            AtomEnum::WM_CLASS => 67,
            AtomEnum::WM_TRANSIENT_FOR => 68,
        }
    }
}
impl From<AtomEnum> for Option<u8> {
    fn from(input: AtomEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<AtomEnum> for u16 {
    fn from(input: AtomEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AtomEnum> for Option<u16> {
    fn from(input: AtomEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<AtomEnum> for u32 {
    fn from(input: AtomEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AtomEnum> for Option<u32> {
    fn from(input: AtomEnum) -> Self {
        Some(u32::from(input))
    }
}

/// Opcode for the SelectionRequest event
pub const SELECTION_REQUEST_EVENT: u8 = 30;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRequestEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
}
impl SelectionRequestEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (owner, remaining) = Window::try_parse(remaining)?;
        let (requestor, remaining) = Window::try_parse(remaining)?;
        let (selection, remaining) = Atom::try_parse(remaining)?;
        let (target, remaining) = Atom::try_parse(remaining)?;
        let (property, remaining) = Atom::try_parse(remaining)?;
        let result = SelectionRequestEvent { response_type, sequence, time, owner, requestor, selection, target, property };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SelectionRequestEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for SelectionRequestEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for SelectionRequestEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&SelectionRequestEvent> for [u8; 32] {
    fn from(input: &SelectionRequestEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let owner = input.owner.serialize();
        let requestor = input.requestor.serialize();
        let selection = input.selection.serialize();
        let target = input.target.serialize();
        let property = input.property.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            owner[0], owner[1], owner[2], owner[3], requestor[0], requestor[1], requestor[2], requestor[3],
            selection[0], selection[1], selection[2], selection[3], target[0], target[1], target[2], target[3],
            property[0], property[1], property[2], property[3], /* trailing padding */ 0, 0, 0, 0
        ]
    }
}
impl From<SelectionRequestEvent> for [u8; 32] {
    fn from(input: SelectionRequestEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the SelectionNotify event
pub const SELECTION_NOTIFY_EVENT: u8 = 31;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
}
impl SelectionNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (requestor, remaining) = Window::try_parse(remaining)?;
        let (selection, remaining) = Atom::try_parse(remaining)?;
        let (target, remaining) = Atom::try_parse(remaining)?;
        let (property, remaining) = Atom::try_parse(remaining)?;
        let result = SelectionNotifyEvent { response_type, sequence, time, requestor, selection, target, property };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for SelectionNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&SelectionNotifyEvent> for [u8; 32] {
    fn from(input: &SelectionNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let time = input.time.serialize();
        let requestor = input.requestor.serialize();
        let selection = input.selection.serialize();
        let target = input.target.serialize();
        let property = input.property.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            requestor[0], requestor[1], requestor[2], requestor[3], selection[0], selection[1], selection[2], selection[3],
            target[0], target[1], target[2], target[3], property[0], property[1], property[2], property[3],
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<SelectionNotifyEvent> for [u8; 32] {
    fn from(input: SelectionNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `Installed` - The colormap was installed.
/// * `Uninstalled` - The colormap was uninstalled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColormapState {
    Uninstalled = 0,
    Installed = 1,
}
impl From<ColormapState> for u8 {
    fn from(input: ColormapState) -> Self {
        match input {
            ColormapState::Uninstalled => 0,
            ColormapState::Installed => 1,
        }
    }
}
impl From<ColormapState> for Option<u8> {
    fn from(input: ColormapState) -> Self {
        Some(u8::from(input))
    }
}
impl From<ColormapState> for u16 {
    fn from(input: ColormapState) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapState> for Option<u16> {
    fn from(input: ColormapState) -> Self {
        Some(u16::from(input))
    }
}
impl From<ColormapState> for u32 {
    fn from(input: ColormapState) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapState> for Option<u32> {
    fn from(input: ColormapState) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ColormapState {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColormapState::Uninstalled),
            1 => Ok(ColormapState::Installed),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ColormapState {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ColormapState {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColormapEnum {
    None = 0,
}
impl From<ColormapEnum> for u8 {
    fn from(input: ColormapEnum) -> Self {
        match input {
            ColormapEnum::None => 0,
        }
    }
}
impl From<ColormapEnum> for Option<u8> {
    fn from(input: ColormapEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<ColormapEnum> for u16 {
    fn from(input: ColormapEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapEnum> for Option<u16> {
    fn from(input: ColormapEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<ColormapEnum> for u32 {
    fn from(input: ColormapEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapEnum> for Option<u32> {
    fn from(input: ColormapEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ColormapEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColormapEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ColormapEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ColormapEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ColormapNotify event
pub const COLORMAP_NOTIFY_EVENT: u8 = 32;
/// the colormap for some window changed.
///
///
/// # Fields
///
/// * `_new` - Indicates whether the colormap was changed (1) or installed/uninstalled (0).
/// * `colormap` - The colormap which is changed, installed or uninstalled. This is `XCB_NONE`
/// when the colormap is changed by a call to `FreeColormap`.
/// * `window` - The window whose associated colormap is changed, installed or uninstalled.
///
/// # See
///
/// * FreeColormap: request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColormapNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub colormap: Colormap,
    pub new: bool,
    pub state: ColormapState,
}
impl ColormapNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (colormap, remaining) = Colormap::try_parse(remaining)?;
        let (new, remaining) = bool::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let result = ColormapNotifyEvent { response_type, sequence, window, colormap, new, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ColormapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ColormapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ColormapNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ColormapNotifyEvent> for [u8; 32] {
    fn from(input: &ColormapNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let colormap = input.colormap.serialize();
        let new = input.new.serialize();
        let state = u8::from(input.state).serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            colormap[0], colormap[1], colormap[2], colormap[3], new[0], state[0], 0, 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ColormapNotifyEvent> for [u8; 32] {
    fn from(input: ColormapNotifyEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClientMessageData([u8; 20]);
impl ClientMessageData {
    pub fn as_data8(&self) -> [u8; 20] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u8; 20], ParseError> {
            let (data8_0, remaining) = u8::try_parse(remaining)?;
            let (data8_1, remaining) = u8::try_parse(remaining)?;
            let (data8_2, remaining) = u8::try_parse(remaining)?;
            let (data8_3, remaining) = u8::try_parse(remaining)?;
            let (data8_4, remaining) = u8::try_parse(remaining)?;
            let (data8_5, remaining) = u8::try_parse(remaining)?;
            let (data8_6, remaining) = u8::try_parse(remaining)?;
            let (data8_7, remaining) = u8::try_parse(remaining)?;
            let (data8_8, remaining) = u8::try_parse(remaining)?;
            let (data8_9, remaining) = u8::try_parse(remaining)?;
            let (data8_10, remaining) = u8::try_parse(remaining)?;
            let (data8_11, remaining) = u8::try_parse(remaining)?;
            let (data8_12, remaining) = u8::try_parse(remaining)?;
            let (data8_13, remaining) = u8::try_parse(remaining)?;
            let (data8_14, remaining) = u8::try_parse(remaining)?;
            let (data8_15, remaining) = u8::try_parse(remaining)?;
            let (data8_16, remaining) = u8::try_parse(remaining)?;
            let (data8_17, remaining) = u8::try_parse(remaining)?;
            let (data8_18, remaining) = u8::try_parse(remaining)?;
            let (data8_19, remaining) = u8::try_parse(remaining)?;
            let data8 = [
                data8_0,
                data8_1,
                data8_2,
                data8_3,
                data8_4,
                data8_5,
                data8_6,
                data8_7,
                data8_8,
                data8_9,
                data8_10,
                data8_11,
                data8_12,
                data8_13,
                data8_14,
                data8_15,
                data8_16,
                data8_17,
                data8_18,
                data8_19,
            ];
            let _ = remaining;
            Ok(data8)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_data16(&self) -> [u16; 10] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u16; 10], ParseError> {
            let (data16_0, remaining) = u16::try_parse(remaining)?;
            let (data16_1, remaining) = u16::try_parse(remaining)?;
            let (data16_2, remaining) = u16::try_parse(remaining)?;
            let (data16_3, remaining) = u16::try_parse(remaining)?;
            let (data16_4, remaining) = u16::try_parse(remaining)?;
            let (data16_5, remaining) = u16::try_parse(remaining)?;
            let (data16_6, remaining) = u16::try_parse(remaining)?;
            let (data16_7, remaining) = u16::try_parse(remaining)?;
            let (data16_8, remaining) = u16::try_parse(remaining)?;
            let (data16_9, remaining) = u16::try_parse(remaining)?;
            let data16 = [
                data16_0,
                data16_1,
                data16_2,
                data16_3,
                data16_4,
                data16_5,
                data16_6,
                data16_7,
                data16_8,
                data16_9,
            ];
            let _ = remaining;
            Ok(data16)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_data32(&self) -> [u32; 5] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u32; 5], ParseError> {
            let (data32_0, remaining) = u32::try_parse(remaining)?;
            let (data32_1, remaining) = u32::try_parse(remaining)?;
            let (data32_2, remaining) = u32::try_parse(remaining)?;
            let (data32_3, remaining) = u32::try_parse(remaining)?;
            let (data32_4, remaining) = u32::try_parse(remaining)?;
            let data32 = [
                data32_0,
                data32_1,
                data32_2,
                data32_3,
                data32_4,
            ];
            let _ = remaining;
            Ok(data32)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for ClientMessageData {
    type Bytes = [u8; 20];
    fn serialize(&self) -> Self::Bytes {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for ClientMessageData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 20] = value.get(..20)
            .ok_or(ParseError::ParseError)?
            .try_into()
            .unwrap();
        let result = ClientMessageData(inner);
        Ok((result, &value[20..]))
    }
}
impl From<[u8; 20]> for ClientMessageData {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}
impl From<[u16; 10]> for ClientMessageData {
    fn from(value: [u16; 10]) -> Self {
        let value0 = value[0].serialize();
        let value1 = value[1].serialize();
        let value2 = value[2].serialize();
        let value3 = value[3].serialize();
        let value4 = value[4].serialize();
        let value5 = value[5].serialize();
        let value6 = value[6].serialize();
        let value7 = value[7].serialize();
        let value8 = value[8].serialize();
        let value9 = value[9].serialize();
        let value = [
            value0[0],
            value0[1],
            value1[0],
            value1[1],
            value2[0],
            value2[1],
            value3[0],
            value3[1],
            value4[0],
            value4[1],
            value5[0],
            value5[1],
            value6[0],
            value6[1],
            value7[0],
            value7[1],
            value8[0],
            value8[1],
            value9[0],
            value9[1],
        ];
        Self(value)
    }
}
impl From<[u32; 5]> for ClientMessageData {
    fn from(value: [u32; 5]) -> Self {
        let value0 = value[0].serialize();
        let value1 = value[1].serialize();
        let value2 = value[2].serialize();
        let value3 = value[3].serialize();
        let value4 = value[4].serialize();
        let value = [
            value0[0],
            value0[1],
            value0[2],
            value0[3],
            value1[0],
            value1[1],
            value1[2],
            value1[3],
            value2[0],
            value2[1],
            value2[2],
            value2[3],
            value3[0],
            value3[1],
            value3[2],
            value3[3],
            value4[0],
            value4[1],
            value4[2],
            value4[3],
        ];
        Self(value)
    }
}

/// Opcode for the ClientMessage event
pub const CLIENT_MESSAGE_EVENT: u8 = 33;
/// NOT YET DOCUMENTED.
///
/// This event represents a ClientMessage, sent by another X11 client. An example
/// is a client sending the `_NET_WM_STATE` ClientMessage to the root window
/// to indicate the fullscreen window state, effectively requesting that the window
/// manager puts it into fullscreen mode.
///
/// # Fields
///
/// * `data` - The data itself (20 bytes max).
/// * `format` - Specifies how to interpret `data`. Can be either 8, 16 or 32.
/// * `type` - An atom which indicates how the data should be interpreted by the receiving
/// client.
///
/// # See
///
/// * SendEvent: request
#[derive(Debug, Clone, Copy)]
pub struct ClientMessageEvent {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: Window,
    pub type_: Atom,
    pub data: ClientMessageData,
}
impl ClientMessageEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (window, remaining) = Window::try_parse(remaining)?;
        let (type_, remaining) = Atom::try_parse(remaining)?;
        let (data, remaining) = ClientMessageData::try_parse(remaining)?;
        let result = ClientMessageEvent { response_type, format, sequence, window, type_, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClientMessageEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for ClientMessageEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for ClientMessageEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&ClientMessageEvent> for [u8; 32] {
    fn from(input: &ClientMessageEvent) -> Self {
        let response_type = input.response_type.serialize();
        let format = input.format.serialize();
        let sequence = input.sequence.serialize();
        let window = input.window.serialize();
        let type_ = input.type_.serialize();
        let data = input.data.serialize();
        [
            response_type[0], format[0], sequence[0], sequence[1], window[0], window[1], window[2], window[3],
            type_[0], type_[1], type_[2], type_[3], data[0], data[1], data[2], data[3],
            data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15], data[16], data[17], data[18], data[19]
        ]
    }
}
impl From<ClientMessageEvent> for [u8; 32] {
    fn from(input: ClientMessageEvent) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mapping {
    Modifier = 0,
    Keyboard = 1,
    Pointer = 2,
}
impl From<Mapping> for u8 {
    fn from(input: Mapping) -> Self {
        match input {
            Mapping::Modifier => 0,
            Mapping::Keyboard => 1,
            Mapping::Pointer => 2,
        }
    }
}
impl From<Mapping> for Option<u8> {
    fn from(input: Mapping) -> Self {
        Some(u8::from(input))
    }
}
impl From<Mapping> for u16 {
    fn from(input: Mapping) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Mapping> for Option<u16> {
    fn from(input: Mapping) -> Self {
        Some(u16::from(input))
    }
}
impl From<Mapping> for u32 {
    fn from(input: Mapping) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Mapping> for Option<u32> {
    fn from(input: Mapping) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Mapping {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mapping::Modifier),
            1 => Ok(Mapping::Keyboard),
            2 => Ok(Mapping::Pointer),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Mapping {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Mapping {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the MappingNotify event
pub const MAPPING_NOTIFY_EVENT: u8 = 34;
/// keyboard mapping changed.
///
///
/// # Fields
///
/// * `count` - The number of keycodes altered.
/// * `first_keycode` - The first number in the range of the altered mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappingNotifyEvent {
    pub response_type: u8,
    pub sequence: u16,
    pub request: Mapping,
    pub first_keycode: Keycode,
    pub count: u8,
}
impl MappingNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (request, remaining) = u8::try_parse(remaining)?;
        let (first_keycode, remaining) = Keycode::try_parse(remaining)?;
        let (count, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let request = request.try_into()?;
        let result = MappingNotifyEvent { response_type, sequence, request, first_keycode, count };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for MappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for MappingNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&MappingNotifyEvent> for [u8; 32] {
    fn from(input: &MappingNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let sequence = input.sequence.serialize();
        let request = u8::from(input.request).serialize();
        let first_keycode = input.first_keycode.serialize();
        let count = input.count.serialize();
        [
            response_type[0], 0, sequence[0], sequence[1], request[0], first_keycode[0], count[0], 0,
            /* trailing padding */ 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<MappingNotifyEvent> for [u8; 32] {
    fn from(input: MappingNotifyEvent) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GeGeneric event
pub const GE_GENERIC_EVENT: u8 = 35;
/// generic event (with length).
///
///
/// # Fields
///
/// * `evtype` - The extension-specific event type
/// * `extension` - The major opcode of the extension creating this event
/// * `length` - The amount (in 4-byte units) of data beyond 32 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeGenericEvent {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
}
impl GeGenericEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (extension, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (event_type, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let result = GeGenericEvent { response_type, extension, sequence, length, event_type };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GeGenericEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for GeGenericEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for GeGenericEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}

/// Opcode for the Request error
pub const REQUEST_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl RequestError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = RequestError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for RequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for RequestError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for RequestError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&RequestError> for [u8; 32] {
    fn from(input: &RequestError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<RequestError> for [u8; 32] {
    fn from(input: RequestError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Value error
pub const VALUE_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl ValueError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ValueError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ValueError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ValueError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ValueError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ValueError> for [u8; 32] {
    fn from(input: &ValueError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ValueError> for [u8; 32] {
    fn from(input: ValueError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Window error
pub const WINDOW_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl WindowError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = WindowError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for WindowError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for WindowError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for WindowError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&WindowError> for [u8; 32] {
    fn from(input: &WindowError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<WindowError> for [u8; 32] {
    fn from(input: WindowError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Pixmap error
pub const PIXMAP_ERROR: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixmapError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl PixmapError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = PixmapError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for PixmapError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for PixmapError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for PixmapError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&PixmapError> for [u8; 32] {
    fn from(input: &PixmapError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<PixmapError> for [u8; 32] {
    fn from(input: PixmapError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Atom error
pub const ATOM_ERROR: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl AtomError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = AtomError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AtomError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for AtomError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for AtomError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&AtomError> for [u8; 32] {
    fn from(input: &AtomError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<AtomError> for [u8; 32] {
    fn from(input: AtomError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Cursor error
pub const CURSOR_ERROR: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl CursorError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = CursorError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CursorError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for CursorError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for CursorError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&CursorError> for [u8; 32] {
    fn from(input: &CursorError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<CursorError> for [u8; 32] {
    fn from(input: CursorError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Font error
pub const FONT_ERROR: u8 = 7;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl FontError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = FontError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for FontError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for FontError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for FontError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&FontError> for [u8; 32] {
    fn from(input: &FontError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<FontError> for [u8; 32] {
    fn from(input: FontError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Match error
pub const MATCH_ERROR: u8 = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl MatchError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = MatchError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for MatchError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for MatchError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for MatchError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&MatchError> for [u8; 32] {
    fn from(input: &MatchError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<MatchError> for [u8; 32] {
    fn from(input: MatchError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Drawable error
pub const DRAWABLE_ERROR: u8 = 9;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawableError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl DrawableError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = DrawableError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for DrawableError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for DrawableError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for DrawableError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&DrawableError> for [u8; 32] {
    fn from(input: &DrawableError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<DrawableError> for [u8; 32] {
    fn from(input: DrawableError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Access error
pub const ACCESS_ERROR: u8 = 10;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl AccessError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = AccessError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AccessError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for AccessError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for AccessError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&AccessError> for [u8; 32] {
    fn from(input: &AccessError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<AccessError> for [u8; 32] {
    fn from(input: AccessError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Alloc error
pub const ALLOC_ERROR: u8 = 11;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl AllocError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = AllocError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AllocError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for AllocError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for AllocError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&AllocError> for [u8; 32] {
    fn from(input: &AllocError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<AllocError> for [u8; 32] {
    fn from(input: AllocError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Colormap error
pub const COLORMAP_ERROR: u8 = 12;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColormapError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl ColormapError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ColormapError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ColormapError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ColormapError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ColormapError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ColormapError> for [u8; 32] {
    fn from(input: &ColormapError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ColormapError> for [u8; 32] {
    fn from(input: ColormapError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the GContext error
pub const G_CONTEXT_ERROR: u8 = 13;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GContextError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl GContextError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = GContextError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GContextError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for GContextError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for GContextError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&GContextError> for [u8; 32] {
    fn from(input: &GContextError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<GContextError> for [u8; 32] {
    fn from(input: GContextError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the IDChoice error
pub const ID_CHOICE_ERROR: u8 = 14;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IDChoiceError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl IDChoiceError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = IDChoiceError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for IDChoiceError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for IDChoiceError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for IDChoiceError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&IDChoiceError> for [u8; 32] {
    fn from(input: &IDChoiceError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<IDChoiceError> for [u8; 32] {
    fn from(input: IDChoiceError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Name error
pub const NAME_ERROR: u8 = 15;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl NameError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = NameError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for NameError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for NameError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for NameError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&NameError> for [u8; 32] {
    fn from(input: &NameError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<NameError> for [u8; 32] {
    fn from(input: NameError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Length error
pub const LENGTH_ERROR: u8 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LengthError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl LengthError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = LengthError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LengthError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for LengthError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for LengthError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&LengthError> for [u8; 32] {
    fn from(input: &LengthError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<LengthError> for [u8; 32] {
    fn from(input: LengthError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Implementation error
pub const IMPLEMENTATION_ERROR: u8 = 17;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplementationError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl ImplementationError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_value, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = ImplementationError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ImplementationError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ImplementationError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ImplementationError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ImplementationError> for [u8; 32] {
    fn from(input: &ImplementationError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_value = input.bad_value.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ImplementationError> for [u8; 32] {
    fn from(input: ImplementationError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WindowClass {
    CopyFromParent = 0,
    InputOutput = 1,
    InputOnly = 2,
}
impl From<WindowClass> for u8 {
    fn from(input: WindowClass) -> Self {
        match input {
            WindowClass::CopyFromParent => 0,
            WindowClass::InputOutput => 1,
            WindowClass::InputOnly => 2,
        }
    }
}
impl From<WindowClass> for Option<u8> {
    fn from(input: WindowClass) -> Self {
        Some(u8::from(input))
    }
}
impl From<WindowClass> for u16 {
    fn from(input: WindowClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<WindowClass> for Option<u16> {
    fn from(input: WindowClass) -> Self {
        Some(u16::from(input))
    }
}
impl From<WindowClass> for u32 {
    fn from(input: WindowClass) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<WindowClass> for Option<u32> {
    fn from(input: WindowClass) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for WindowClass {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WindowClass::CopyFromParent),
            1 => Ok(WindowClass::InputOutput),
            2 => Ok(WindowClass::InputOnly),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for WindowClass {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for WindowClass {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `BackPixel` - Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
/// background pixel is used for the background. Range-checking is not performed,
/// the background pixel is truncated to the appropriate number of bits.
/// * `BackPixmap` - Overrides the default background-pixmap. The background pixmap and window must
/// have the same root and same depth. Any size pixmap can be used, although some
/// sizes may be faster than others.
///
/// If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
/// The server may fill the contents with the previous screen contents or with
/// contents of its own choosing.
///
/// If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
/// used, but the window must have the same depth as the parent (or a Match error
/// results).   The parent's background is tracked, and the current version is
/// used each time the window background is required.
/// * `BackingPixel` - The backing-pixel specifies what value to use in planes not covered by
/// backing-planes. The server is free to save only the specified bit planes in the
/// backing-store or save-under and regenerate the remaining planes with the
/// specified pixel value. Any bits beyond the specified depth of the window in
/// these values are simply ignored.
/// * `BackingPlanes` - The backing-planes indicates (with bits set to 1) which bit planes of the
/// window hold dynamic data that must be preserved in backing-stores and during
/// save-unders.
/// * `BackingStore` - A backing-store of `WhenMapped` advises the server that maintaining contents of
/// obscured regions when the window is mapped would be beneficial. A backing-store
/// of `Always` advises the server that maintaining contents even when the window
/// is unmapped would be beneficial. In this case, the server may generate an
/// exposure event when the window is created. A value of `NotUseful` advises the
/// server that maintaining contents is unnecessary, although a server may still
/// choose to maintain contents while the window is mapped. Note that if the server
/// maintains contents, then the server should maintain complete contents not just
/// the region within the parent boundaries, even if the window is larger than its
/// parent. While the server maintains contents, exposure events will not normally
/// be generated, but the server may stop maintaining contents at any time.
/// * `BitGravity` - Defines which region of the window should be retained if the window is resized.
/// * `BorderPixel` - Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
/// border pixel is used for the border. Range checking is not performed on the
/// border-pixel value, it is truncated to the appropriate number of bits.
/// * `BorderPixmap` - Overrides the default border-pixmap. The border pixmap and window must have the
/// same root and the same depth. Any size pixmap can be used, although some sizes
/// may be faster than others.
///
/// The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
/// copied (subsequent changes to the parent's border attribute do not affect the
/// child), but the window must have the same depth as the parent.
/// * `Colormap` - The colormap specifies the colormap that best reflects the true colors of the window. Servers
/// capable of supporting multiple hardware colormaps may use this information, and window man-
/// agers may use it for InstallColormap requests. The colormap must have the same visual type
/// and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
/// colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
/// However, the window must have the same visual type as the parent (or a Match error results),
/// and the parent must not have a colormap of None (or a Match error results). For an explanation
/// of None, see FreeColormap request. The colormap is copied by sharing the colormap object
/// between the child and the parent, not by making a complete copy of the colormap contents.
/// * `Cursor` - If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
/// fied, the parent's cursor will be used when the pointer is in the window, and any change in the
/// parent's cursor will cause an immediate change in the displayed cursor.
/// * `DontPropagate` - The do-not-propagate-mask defines which events should not be propagated to
/// ancestor windows when no client has the event type selected in this window.
/// * `EventMask` - The event-mask defines which events the client is interested in for this window
/// (or for some event types, inferiors of the window).
/// * `OverrideRedirect` - The override-redirect specifies whether map and configure requests on this
/// window should override a SubstructureRedirect on the parent, typically to
/// inform a window manager not to tamper with the window.
/// * `SaveUnder` - If 1, the server is advised that when this window is mapped, saving the
/// contents of windows it obscures would be beneficial.
/// * `WinGravity` - Defines how the window should be repositioned if the parent is resized (see
/// `ConfigureWindow`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CW {
    BackPixmap = 1 << 0,
    BackPixel = 1 << 1,
    BorderPixmap = 1 << 2,
    BorderPixel = 1 << 3,
    BitGravity = 1 << 4,
    WinGravity = 1 << 5,
    BackingStore = 1 << 6,
    BackingPlanes = 1 << 7,
    BackingPixel = 1 << 8,
    OverrideRedirect = 1 << 9,
    SaveUnder = 1 << 10,
    EventMask = 1 << 11,
    DontPropagate = 1 << 12,
    Colormap = 1 << 13,
    Cursor = 1 << 14,
}
impl From<CW> for u16 {
    fn from(input: CW) -> Self {
        match input {
            CW::BackPixmap => 1 << 0,
            CW::BackPixel => 1 << 1,
            CW::BorderPixmap => 1 << 2,
            CW::BorderPixel => 1 << 3,
            CW::BitGravity => 1 << 4,
            CW::WinGravity => 1 << 5,
            CW::BackingStore => 1 << 6,
            CW::BackingPlanes => 1 << 7,
            CW::BackingPixel => 1 << 8,
            CW::OverrideRedirect => 1 << 9,
            CW::SaveUnder => 1 << 10,
            CW::EventMask => 1 << 11,
            CW::DontPropagate => 1 << 12,
            CW::Colormap => 1 << 13,
            CW::Cursor => 1 << 14,
        }
    }
}
impl From<CW> for Option<u16> {
    fn from(input: CW) -> Self {
        Some(u16::from(input))
    }
}
impl From<CW> for u32 {
    fn from(input: CW) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<CW> for Option<u32> {
    fn from(input: CW) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for CW {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CW::BackPixmap),
            2 => Ok(CW::BackPixel),
            4 => Ok(CW::BorderPixmap),
            8 => Ok(CW::BorderPixel),
            16 => Ok(CW::BitGravity),
            32 => Ok(CW::WinGravity),
            64 => Ok(CW::BackingStore),
            128 => Ok(CW::BackingPlanes),
            256 => Ok(CW::BackingPixel),
            512 => Ok(CW::OverrideRedirect),
            1024 => Ok(CW::SaveUnder),
            2048 => Ok(CW::EventMask),
            4096 => Ok(CW::DontPropagate),
            8192 => Ok(CW::Colormap),
            16384 => Ok(CW::Cursor),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u32> for CW {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(CW, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BackPixmap {
    None = 0,
    ParentRelative = 1,
}
impl From<BackPixmap> for u8 {
    fn from(input: BackPixmap) -> Self {
        match input {
            BackPixmap::None => 0,
            BackPixmap::ParentRelative => 1,
        }
    }
}
impl From<BackPixmap> for Option<u8> {
    fn from(input: BackPixmap) -> Self {
        Some(u8::from(input))
    }
}
impl From<BackPixmap> for u16 {
    fn from(input: BackPixmap) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackPixmap> for Option<u16> {
    fn from(input: BackPixmap) -> Self {
        Some(u16::from(input))
    }
}
impl From<BackPixmap> for u32 {
    fn from(input: BackPixmap) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackPixmap> for Option<u32> {
    fn from(input: BackPixmap) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BackPixmap {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BackPixmap::None),
            1 => Ok(BackPixmap::ParentRelative),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for BackPixmap {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for BackPixmap {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gravity {
    BitForget,
    WinUnmap,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}
impl From<Gravity> for u8 {
    fn from(input: Gravity) -> Self {
        match input {
            Gravity::BitForget => 0,
            Gravity::WinUnmap => 0,
            Gravity::NorthWest => 1,
            Gravity::North => 2,
            Gravity::NorthEast => 3,
            Gravity::West => 4,
            Gravity::Center => 5,
            Gravity::East => 6,
            Gravity::SouthWest => 7,
            Gravity::South => 8,
            Gravity::SouthEast => 9,
            Gravity::Static => 10,
        }
    }
}
impl From<Gravity> for Option<u8> {
    fn from(input: Gravity) -> Self {
        Some(u8::from(input))
    }
}
impl From<Gravity> for u16 {
    fn from(input: Gravity) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Gravity> for Option<u16> {
    fn from(input: Gravity) -> Self {
        Some(u16::from(input))
    }
}
impl From<Gravity> for u32 {
    fn from(input: Gravity) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Gravity> for Option<u32> {
    fn from(input: Gravity) -> Self {
        Some(u32::from(input))
    }
}

/// Opcode for the CreateWindow request
pub const CREATE_WINDOW_REQUEST: u8 = 1;
/// Auxiliary and optional information for the create_window function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateWindowAux {
    pub background_pixmap: Option<Pixmap>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<Pixmap>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<u32>,
    pub win_gravity: Option<u32>,
    pub backing_store: Option<u32>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<Bool32>,
    pub save_under: Option<Bool32>,
    pub event_mask: Option<u32>,
    pub do_not_propogate_mask: Option<u32>,
    pub colormap: Option<Colormap>,
    pub cursor: Option<Cursor>,
}
impl CreateWindowAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.background_pixmap.is_some() {
            mask |= u32::from(CW::BackPixmap);
        }
        if self.background_pixel.is_some() {
            mask |= u32::from(CW::BackPixel);
        }
        if self.border_pixmap.is_some() {
            mask |= u32::from(CW::BorderPixmap);
        }
        if self.border_pixel.is_some() {
            mask |= u32::from(CW::BorderPixel);
        }
        if self.bit_gravity.is_some() {
            mask |= u32::from(CW::BitGravity);
        }
        if self.win_gravity.is_some() {
            mask |= u32::from(CW::WinGravity);
        }
        if self.backing_store.is_some() {
            mask |= u32::from(CW::BackingStore);
        }
        if self.backing_planes.is_some() {
            mask |= u32::from(CW::BackingPlanes);
        }
        if self.backing_pixel.is_some() {
            mask |= u32::from(CW::BackingPixel);
        }
        if self.override_redirect.is_some() {
            mask |= u32::from(CW::OverrideRedirect);
        }
        if self.save_under.is_some() {
            mask |= u32::from(CW::SaveUnder);
        }
        if self.event_mask.is_some() {
            mask |= u32::from(CW::EventMask);
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= u32::from(CW::DontPropagate);
        }
        if self.colormap.is_some() {
            mask |= u32::from(CW::Colormap);
        }
        if self.cursor.is_some() {
            mask |= u32::from(CW::Cursor);
        }
        mask
    }
    /// Set the background_pixmap field of this structure.
    pub fn background_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.background_pixmap = value.into();
        self
    }
    /// Set the background_pixel field of this structure.
    pub fn background_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background_pixel = value.into();
        self
    }
    /// Set the border_pixmap field of this structure.
    pub fn border_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.border_pixmap = value.into();
        self
    }
    /// Set the border_pixel field of this structure.
    pub fn border_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_pixel = value.into();
        self
    }
    /// Set the bit_gravity field of this structure.
    pub fn bit_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.bit_gravity = value.into();
        self
    }
    /// Set the win_gravity field of this structure.
    pub fn win_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.win_gravity = value.into();
        self
    }
    /// Set the backing_store field of this structure.
    pub fn backing_store<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_store = value.into();
        self
    }
    /// Set the backing_planes field of this structure.
    pub fn backing_planes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_planes = value.into();
        self
    }
    /// Set the backing_pixel field of this structure.
    pub fn backing_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_pixel = value.into();
        self
    }
    /// Set the override_redirect field of this structure.
    pub fn override_redirect<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.override_redirect = value.into();
        self
    }
    /// Set the save_under field of this structure.
    pub fn save_under<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.save_under = value.into();
        self
    }
    /// Set the event_mask field of this structure.
    pub fn event_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.event_mask = value.into();
        self
    }
    /// Set the do_not_propogate_mask field of this structure.
    pub fn do_not_propogate_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.do_not_propogate_mask = value.into();
        self
    }
    /// Set the colormap field of this structure.
    pub fn colormap<I>(mut self, value: I) -> Self where I: Into<Option<Colormap>> {
        self.colormap = value.into();
        self
    }
    /// Set the cursor field of this structure.
    pub fn cursor<I>(mut self, value: I) -> Self where I: Into<Option<Cursor>> {
        self.cursor = value.into();
        self
    }
}
impl Serialize for CreateWindowAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.background_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.background_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bit_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.win_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_store {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_planes {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.override_redirect {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.save_under {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.event_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.do_not_propogate_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.colormap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.cursor {
            value.serialize_into(bytes);
        }
    }
}
/// Creates a window.
///
/// Creates an unmapped window as child of the specified `parent` window. A
/// CreateNotify event will be generated. The new window is placed on top in the
/// stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical with
/// the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
/// of pixels, and coincide with pixel centers. Each window and pixmap has its own
/// coordinate system. For a window, the origin is inside the border at the inside,
/// upper-left corner.
///
/// The created window is not yet displayed (mapped), call `xcb_map_window` to
/// display it.
///
/// The created window will initially use the same cursor as its parent.
///
/// # Fields
///
/// * `border_width` - TODO:
///
/// Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
/// * `depth` - Specifies the new window's depth (TODO: what unit?).
///
/// The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
/// `parent` window.
/// * `height` - The height of the new window.
/// * `parent` - The parent window of the new window.
/// * `visual` - Specifies the id for the new window's visual.
///
/// The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
/// `parent` window.
/// * `wid` - The ID with which you will refer to the new window, created by
/// `xcb_generate_id`.
/// * `width` - The width of the new window.
/// * `x` - The X coordinate of the new window.
/// * `y` - The Y coordinate of the new window.
///
/// # Errors
///
/// * `Alloc` - The X server could not allocate the requested resources (no memory?).
/// * `Colormap` - TODO: reasons?
/// * `Cursor` - TODO: reasons?
/// * `Match` - TODO: reasons?
/// * `Pixmap` - TODO: reasons?
/// * `Value` - TODO: reasons?
/// * `Window` - TODO: reasons?
///
/// # See
///
/// * CreateNotify: event
/// * MapWindow: request
/// * xcb_generate_id: function
pub fn create_window<'c, Conn, A>(conn: &'c Conn, depth: u8, wid: Window, parent: Window, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: A, visual: Visualid, value_list: &CreateWindowAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u16>
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (32 + value_list_bytes.len() + 3) / 4;
    let depth_bytes = depth.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let wid_bytes = wid.serialize();
    let parent_bytes = parent.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let border_width_bytes = border_width.serialize();
    let class = class.into();
    let class_bytes = class.serialize();
    let visual_bytes = visual.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CREATE_WINDOW_REQUEST,
        depth_bytes[0],
        length_bytes[0],
        length_bytes[1],
        wid_bytes[0],
        wid_bytes[1],
        wid_bytes[2],
        wid_bytes[3],
        parent_bytes[0],
        parent_bytes[1],
        parent_bytes[2],
        parent_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        border_width_bytes[0],
        border_width_bytes[1],
        class_bytes[0],
        class_bytes[1],
        visual_bytes[0],
        visual_bytes[1],
        visual_bytes[2],
        visual_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeWindowAttributes request
pub const CHANGE_WINDOW_ATTRIBUTES_REQUEST: u8 = 2;
/// Auxiliary and optional information for the change_window_attributes function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeWindowAttributesAux {
    pub background_pixmap: Option<Pixmap>,
    pub background_pixel: Option<u32>,
    pub border_pixmap: Option<Pixmap>,
    pub border_pixel: Option<u32>,
    pub bit_gravity: Option<u32>,
    pub win_gravity: Option<u32>,
    pub backing_store: Option<u32>,
    pub backing_planes: Option<u32>,
    pub backing_pixel: Option<u32>,
    pub override_redirect: Option<Bool32>,
    pub save_under: Option<Bool32>,
    pub event_mask: Option<u32>,
    pub do_not_propogate_mask: Option<u32>,
    pub colormap: Option<Colormap>,
    pub cursor: Option<Cursor>,
}
impl ChangeWindowAttributesAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.background_pixmap.is_some() {
            mask |= u32::from(CW::BackPixmap);
        }
        if self.background_pixel.is_some() {
            mask |= u32::from(CW::BackPixel);
        }
        if self.border_pixmap.is_some() {
            mask |= u32::from(CW::BorderPixmap);
        }
        if self.border_pixel.is_some() {
            mask |= u32::from(CW::BorderPixel);
        }
        if self.bit_gravity.is_some() {
            mask |= u32::from(CW::BitGravity);
        }
        if self.win_gravity.is_some() {
            mask |= u32::from(CW::WinGravity);
        }
        if self.backing_store.is_some() {
            mask |= u32::from(CW::BackingStore);
        }
        if self.backing_planes.is_some() {
            mask |= u32::from(CW::BackingPlanes);
        }
        if self.backing_pixel.is_some() {
            mask |= u32::from(CW::BackingPixel);
        }
        if self.override_redirect.is_some() {
            mask |= u32::from(CW::OverrideRedirect);
        }
        if self.save_under.is_some() {
            mask |= u32::from(CW::SaveUnder);
        }
        if self.event_mask.is_some() {
            mask |= u32::from(CW::EventMask);
        }
        if self.do_not_propogate_mask.is_some() {
            mask |= u32::from(CW::DontPropagate);
        }
        if self.colormap.is_some() {
            mask |= u32::from(CW::Colormap);
        }
        if self.cursor.is_some() {
            mask |= u32::from(CW::Cursor);
        }
        mask
    }
    /// Set the background_pixmap field of this structure.
    pub fn background_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.background_pixmap = value.into();
        self
    }
    /// Set the background_pixel field of this structure.
    pub fn background_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background_pixel = value.into();
        self
    }
    /// Set the border_pixmap field of this structure.
    pub fn border_pixmap<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.border_pixmap = value.into();
        self
    }
    /// Set the border_pixel field of this structure.
    pub fn border_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_pixel = value.into();
        self
    }
    /// Set the bit_gravity field of this structure.
    pub fn bit_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.bit_gravity = value.into();
        self
    }
    /// Set the win_gravity field of this structure.
    pub fn win_gravity<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.win_gravity = value.into();
        self
    }
    /// Set the backing_store field of this structure.
    pub fn backing_store<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_store = value.into();
        self
    }
    /// Set the backing_planes field of this structure.
    pub fn backing_planes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_planes = value.into();
        self
    }
    /// Set the backing_pixel field of this structure.
    pub fn backing_pixel<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.backing_pixel = value.into();
        self
    }
    /// Set the override_redirect field of this structure.
    pub fn override_redirect<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.override_redirect = value.into();
        self
    }
    /// Set the save_under field of this structure.
    pub fn save_under<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.save_under = value.into();
        self
    }
    /// Set the event_mask field of this structure.
    pub fn event_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.event_mask = value.into();
        self
    }
    /// Set the do_not_propogate_mask field of this structure.
    pub fn do_not_propogate_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.do_not_propogate_mask = value.into();
        self
    }
    /// Set the colormap field of this structure.
    pub fn colormap<I>(mut self, value: I) -> Self where I: Into<Option<Colormap>> {
        self.colormap = value.into();
        self
    }
    /// Set the cursor field of this structure.
    pub fn cursor<I>(mut self, value: I) -> Self where I: Into<Option<Cursor>> {
        self.cursor = value.into();
        self
    }
}
impl Serialize for ChangeWindowAttributesAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.background_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.background_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixmap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bit_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.win_gravity {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_store {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_planes {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.backing_pixel {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.override_redirect {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.save_under {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.event_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.do_not_propogate_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.colormap {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.cursor {
            value.serialize_into(bytes);
        }
    }
}
/// change window attributes.
///
/// Changes the attributes specified by `value_mask` for the specified `window`.
///
/// # Fields
///
/// * `value_list` - Values for each of the attributes specified in the bitmask `value_mask`. The
/// order has to correspond to the order of possible `value_mask` bits. See the
/// example.
/// * `window` - The window to change.
///
/// # Errors
///
/// * `Access` - TODO: reasons?
/// * `Colormap` - TODO: reasons?
/// * `Cursor` - TODO: reasons?
/// * `Match` - TODO: reasons?
/// * `Pixmap` - TODO: reasons?
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
pub fn change_window_attributes<'c, Conn>(conn: &'c Conn, window: Window, value_list: &ChangeWindowAttributesAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (12 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CHANGE_WINDOW_ATTRIBUTES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MapState {
    Unmapped = 0,
    Unviewable = 1,
    Viewable = 2,
}
impl From<MapState> for u8 {
    fn from(input: MapState) -> Self {
        match input {
            MapState::Unmapped => 0,
            MapState::Unviewable => 1,
            MapState::Viewable => 2,
        }
    }
}
impl From<MapState> for Option<u8> {
    fn from(input: MapState) -> Self {
        Some(u8::from(input))
    }
}
impl From<MapState> for u16 {
    fn from(input: MapState) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapState> for Option<u16> {
    fn from(input: MapState) -> Self {
        Some(u16::from(input))
    }
}
impl From<MapState> for u32 {
    fn from(input: MapState) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapState> for Option<u32> {
    fn from(input: MapState) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for MapState {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MapState::Unmapped),
            1 => Ok(MapState::Unviewable),
            2 => Ok(MapState::Viewable),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for MapState {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for MapState {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GetWindowAttributes request
pub const GET_WINDOW_ATTRIBUTES_REQUEST: u8 = 3;
/// Gets window attributes.
///
/// Gets the current attributes for the specified `window`.
///
/// # Fields
///
/// * `window` - The window to get the attributes from.
///
/// # Errors
///
/// * `Drawable` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
pub fn get_window_attributes<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, GetWindowAttributesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        GET_WINDOW_ATTRIBUTES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `all_event_masks` - Set of events all people have interest in.
/// * `backing_pixel` - Value to be used when restoring planes.
/// * `backing_planes` - Planes to be preserved if possible.
/// * `colormap` - Color map to be associated with window.
/// * `do_not_propagate_mask` - Set of events that should not propagate.
/// * `override_redirect` - Window managers should ignore this window if `override_redirect` is 1.
/// * `save_under` - Boolean, should bits under be saved?
/// * `visual` - The associated visual structure of `window`.
/// * `your_event_mask` - My event mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWindowAttributesReply {
    pub response_type: u8,
    pub backing_store: BackingStore,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub class: WindowClass,
    pub bit_gravity: u8,
    pub win_gravity: u8,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: bool,
    pub map_is_installed: bool,
    pub map_state: MapState,
    pub override_redirect: bool,
    pub colormap: Colormap,
    pub all_event_masks: u32,
    pub your_event_mask: u32,
    pub do_not_propagate_mask: u16,
}
impl GetWindowAttributesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (backing_store, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (visual, remaining) = Visualid::try_parse(remaining)?;
        let (class, remaining) = u16::try_parse(remaining)?;
        let (bit_gravity, remaining) = u8::try_parse(remaining)?;
        let (win_gravity, remaining) = u8::try_parse(remaining)?;
        let (backing_planes, remaining) = u32::try_parse(remaining)?;
        let (backing_pixel, remaining) = u32::try_parse(remaining)?;
        let (save_under, remaining) = bool::try_parse(remaining)?;
        let (map_is_installed, remaining) = bool::try_parse(remaining)?;
        let (map_state, remaining) = u8::try_parse(remaining)?;
        let (override_redirect, remaining) = bool::try_parse(remaining)?;
        let (colormap, remaining) = Colormap::try_parse(remaining)?;
        let (all_event_masks, remaining) = u32::try_parse(remaining)?;
        let (your_event_mask, remaining) = u32::try_parse(remaining)?;
        let (do_not_propagate_mask, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let backing_store = backing_store.try_into()?;
        let class = class.try_into()?;
        let map_state = map_state.try_into()?;
        let result = GetWindowAttributesReply { response_type, backing_store, sequence, length, visual, class, bit_gravity, win_gravity, backing_planes, backing_pixel, save_under, map_is_installed, map_state, override_redirect, colormap, all_event_masks, your_event_mask, do_not_propagate_mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowAttributesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the DestroyWindow request
pub const DESTROY_WINDOW_REQUEST: u8 = 4;
/// Destroys a window.
///
/// Destroys the specified window and all of its subwindows. A DestroyNotify event
/// is generated for each destroyed window (a DestroyNotify event is first generated
/// for any given window's inferiors). If the window was mapped, it will be
/// automatically unmapped before destroying.
///
/// Calling DestroyWindow on the root window will do nothing.
///
/// # Fields
///
/// * `window` - The window to destroy.
///
/// # Errors
///
/// * `Window` - The specified window does not exist.
///
/// # See
///
/// * DestroyNotify: event
/// * MapWindow: request
/// * UnmapWindow: request
pub fn destroy_window<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        DESTROY_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DestroySubwindows request
pub const DESTROY_SUBWINDOWS_REQUEST: u8 = 5;
pub fn destroy_subwindows<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        DESTROY_SUBWINDOWS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetMode {
    Insert = 0,
    Delete = 1,
}
impl From<SetMode> for u8 {
    fn from(input: SetMode) -> Self {
        match input {
            SetMode::Insert => 0,
            SetMode::Delete => 1,
        }
    }
}
impl From<SetMode> for Option<u8> {
    fn from(input: SetMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<SetMode> for u16 {
    fn from(input: SetMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetMode> for Option<u16> {
    fn from(input: SetMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<SetMode> for u32 {
    fn from(input: SetMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SetMode> for Option<u32> {
    fn from(input: SetMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SetMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SetMode::Insert),
            1 => Ok(SetMode::Delete),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SetMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SetMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeSaveSet request
pub const CHANGE_SAVE_SET_REQUEST: u8 = 6;
/// Changes a client's save set.
///
/// TODO: explain what the save set is for.
///
/// This function either adds or removes the specified window to the client's (your
/// application's) save set.
///
/// # Fields
///
/// * `mode` - Insert to add the specified window to the save set or Delete to delete it from the save set.
/// * `window` - The window to add or delete to/from your save set.
///
/// # Errors
///
/// * `Match` - You created the specified window. This does not make sense, you can only add
///windows created by other clients to your save set.
/// * `Value` - You specified an invalid mode.
/// * `Window` - The specified window does not exist.
///
/// # See
///
/// * ReparentWindow: request
pub fn change_save_set<Conn, A>(conn: &Conn, mode: A, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (8) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        CHANGE_SAVE_SET_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ReparentWindow request
pub const REPARENT_WINDOW_REQUEST: u8 = 7;
/// Reparents a window.
///
/// Makes the specified window a child of the specified parent window. If the
/// window is mapped, it will automatically be unmapped before reparenting and
/// re-mapped after reparenting. The window is placed in the stacking order on top
/// with respect to sibling windows.
///
/// After reparenting, a ReparentNotify event is generated.
///
/// # Fields
///
/// * `parent` - The new parent of the window.
/// * `window` - The window to reparent.
/// * `x` - The X position of the window within its new parent.
/// * `y` - The Y position of the window within its new parent.
///
/// # Errors
///
/// * `Match` - The new parent window is not on the same screen as the old parent window.
///
///The new parent window is the specified window or an inferior of the specified window.
///
///The new parent is InputOnly and the window is not.
///
///The specified window has a ParentRelative background and the new parent window is not the same depth as the specified window.
/// * `Window` - The specified window does not exist.
///
/// # See
///
/// * MapWindow: request
/// * ReparentNotify: event
/// * UnmapWindow: request
pub fn reparent_window<Conn>(conn: &Conn, window: Window, parent: Window, x: i16, y: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let parent_bytes = parent.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        REPARENT_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        parent_bytes[0],
        parent_bytes[1],
        parent_bytes[2],
        parent_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the MapWindow request
pub const MAP_WINDOW_REQUEST: u8 = 8;
/// Makes a window visible.
///
/// Maps the specified window. This means making the window visible (as long as its
/// parent is visible).
///
/// This MapWindow request will be translated to a MapRequest request if a window
/// manager is running. The window manager then decides to either map the window or
/// not. Set the override-redirect window attribute to true if you want to bypass
/// this mechanism.
///
/// If the window manager decides to map the window (or if no window manager is
/// running), a MapNotify event is generated.
///
/// If the window becomes viewable and no earlier contents for it are remembered,
/// the X server tiles the window with its background. If the window's background
/// is undefined, the existing screen contents are not altered, and the X server
/// generates zero or more Expose events.
///
/// If the window type is InputOutput, an Expose event will be generated when the
/// window becomes visible. The normal response to an Expose event should be to
/// repaint the window.
///
/// # Fields
///
/// * `window` - The window to make visible.
///
/// # Errors
///
/// * `Match` - The specified window does not exist.
///
/// # See
///
/// * Expose: event
/// * MapNotify: event
/// * UnmapWindow: request
pub fn map_window<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        MAP_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the MapSubwindows request
pub const MAP_SUBWINDOWS_REQUEST: u8 = 9;
pub fn map_subwindows<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        MAP_SUBWINDOWS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UnmapWindow request
pub const UNMAP_WINDOW_REQUEST: u8 = 10;
/// Makes a window invisible.
///
/// Unmaps the specified window. This means making the window invisible (and all
/// its child windows).
///
/// Unmapping a window leads to the `UnmapNotify` event being generated. Also,
/// `Expose` events are generated for formerly obscured windows.
///
/// # Fields
///
/// * `window` - The window to make invisible.
///
/// # Errors
///
/// * `Window` - The specified window does not exist.
///
/// # See
///
/// * Expose: event
/// * MapWindow: request
/// * UnmapNotify: event
pub fn unmap_window<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        UNMAP_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UnmapSubwindows request
pub const UNMAP_SUBWINDOWS_REQUEST: u8 = 11;
pub fn unmap_subwindows<Conn>(conn: &Conn, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        UNMAP_SUBWINDOWS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConfigWindow {
    X = 1 << 0,
    Y = 1 << 1,
    Width = 1 << 2,
    Height = 1 << 3,
    BorderWidth = 1 << 4,
    Sibling = 1 << 5,
    StackMode = 1 << 6,
}
impl From<ConfigWindow> for u8 {
    fn from(input: ConfigWindow) -> Self {
        match input {
            ConfigWindow::X => 1 << 0,
            ConfigWindow::Y => 1 << 1,
            ConfigWindow::Width => 1 << 2,
            ConfigWindow::Height => 1 << 3,
            ConfigWindow::BorderWidth => 1 << 4,
            ConfigWindow::Sibling => 1 << 5,
            ConfigWindow::StackMode => 1 << 6,
        }
    }
}
impl From<ConfigWindow> for Option<u8> {
    fn from(input: ConfigWindow) -> Self {
        Some(u8::from(input))
    }
}
impl From<ConfigWindow> for u16 {
    fn from(input: ConfigWindow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ConfigWindow> for Option<u16> {
    fn from(input: ConfigWindow) -> Self {
        Some(u16::from(input))
    }
}
impl From<ConfigWindow> for u32 {
    fn from(input: ConfigWindow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ConfigWindow> for Option<u32> {
    fn from(input: ConfigWindow) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ConfigWindow::X),
            2 => Ok(ConfigWindow::Y),
            4 => Ok(ConfigWindow::Width),
            8 => Ok(ConfigWindow::Height),
            16 => Ok(ConfigWindow::BorderWidth),
            32 => Ok(ConfigWindow::Sibling),
            64 => Ok(ConfigWindow::StackMode),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ConfigWindow, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackMode {
    Above = 0,
    Below = 1,
    TopIf = 2,
    BottomIf = 3,
    Opposite = 4,
}
impl From<StackMode> for u8 {
    fn from(input: StackMode) -> Self {
        match input {
            StackMode::Above => 0,
            StackMode::Below => 1,
            StackMode::TopIf => 2,
            StackMode::BottomIf => 3,
            StackMode::Opposite => 4,
        }
    }
}
impl From<StackMode> for Option<u8> {
    fn from(input: StackMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<StackMode> for u16 {
    fn from(input: StackMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<StackMode> for Option<u16> {
    fn from(input: StackMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<StackMode> for u32 {
    fn from(input: StackMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<StackMode> for Option<u32> {
    fn from(input: StackMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for StackMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StackMode::Above),
            1 => Ok(StackMode::Below),
            2 => Ok(StackMode::TopIf),
            3 => Ok(StackMode::BottomIf),
            4 => Ok(StackMode::Opposite),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for StackMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for StackMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ConfigureWindow request
pub const CONFIGURE_WINDOW_REQUEST: u8 = 12;
/// Auxiliary and optional information for the configure_window function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConfigureWindowAux {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub border_width: Option<u32>,
    pub sibling: Option<Window>,
    pub stack_mode: Option<u32>,
}
impl ConfigureWindowAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u16 {
        let mut mask = 0;
        if self.x.is_some() {
            mask |= u16::from(ConfigWindow::X);
        }
        if self.y.is_some() {
            mask |= u16::from(ConfigWindow::Y);
        }
        if self.width.is_some() {
            mask |= u16::from(ConfigWindow::Width);
        }
        if self.height.is_some() {
            mask |= u16::from(ConfigWindow::Height);
        }
        if self.border_width.is_some() {
            mask |= u16::from(ConfigWindow::BorderWidth);
        }
        if self.sibling.is_some() {
            mask |= u16::from(ConfigWindow::Sibling);
        }
        if self.stack_mode.is_some() {
            mask |= u16::from(ConfigWindow::StackMode);
        }
        mask
    }
    /// Set the x field of this structure.
    pub fn x<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.x = value.into();
        self
    }
    /// Set the y field of this structure.
    pub fn y<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.y = value.into();
        self
    }
    /// Set the width field of this structure.
    pub fn width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.width = value.into();
        self
    }
    /// Set the height field of this structure.
    pub fn height<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.height = value.into();
        self
    }
    /// Set the border_width field of this structure.
    pub fn border_width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_width = value.into();
        self
    }
    /// Set the sibling field of this structure.
    pub fn sibling<I>(mut self, value: I) -> Self where I: Into<Option<Window>> {
        self.sibling = value.into();
        self
    }
    /// Set the stack_mode field of this structure.
    pub fn stack_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.stack_mode = value.into();
        self
    }
}
impl Serialize for ConfigureWindowAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.x {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.y {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.width {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.height {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.border_width {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.sibling {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.stack_mode {
            value.serialize_into(bytes);
        }
    }
}
/// Configures window attributes.
///
/// Configures a window's size, position, border width and stacking order.
///
/// # Fields
///
/// * `value_list` - New values, corresponding to the attributes in value_mask. The order has to
/// correspond to the order of possible `value_mask` bits. See the example.
/// * `value_mask` - Bitmask of attributes to change.
/// * `window` - The window to configure.
///
/// # Errors
///
/// * `Match` - You specified a Sibling without also specifying StackMode or the window is not
///actually a Sibling.
/// * `Value` - TODO: reasons?
/// * `Window` - The specified window does not exist. TODO: any other reason?
///
/// # See
///
/// * Expose: event
/// * MapNotify: event
///
/// # Example
///
/// ```text
/// /*
///  * Configures the given window to the left upper corner
///  * with a size of 1024x768 pixels.
///  *
///  */
/// void my_example(xcb_connection_t *c, xcb_window_t window) {
///     uint16_t mask = 0;
///
///     mask |= XCB_CONFIG_WINDOW_X;
///     mask |= XCB_CONFIG_WINDOW_Y;
///     mask |= XCB_CONFIG_WINDOW_WIDTH;
///     mask |= XCB_CONFIG_WINDOW_HEIGHT;
///
///     const uint32_t values[] = {
///         0,    /* x */
///         0,    /* y */
///         1024, /* width */
///         768   /* height */
///     };
///
///     xcb_configure_window(c, window, mask, values);
///     xcb_flush(c);
/// }
/// ```
pub fn configure_window<'c, Conn>(conn: &'c Conn, window: Window, value_list: &ConfigureWindowAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (12 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CONFIGURE_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Circulate {
    RaiseLowest = 0,
    LowerHighest = 1,
}
impl From<Circulate> for u8 {
    fn from(input: Circulate) -> Self {
        match input {
            Circulate::RaiseLowest => 0,
            Circulate::LowerHighest => 1,
        }
    }
}
impl From<Circulate> for Option<u8> {
    fn from(input: Circulate) -> Self {
        Some(u8::from(input))
    }
}
impl From<Circulate> for u16 {
    fn from(input: Circulate) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Circulate> for Option<u16> {
    fn from(input: Circulate) -> Self {
        Some(u16::from(input))
    }
}
impl From<Circulate> for u32 {
    fn from(input: Circulate) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Circulate> for Option<u32> {
    fn from(input: Circulate) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Circulate {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Circulate::RaiseLowest),
            1 => Ok(Circulate::LowerHighest),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Circulate {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Circulate {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CirculateWindow request
pub const CIRCULATE_WINDOW_REQUEST: u8 = 13;
/// Change window stacking order.
///
/// If `direction` is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
/// any) will be raised to the top of the stack.
///
/// If `direction` is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
/// be lowered to the bottom of the stack.
///
/// # Fields
///
/// * `window` - The window to raise/lower (depending on `direction`).
///
/// # Errors
///
/// * `Value` - The specified `direction` is invalid.
/// * `Window` - The specified `window` does not exist.
pub fn circulate_window<Conn, A>(conn: &Conn, direction: A, window: Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (8) / 4;
    let direction = direction.into();
    let direction_bytes = direction.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        CIRCULATE_WINDOW_REQUEST,
        direction_bytes[0],
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetGeometry request
pub const GET_GEOMETRY_REQUEST: u8 = 14;
/// Get current window geometry.
///
/// Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
///
/// # Fields
///
/// * `drawable` - The drawable (`Window` or `Pixmap`) of which the geometry will be received.
///
/// # Errors
///
/// * `Drawable` - TODO: reasons?
/// * `Window` - TODO: reasons?
///
/// # See
///
/// * xwininfo: program
///
/// # Example
///
/// ```text
/// /*
///  * Displays the x and y position of the given window.
///  *
///  */
/// void my_example(xcb_connection_t *c, xcb_window_t window) {
///     xcb_get_geometry_cookie_t cookie;
///     xcb_get_geometry_reply_t *reply;
///
///     cookie = xcb_get_geometry(c, window);
///     /* ... do other work here if possible ... */
///     if ((reply = xcb_get_geometry_reply(c, cookie, NULL))) {
///         printf("This window is at %d, %d\\n", reply->x, reply->y);
///     }
///     free(reply);
/// }
/// ```
pub fn get_geometry<Conn>(conn: &Conn, drawable: Drawable) -> Result<Cookie<'_, Conn, GetGeometryReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let request0 = [
        GET_GEOMETRY_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `border_width` - The border width (in pixels).
/// * `depth` - The depth of the drawable (bits per pixel for the object).
/// * `height` - The height of `drawable`.
/// * `root` - Root window of the screen containing `drawable`.
/// * `width` - The width of `drawable`.
/// * `x` - The X coordinate of `drawable`. If `drawable` is a window, the coordinate
/// specifies the upper-left outer corner relative to its parent's origin. If
/// `drawable` is a pixmap, the X coordinate is always 0.
/// * `y` - The Y coordinate of `drawable`. If `drawable` is a window, the coordinate
/// specifies the upper-left outer corner relative to its parent's origin. If
/// `drawable` is a pixmap, the Y coordinate is always 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGeometryReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}
impl GetGeometryReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (border_width, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = GetGeometryReply { response_type, depth, sequence, length, root, x, y, width, height, border_width };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetGeometryReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryTree request
pub const QUERY_TREE_REQUEST: u8 = 15;
/// query the window tree.
///
/// Gets the root window ID, parent window ID and list of children windows for the
/// specified `window`. The children are listed in bottom-to-top stacking order.
///
/// # Fields
///
/// * `window` - The `window` to query.
///
/// # See
///
/// * xwininfo: program
///
/// # Example
///
/// ```text
/// /*
///  * Displays the root, parent and children of the specified window.
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_window_t window) {
///     xcb_query_tree_cookie_t cookie;
///     xcb_query_tree_reply_t *reply;
///
///     cookie = xcb_query_tree(conn, window);
///     if ((reply = xcb_query_tree_reply(conn, cookie, NULL))) {
///         printf("root = 0x%08x\\n", reply->root);
///         printf("parent = 0x%08x\\n", reply->parent);
///
///         xcb_window_t *children = xcb_query_tree_children(reply);
///         for (int i = 0; i < xcb_query_tree_children_length(reply); i++)
///             printf("child window = 0x%08x\\n", children[i]);
///
///         free(reply);
///     }
/// }
/// ```
pub fn query_tree<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, QueryTreeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        QUERY_TREE_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `children_len` - The number of child windows.
/// * `parent` - The parent window of `window`.
/// * `root` - The root window of `window`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryTreeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub parent: Window,
    pub children: Vec<Window>,
}
impl QueryTreeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (parent, remaining) = Window::try_parse(remaining)?;
        let (children_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(14..).ok_or(ParseError::ParseError)?;
        let (children, remaining) = crate::x11_utils::parse_list::<Window>(remaining, children_len as usize)?;
        let result = QueryTreeReply { response_type, sequence, length, root, parent, children };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryTreeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the InternAtom request
pub const INTERN_ATOM_REQUEST: u8 = 16;
/// Get atom identifier by name.
///
/// Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
/// name. Atoms are used in protocols like EWMH, for example to store window titles
/// (`_NET_WM_NAME` atom) as property of a window.
///
/// If `only_if_exists` is 0, the atom will be created if it does not already exist.
/// If `only_if_exists` is 1, `XCB_ATOM_NONE` will be returned if the atom does
/// not yet exist.
///
/// # Fields
///
/// * `name` - The name of the atom.
/// * `name_len` - The length of the following `name`.
/// * `only_if_exists` - Return a valid atom id only if the atom already exists.
///
/// # Errors
///
/// * `Alloc` - TODO: reasons?
/// * `Value` - A value other than 0 or 1 was specified for `only_if_exists`.
///
/// # See
///
/// * GetAtomName: request
/// * xlsatoms: program
///
/// # Example
///
/// ```text
/// /*
///  * Resolves the _NET_WM_NAME atom.
///  *
///  */
/// void my_example(xcb_connection_t *c) {
///     xcb_intern_atom_cookie_t cookie;
///     xcb_intern_atom_reply_t *reply;
///
///     cookie = xcb_intern_atom(c, 0, strlen("_NET_WM_NAME"), "_NET_WM_NAME");
///     /* ... do other work here if possible ... */
///     if ((reply = xcb_intern_atom_reply(c, cookie, NULL))) {
///         printf("The _NET_WM_NAME atom has ID %u\n", reply->atom);
///         free(reply);
///     }
/// }
/// ```
pub fn intern_atom<'c, Conn>(conn: &'c Conn, only_if_exists: bool, name: &[u8]) -> Result<Cookie<'c, Conn, InternAtomReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 1 * name.len() + 3) / 4;
    let only_if_exists_bytes = (only_if_exists as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        INTERN_ATOM_REQUEST,
        only_if_exists_bytes[0],
        length_bytes[0],
        length_bytes[1],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternAtomReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: Atom,
}
impl InternAtomReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (atom, remaining) = Atom::try_parse(remaining)?;
        let result = InternAtomReply { response_type, sequence, length, atom };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InternAtomReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetAtomName request
pub const GET_ATOM_NAME_REQUEST: u8 = 17;
pub fn get_atom_name<Conn>(conn: &Conn, atom: Atom) -> Result<Cookie<'_, Conn, GetAtomNameReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let atom_bytes = atom.serialize();
    let request0 = [
        GET_ATOM_NAME_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        atom_bytes[0],
        atom_bytes[1],
        atom_bytes[2],
        atom_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAtomNameReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub name: Vec<u8>,
}
impl GetAtomNameReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (name_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        let result = GetAtomNameReply { response_type, sequence, length, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetAtomNameReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `Append` - Insert the new data after the beginning of existing data. The `format` must
/// match existing property value. If the property is undefined, it is treated as
/// defined with the correct type and format with zero-length data.
/// * `Prepend` - Insert the new data before the beginning of existing data. The `format` must
/// match existing property value. If the property is undefined, it is treated as
/// defined with the correct type and format with zero-length data.
/// * `Replace` - Discard the previous property value and store the new data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PropMode {
    Replace = 0,
    Prepend = 1,
    Append = 2,
}
impl From<PropMode> for u8 {
    fn from(input: PropMode) -> Self {
        match input {
            PropMode::Replace => 0,
            PropMode::Prepend => 1,
            PropMode::Append => 2,
        }
    }
}
impl From<PropMode> for Option<u8> {
    fn from(input: PropMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<PropMode> for u16 {
    fn from(input: PropMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropMode> for Option<u16> {
    fn from(input: PropMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<PropMode> for u32 {
    fn from(input: PropMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PropMode> for Option<u32> {
    fn from(input: PropMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PropMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PropMode::Replace),
            1 => Ok(PropMode::Prepend),
            2 => Ok(PropMode::Append),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PropMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PropMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeProperty request
pub const CHANGE_PROPERTY_REQUEST: u8 = 18;
/// Changes a window property.
///
/// Sets or updates a property on the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// # Fields
///
/// * `data` - The property data.
/// * `data_len` - Specifies the number of elements (see `format`).
/// * `format` - Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
/// 32-bit quantities. Possible values are 8, 16 and 32. This information allows
/// the X server to correctly perform byte-swap operations as necessary.
/// * `property` - The property you want to change (an atom).
/// * `type` - The type of the property you want to change (an atom).
/// * `window` - The window whose property you want to change.
///
/// # Errors
///
/// * `Alloc` - The X server could not store the property (no memory?).
/// * `Atom` - `property` or `type` do not refer to a valid atom.
/// * `Match` - TODO: reasons?
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
///
/// # See
///
/// * InternAtom: request
/// * xprop: program
///
/// # Example
///
/// ```text
/// /*
///  * Sets the WM_NAME property of the window to "XCB Example".
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_window_t window) {
///     xcb_change_property(conn,
///         XCB_PROP_MODE_REPLACE,
///         window,
///         XCB_ATOM_WM_NAME,
///         XCB_ATOM_STRING,
///         8,
///         strlen("XCB Example"),
///         "XCB Example");
///     xcb_flush(conn);
/// }
/// ```
pub fn change_property<'c, Conn, A, B, C>(conn: &'c Conn, mode: A, window: Window, property: B, type_: C, format: u8, data_len: u32, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<Atom>, C: Into<Atom>
{
    let length: usize = (24 + 1 * data.len() + 3) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let property = property.into();
    let property_bytes = property.serialize();
    let type_ = type_.into();
    let type_bytes = type_.serialize();
    let format_bytes = format.serialize();
    let data_len_bytes = data_len.serialize();
    assert_eq!(data.len(), ((data_len as usize) * (format as usize)) / (8), "Argument data has an incorrect length");
    let request0 = [
        CHANGE_PROPERTY_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        format_bytes[0],
        0,
        0,
        0,
        data_len_bytes[0],
        data_len_bytes[1],
        data_len_bytes[2],
        data_len_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteProperty request
pub const DELETE_PROPERTY_REQUEST: u8 = 19;
pub fn delete_property<Conn>(conn: &Conn, window: Window, property: Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let property_bytes = property.serialize();
    let request0 = [
        DELETE_PROPERTY_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GetPropertyType {
    Any = 0,
}
impl From<GetPropertyType> for u8 {
    fn from(input: GetPropertyType) -> Self {
        match input {
            GetPropertyType::Any => 0,
        }
    }
}
impl From<GetPropertyType> for Option<u8> {
    fn from(input: GetPropertyType) -> Self {
        Some(u8::from(input))
    }
}
impl From<GetPropertyType> for u16 {
    fn from(input: GetPropertyType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GetPropertyType> for Option<u16> {
    fn from(input: GetPropertyType) -> Self {
        Some(u16::from(input))
    }
}
impl From<GetPropertyType> for u32 {
    fn from(input: GetPropertyType) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GetPropertyType> for Option<u32> {
    fn from(input: GetPropertyType) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GetPropertyType {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GetPropertyType::Any),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GetPropertyType {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GetPropertyType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GetProperty request
pub const GET_PROPERTY_REQUEST: u8 = 20;
impl GetPropertyReply {
    /// Iterate over the contained value if its format is 8.
    ///
    /// This function checks if the `format` member of the reply
    /// is 8. If it it is not, `None` is returned.  Otherwise
    /// and iterator is returned that interprets the value in
    /// this reply as type `u8`.
    ///
    /// # Examples
    ///
    /// Successfully iterate over the value:
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 8,
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 2, 3, 4],
    /// };
    ///
    /// // This is the actual example: Iterate over the value.
    /// let mut iter = reply.value8().unwrap();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(4));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// An iterator is only returned when the `format` is correct.
    /// The following example shows this.
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 42, // Not allowed in X11, but used for the example
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 2, 3, 4],
    /// };
    /// assert!(reply.value8().is_none());
    /// ```
    #[allow(single_use_lifetimes, private_doc_tests)] // Work around a rustc bug and rustdoc bug
    pub fn value8<'a>(&'a self) -> Option<impl Iterator<Item=u8> + 'a> {
        if self.format == 8 {
            Some(crate::wrapper::PropertyIterator::new(&self.value))
        } else {
            None
        }
    }
    /// Iterate over the contained value if its format is 16.
    ///
    /// This function checks if the `format` member of the reply
    /// is 16. If it it is not, `None` is returned.  Otherwise
    /// and iterator is returned that interprets the value in
    /// this reply as type `u16`.
    ///
    /// # Examples
    ///
    /// Successfully iterate over the value:
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 16,
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 1, 2, 2],
    /// };
    ///
    /// // This is the actual example: Iterate over the value.
    /// let mut iter = reply.value16().unwrap();
    /// assert_eq!(iter.next(), Some(257));
    /// assert_eq!(iter.next(), Some(514));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// An iterator is only returned when the `format` is correct.
    /// The following example shows this.
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 42, // Not allowed in X11, but used for the example
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 2, 3, 4],
    /// };
    /// assert!(reply.value16().is_none());
    /// ```
    #[allow(single_use_lifetimes, private_doc_tests)] // Work around a rustc bug and rustdoc bug
    pub fn value16<'a>(&'a self) -> Option<impl Iterator<Item=u16> + 'a> {
        if self.format == 16 {
            Some(crate::wrapper::PropertyIterator::new(&self.value))
        } else {
            None
        }
    }
    /// Iterate over the contained value if its format is 32.
    ///
    /// This function checks if the `format` member of the reply
    /// is 32. If it it is not, `None` is returned.  Otherwise
    /// and iterator is returned that interprets the value in
    /// this reply as type `u32`.
    ///
    /// # Examples
    ///
    /// Successfully iterate over the value:
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 32,
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 2, 2, 1],
    /// };
    ///
    /// // This is the actual example: Iterate over the value.
    /// let mut iter = reply.value32().unwrap();
    /// assert_eq!(iter.next(), Some(16908801));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// An iterator is only returned when the `format` is correct.
    /// The following example shows this.
    /// ```
    /// // First, we have to 'invent' a GetPropertyReply.
    /// let reply = x11rb::xproto::GetPropertyReply {
    ///     response_type: 1,
    ///     format: 42, // Not allowed in X11, but used for the example
    ///     sequence: 0,
    ///     length: 0, // This value is incorrect
    ///     type_: 0, // This value is incorrect
    ///     bytes_after: 0,
    ///     value_len: 4,
    ///     value: vec![1, 2, 3, 4],
    /// };
    /// assert!(reply.value32().is_none());
    /// ```
    #[allow(single_use_lifetimes, private_doc_tests)] // Work around a rustc bug and rustdoc bug
    pub fn value32<'a>(&'a self) -> Option<impl Iterator<Item=u32> + 'a> {
        if self.format == 32 {
            Some(crate::wrapper::PropertyIterator::new(&self.value))
        } else {
            None
        }
    }
}
/// Gets a window property.
///
/// Gets the specified `property` from the specified `window`. Properties are for
/// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
/// Protocols such as EWMH also use properties - for example EWMH defines the
/// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
///
/// TODO: talk about `type`
///
/// TODO: talk about `delete`
///
/// TODO: talk about the offset/length thing. what's a valid use case?
///
/// # Fields
///
/// * `delete` - Whether the property should actually be deleted. For deleting a property, the
/// specified `type` has to match the actual property type.
/// * `long_length` - Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
/// set `long_length` to 4, you will receive 16 bytes of data).
/// * `long_offset` - Specifies the offset (in 32-bit multiples) in the specified property where the
/// data is to be retrieved.
/// * `property` - The property you want to get (an atom).
/// * `type` - The type of the property you want to get (an atom).
/// * `window` - The window whose property you want to get.
///
/// # Errors
///
/// * `Atom` - `property` or `type` do not refer to a valid atom.
/// * `Value` - The specified `long_offset` is beyond the actual property length (e.g. the
///property has a length of 3 bytes and you are setting `long_offset` to 1,
///resulting in a byte offset of 4).
/// * `Window` - The specified `window` does not exist.
///
/// # See
///
/// * InternAtom: request
/// * xprop: program
///
/// # Example
///
/// ```text
/// /*
///  * Prints the WM_NAME property of the window.
///  *
///  */
/// void my_example(xcb_connection_t *c, xcb_window_t window) {
///     xcb_get_property_cookie_t cookie;
///     xcb_get_property_reply_t *reply;
///
///     /* These atoms are predefined in the X11 protocol. */
///     xcb_atom_t property = XCB_ATOM_WM_NAME;
///     xcb_atom_t type = XCB_ATOM_STRING;
///
///     // TODO: a reasonable long_length for WM_NAME?
///     cookie = xcb_get_property(c, 0, window, property, type, 0, 0);
///     if ((reply = xcb_get_property_reply(c, cookie, NULL))) {
///         int len = xcb_get_property_value_length(reply);
///         if (len == 0) {
///             printf("TODO\\n");
///             free(reply);
///             return;
///         }
///         printf("WM_NAME is %.*s\\n", len,
///                (char*)xcb_get_property_value(reply));
///     }
///     free(reply);
/// }
/// ```
pub fn get_property<Conn>(conn: &Conn, delete: bool, window: Window, property: Atom, type_: Atom, long_offset: u32, long_length: u32) -> Result<Cookie<'_, Conn, GetPropertyReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (24) / 4;
    let delete_bytes = (delete as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let property_bytes = property.serialize();
    let type_bytes = type_.serialize();
    let long_offset_bytes = long_offset.serialize();
    let long_length_bytes = long_length.serialize();
    let request0 = [
        GET_PROPERTY_REQUEST,
        delete_bytes[0],
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        type_bytes[0],
        type_bytes[1],
        type_bytes[2],
        type_bytes[3],
        long_offset_bytes[0],
        long_offset_bytes[1],
        long_offset_bytes[2],
        long_offset_bytes[3],
        long_length_bytes[0],
        long_length_bytes[1],
        long_length_bytes[2],
        long_length_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `bytes_after` - The number of bytes remaining to be read in the property if a partial read was
/// performed.
/// * `format` - Specifies whether the data should be viewed as a list of 8-bit, 16-bit, or
/// 32-bit quantities. Possible values are 8, 16, and 32. This information allows
/// the X server to correctly perform byte-swap operations as necessary.
/// * `type` - The actual type of the property (an atom).
/// * `value_len` - The length of value. You should use the corresponding accessor instead of this
/// field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyReply {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: Atom,
    pub bytes_after: u32,
    pub value_len: u32,
    pub value: Vec<u8>,
}
impl GetPropertyReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (format, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (type_, remaining) = Atom::try_parse(remaining)?;
        let (bytes_after, remaining) = u32::try_parse(remaining)?;
        let (value_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (value, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (value_len as usize) * ((format as usize) / (8)))?;
        let result = GetPropertyReply { response_type, format, sequence, length, type_, bytes_after, value_len, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListProperties request
pub const LIST_PROPERTIES_REQUEST: u8 = 21;
pub fn list_properties<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        LIST_PROPERTIES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<Atom>,
}
impl ListPropertiesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (atoms_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (atoms, remaining) = crate::x11_utils::parse_list::<Atom>(remaining, atoms_len as usize)?;
        let result = ListPropertiesReply { response_type, sequence, length, atoms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetSelectionOwner request
pub const SET_SELECTION_OWNER_REQUEST: u8 = 22;
/// Sets the owner of a selection.
///
/// Makes `window` the owner of the selection `selection` and updates the
/// last-change time of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
/// # Fields
///
/// * `owner` - The new owner of the selection.
///
/// The special value `XCB_NONE` means that the selection will have no owner.
/// * `selection` - The selection.
/// * `time` - Timestamp to avoid race conditions when running X over the network.
///
/// The selection will not be changed if `time` is earlier than the current
/// last-change time of the `selection` or is later than the current X server time.
/// Otherwise, the last-change time is set to the specified time.
///
/// The special value `XCB_CURRENT_TIME` will be replaced with the current server
/// time.
///
/// # Errors
///
/// * `Atom` - `selection` does not refer to a valid atom.
///
/// # See
///
/// * SetSelectionOwner: request
pub fn set_selection_owner<Conn>(conn: &Conn, owner: Window, selection: Atom, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let owner_bytes = owner.serialize();
    let selection_bytes = selection.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        SET_SELECTION_OWNER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        owner_bytes[0],
        owner_bytes[1],
        owner_bytes[2],
        owner_bytes[3],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetSelectionOwner request
pub const GET_SELECTION_OWNER_REQUEST: u8 = 23;
/// Gets the owner of a selection.
///
/// Gets the owner of the specified selection.
///
/// TODO: briefly explain what a selection is.
///
/// # Fields
///
/// * `selection` - The selection.
///
/// # Errors
///
/// * `Atom` - `selection` does not refer to a valid atom.
///
/// # See
///
/// * SetSelectionOwner: request
pub fn get_selection_owner<Conn>(conn: &Conn, selection: Atom) -> Result<Cookie<'_, Conn, GetSelectionOwnerReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let selection_bytes = selection.serialize();
    let request0 = [
        GET_SELECTION_OWNER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `owner` - The current selection owner window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionOwnerReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: Window,
}
impl GetSelectionOwnerReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (owner, remaining) = Window::try_parse(remaining)?;
        let result = GetSelectionOwnerReply { response_type, sequence, length, owner };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionOwnerReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ConvertSelection request
pub const CONVERT_SELECTION_REQUEST: u8 = 24;
pub fn convert_selection<Conn>(conn: &Conn, requestor: Window, selection: Atom, target: Atom, property: Atom, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let requestor_bytes = requestor.serialize();
    let selection_bytes = selection.serialize();
    let target_bytes = target.serialize();
    let property_bytes = property.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        CONVERT_SELECTION_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        requestor_bytes[0],
        requestor_bytes[1],
        requestor_bytes[2],
        requestor_bytes[3],
        selection_bytes[0],
        selection_bytes[1],
        selection_bytes[2],
        selection_bytes[3],
        target_bytes[0],
        target_bytes[1],
        target_bytes[2],
        target_bytes[3],
        property_bytes[0],
        property_bytes[1],
        property_bytes[2],
        property_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SendEventDest {
    PointerWindow = 0,
    ItemFocus = 1,
}
impl From<SendEventDest> for u8 {
    fn from(input: SendEventDest) -> Self {
        match input {
            SendEventDest::PointerWindow => 0,
            SendEventDest::ItemFocus => 1,
        }
    }
}
impl From<SendEventDest> for Option<u8> {
    fn from(input: SendEventDest) -> Self {
        Some(u8::from(input))
    }
}
impl From<SendEventDest> for u16 {
    fn from(input: SendEventDest) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SendEventDest> for Option<u16> {
    fn from(input: SendEventDest) -> Self {
        Some(u16::from(input))
    }
}
impl From<SendEventDest> for u32 {
    fn from(input: SendEventDest) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SendEventDest> for Option<u32> {
    fn from(input: SendEventDest) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SendEventDest {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SendEventDest::PointerWindow),
            1 => Ok(SendEventDest::ItemFocus),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SendEventDest {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SendEventDest {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SendEvent request
pub const SEND_EVENT_REQUEST: u8 = 25;
/// send an event.
///
/// Identifies the `destination` window, determines which clients should receive
/// the specified event and ignores any active grabs.
///
/// The `event` must be one of the core events or an event defined by an extension,
/// so that the X server can correctly byte-swap the contents as necessary. The
/// contents of `event` are otherwise unaltered and unchecked except for the
/// `send_event` field which is forced to 'true'.
///
/// # Fields
///
/// * `destination` - The window to send this event to. Every client which selects any event within
/// `event_mask` on `destination` will get the event.
///
/// The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
/// that contains the mouse pointer.
///
/// The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
/// has the keyboard focus.
/// * `event` - The event to send to the specified `destination`.
/// * `event_mask` - Event_mask for determining which clients should receive the specified event.
/// See `destination` and `propagate`.
/// * `propagate` - If `propagate` is true and no clients have selected any event on `destination`,
/// the destination is replaced with the closest ancestor of `destination` for
/// which some client has selected a type in `event_mask` and for which no
/// intervening window has that type in its do-not-propagate-mask. If no such
/// window exists or if the window is an ancestor of the focus window and
/// `InputFocus` was originally specified as the destination, the event is not sent
/// to any clients. Otherwise, the event is reported to every client selecting on
/// the final destination any of the types specified in `event_mask`.
///
/// # Errors
///
/// * `Value` - The given `event` is neither a core event nor an event defined by an extension.
/// * `Window` - The specified `destination` window does not exist.
///
/// # See
///
/// * ConfigureNotify: event
///
/// # Example
///
/// ```text
/// /*
///  * Tell the given window that it was configured to a size of 800x600 pixels.
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_window_t window) {
///     /* Every X11 event is 32 bytes long. Therefore, XCB will copy 32 bytes.
///      * In order to properly initialize these bytes, we allocate 32 bytes even
///      * though we only need less for an xcb_configure_notify_event_t */
///     xcb_configure_notify_event_t *event = calloc(32, 1);
///
///     event->event = window;
///     event->window = window;
///     event->response_type = XCB_CONFIGURE_NOTIFY;
///
///     event->x = 0;
///     event->y = 0;
///     event->width = 800;
///     event->height = 600;
///
///     event->border_width = 0;
///     event->above_sibling = XCB_NONE;
///     event->override_redirect = false;
///
///     xcb_send_event(conn, false, window, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
///                    (char*)event);
///     xcb_flush(conn);
///     free(event);
/// }
/// ```
pub fn send_event<Conn, A>(conn: &Conn, propagate: bool, destination: Window, event_mask: u32, event: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<[u8; 32]>
{
    let length: usize = (44) / 4;
    let propagate_bytes = (propagate as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let destination_bytes = destination.serialize();
    let event_mask_bytes = event_mask.serialize();
    let event = event.into();
    let event = &event;
    let request0 = [
        SEND_EVENT_REQUEST,
        propagate_bytes[0],
        length_bytes[0],
        length_bytes[1],
        destination_bytes[0],
        destination_bytes[1],
        destination_bytes[2],
        destination_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        event_mask_bytes[2],
        event_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (event).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(event)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `Async` - Keyboard event processing continues normally.
/// * `Sync` - The state of the keyboard appears to freeze: No further keyboard events are
/// generated by the server until the grabbing client issues a releasing
/// `AllowEvents` request or until the keyboard grab is released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabMode {
    Sync = 0,
    Async = 1,
}
impl From<GrabMode> for u8 {
    fn from(input: GrabMode) -> Self {
        match input {
            GrabMode::Sync => 0,
            GrabMode::Async => 1,
        }
    }
}
impl From<GrabMode> for Option<u8> {
    fn from(input: GrabMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabMode> for u16 {
    fn from(input: GrabMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabMode> for Option<u16> {
    fn from(input: GrabMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabMode> for u32 {
    fn from(input: GrabMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabMode> for Option<u32> {
    fn from(input: GrabMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabMode::Sync),
            1 => Ok(GrabMode::Async),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GrabMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrabStatus {
    Success = 0,
    AlreadyGrabbed = 1,
    InvalidTime = 2,
    NotViewable = 3,
    Frozen = 4,
}
impl From<GrabStatus> for u8 {
    fn from(input: GrabStatus) -> Self {
        match input {
            GrabStatus::Success => 0,
            GrabStatus::AlreadyGrabbed => 1,
            GrabStatus::InvalidTime => 2,
            GrabStatus::NotViewable => 3,
            GrabStatus::Frozen => 4,
        }
    }
}
impl From<GrabStatus> for Option<u8> {
    fn from(input: GrabStatus) -> Self {
        Some(u8::from(input))
    }
}
impl From<GrabStatus> for u16 {
    fn from(input: GrabStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabStatus> for Option<u16> {
    fn from(input: GrabStatus) -> Self {
        Some(u16::from(input))
    }
}
impl From<GrabStatus> for u32 {
    fn from(input: GrabStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GrabStatus> for Option<u32> {
    fn from(input: GrabStatus) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GrabStatus {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GrabStatus::Success),
            1 => Ok(GrabStatus::AlreadyGrabbed),
            2 => Ok(GrabStatus::InvalidTime),
            3 => Ok(GrabStatus::NotViewable),
            4 => Ok(GrabStatus::Frozen),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GrabStatus {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GrabStatus {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CursorEnum {
    None = 0,
}
impl From<CursorEnum> for u8 {
    fn from(input: CursorEnum) -> Self {
        match input {
            CursorEnum::None => 0,
        }
    }
}
impl From<CursorEnum> for Option<u8> {
    fn from(input: CursorEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<CursorEnum> for u16 {
    fn from(input: CursorEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorEnum> for Option<u16> {
    fn from(input: CursorEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<CursorEnum> for u32 {
    fn from(input: CursorEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CursorEnum> for Option<u32> {
    fn from(input: CursorEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CursorEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CursorEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CursorEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CursorEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GrabPointer request
pub const GRAB_POINTER_REQUEST: u8 = 26;
/// Grab the pointer.
///
/// Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
///
/// # Fields
///
/// * `confine_to` - Specifies the window to confine the pointer in (the user will not be able to
/// move the pointer out of that window).
///
/// The special value `XCB_NONE` means don't confine the pointer.
/// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
/// cursor.
/// * `event_mask` - Specifies which pointer events are reported to the client.
///
/// TODO: which values?
/// * `grab_window` - Specifies the window on which the pointer should be grabbed.
/// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
/// reported to the `grab_window`.
/// * `time` - The time argument allows you to avoid certain circumstances that come up if
/// applications take a long time to respond or if there are long network delays.
/// Consider a situation where you have two applications, both of which normally
/// grab the pointer when clicked on. If both applications specify the timestamp
/// from the event, the second application may wake up faster and successfully grab
/// the pointer before the first application. The first application then will get
/// an indication that the other application grabbed the pointer before its request
/// was processed.
///
/// The special value `XCB_CURRENT_TIME` will be replaced with the current server
/// time.
///
/// # Errors
///
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
///
/// # See
///
/// * GrabKeyboard: request
///
/// # Example
///
/// ```text
/// /*
///  * Grabs the pointer actively
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_screen_t *screen, xcb_cursor_t cursor) {
///     xcb_grab_pointer_cookie_t cookie;
///     xcb_grab_pointer_reply_t *reply;
///
///     cookie = xcb_grab_pointer(
///         conn,
///         false,               /* get all pointer events specified by the following mask */
///         screen->root,        /* grab the root window */
///         XCB_NONE,            /* which events to let through */
///         XCB_GRAB_MODE_ASYNC, /* pointer events should continue as normal */
///         XCB_GRAB_MODE_ASYNC, /* keyboard mode */
///         XCB_NONE,            /* confine_to = in which window should the cursor stay */
///         cursor,              /* we change the cursor to whatever the user wanted */
///         XCB_CURRENT_TIME
///     );
///
///     if ((reply = xcb_grab_pointer_reply(conn, cookie, NULL))) {
///         if (reply->status == XCB_GRAB_STATUS_SUCCESS)
///             printf("successfully grabbed the pointer\\n");
///         free(preply);
///     }
/// }
/// ```
pub fn grab_pointer<Conn, A, B>(conn: &Conn, owner_events: bool, grab_window: Window, event_mask: u16, pointer_mode: A, keyboard_mode: B, confine_to: Window, cursor: Cursor, time: Timestamp) -> Result<Cookie<'_, Conn, GrabPointerReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (24) / 4;
    let owner_events_bytes = (owner_events as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let event_mask_bytes = event_mask.serialize();
    let pointer_mode = pointer_mode.into();
    let pointer_mode_bytes = pointer_mode.serialize();
    let keyboard_mode = keyboard_mode.into();
    let keyboard_mode_bytes = keyboard_mode.serialize();
    let confine_to_bytes = confine_to.serialize();
    let cursor_bytes = cursor.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        GRAB_POINTER_REQUEST,
        owner_events_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        pointer_mode_bytes[0],
        keyboard_mode_bytes[0],
        confine_to_bytes[0],
        confine_to_bytes[1],
        confine_to_bytes[2],
        confine_to_bytes[3],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabPointerReply {
    pub response_type: u8,
    pub status: GrabStatus,
    pub sequence: u16,
    pub length: u32,
}
impl GrabPointerReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = GrabPointerReply { response_type, status, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GrabPointerReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the UngrabPointer request
pub const UNGRAB_POINTER_REQUEST: u8 = 27;
/// release the pointer.
///
/// Releases the pointer and any queued events if you actively grabbed the pointer
/// before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
/// press.
///
/// EnterNotify and LeaveNotify events are generated.
///
/// # Fields
///
/// * `name` - A pattern describing an X core font.
/// * `name_len` - Length (in bytes) of `name`.
/// * `time` - Timestamp to avoid race conditions when running X over the network.
///
/// The pointer will not be released if `time` is earlier than the
/// last-pointer-grab time or later than the current X server time.
///
/// # See
///
/// * EnterNotify: event
/// * GrabButton: request
/// * GrabPointer: request
/// * LeaveNotify: event
pub fn ungrab_pointer<Conn>(conn: &Conn, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let request0 = [
        UNGRAB_POINTER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `1` - The left mouse button.
/// * `2` - The right mouse button.
/// * `3` - The middle mouse button.
/// * `4` - Scroll wheel. TODO: direction?
/// * `5` - Scroll wheel. TODO: direction?
/// * `Any` - Any of the following (or none):
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ButtonIndex {
    Any = 0,
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
}
impl From<ButtonIndex> for u8 {
    fn from(input: ButtonIndex) -> Self {
        match input {
            ButtonIndex::Any => 0,
            ButtonIndex::M1 => 1,
            ButtonIndex::M2 => 2,
            ButtonIndex::M3 => 3,
            ButtonIndex::M4 => 4,
            ButtonIndex::M5 => 5,
        }
    }
}
impl From<ButtonIndex> for Option<u8> {
    fn from(input: ButtonIndex) -> Self {
        Some(u8::from(input))
    }
}
impl From<ButtonIndex> for u16 {
    fn from(input: ButtonIndex) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ButtonIndex> for Option<u16> {
    fn from(input: ButtonIndex) -> Self {
        Some(u16::from(input))
    }
}
impl From<ButtonIndex> for u32 {
    fn from(input: ButtonIndex) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ButtonIndex> for Option<u32> {
    fn from(input: ButtonIndex) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ButtonIndex {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ButtonIndex::Any),
            1 => Ok(ButtonIndex::M1),
            2 => Ok(ButtonIndex::M2),
            3 => Ok(ButtonIndex::M3),
            4 => Ok(ButtonIndex::M4),
            5 => Ok(ButtonIndex::M5),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ButtonIndex {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ButtonIndex {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GrabButton request
pub const GRAB_BUTTON_REQUEST: u8 = 28;
/// Grab pointer button(s).
///
/// This request establishes a passive grab. The pointer is actively grabbed as
/// described in GrabPointer, the last-pointer-grab time is set to the time at
/// which the button was pressed (as transmitted in the ButtonPress event), and the
/// ButtonPress event is reported if all of the following conditions are true:
///
/// The pointer is not grabbed and the specified button is logically pressed when
/// the specified modifier keys are logically down, and no other buttons or
/// modifier keys are logically down.
///
/// The grab-window contains the pointer.
///
/// The confine-to window (if any) is viewable.
///
/// A passive grab on the same button/key combination does not exist on any
/// ancestor of grab-window.
///
/// The interpretation of the remaining arguments is the same as for GrabPointer.
/// The active grab is terminated automatically when the logical state of the
/// pointer has all buttons released, independent of the logical state of modifier
/// keys. Note that the logical state of a device (as seen by means of the
/// protocol) may lag the physical state if device event processing is frozen. This
/// request overrides all previous passive grabs by the same client on the same
/// button/key combinations on the same window. A modifier of AnyModifier is
/// equivalent to issuing the request for all possible modifier combinations
/// (including the combination of no modifiers). It is not required that all
/// specified modifiers have currently assigned keycodes. A button of AnyButton is
/// equivalent to issuing the request for all possible buttons. Otherwise, it is
/// not required that the button specified currently be assigned to a physical
/// button.
///
/// An Access error is generated if some other client has already issued a
/// GrabButton request with the same button/key combination on the same window.
/// When using AnyModifier or AnyButton, the request fails completely (no grabs are
/// established), and an Access error is generated if there is a conflicting grab
/// for any combination. The request has no effect on an active grab.
///
/// # Fields
///
/// * `confine_to` - Specifies the window to confine the pointer in (the user will not be able to
/// move the pointer out of that window).
///
/// The special value `XCB_NONE` means don't confine the pointer.
/// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
/// cursor.
/// * `event_mask` - Specifies which pointer events are reported to the client.
///
/// TODO: which values?
/// * `grab_window` - Specifies the window on which the pointer should be grabbed.
/// * `modifiers` - The modifiers to grab.
///
/// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
/// possible modifier combinations.
/// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
/// reported to the `grab_window`.
///
/// # Errors
///
/// * `Access` - Another client has already issued a GrabButton with the same button/key
///combination on the same window.
/// * `Cursor` - The specified `cursor` does not exist.
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
pub fn grab_button<Conn, A, B, C>(conn: &Conn, owner_events: bool, grab_window: Window, event_mask: u16, pointer_mode: A, keyboard_mode: B, confine_to: Window, cursor: Cursor, button: C, modifiers: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>, C: Into<u8>
{
    let length: usize = (24) / 4;
    let owner_events_bytes = (owner_events as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let event_mask_bytes = event_mask.serialize();
    let pointer_mode = pointer_mode.into();
    let pointer_mode_bytes = pointer_mode.serialize();
    let keyboard_mode = keyboard_mode.into();
    let keyboard_mode_bytes = keyboard_mode.serialize();
    let confine_to_bytes = confine_to.serialize();
    let cursor_bytes = cursor.serialize();
    let button = button.into();
    let button_bytes = button.serialize();
    let modifiers_bytes = modifiers.serialize();
    let request0 = [
        GRAB_BUTTON_REQUEST,
        owner_events_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        pointer_mode_bytes[0],
        keyboard_mode_bytes[0],
        confine_to_bytes[0],
        confine_to_bytes[1],
        confine_to_bytes[2],
        confine_to_bytes[3],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        button_bytes[0],
        0,
        modifiers_bytes[0],
        modifiers_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UngrabButton request
pub const UNGRAB_BUTTON_REQUEST: u8 = 29;
pub fn ungrab_button<Conn, A>(conn: &Conn, button: A, grab_window: Window, modifiers: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12) / 4;
    let button = button.into();
    let button_bytes = button.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let modifiers_bytes = modifiers.serialize();
    let request0 = [
        UNGRAB_BUTTON_REQUEST,
        button_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        modifiers_bytes[0],
        modifiers_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ChangeActivePointerGrab request
pub const CHANGE_ACTIVE_POINTER_GRAB_REQUEST: u8 = 30;
pub fn change_active_pointer_grab<Conn>(conn: &Conn, cursor: Cursor, time: Timestamp, event_mask: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cursor_bytes = cursor.serialize();
    let time_bytes = time.serialize();
    let event_mask_bytes = event_mask.serialize();
    let request0 = [
        CHANGE_ACTIVE_POINTER_GRAB_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        event_mask_bytes[0],
        event_mask_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GrabKeyboard request
pub const GRAB_KEYBOARD_REQUEST: u8 = 31;
/// Grab the keyboard.
///
/// Actively grabs control of the keyboard and generates FocusIn and FocusOut
/// events. Further key events are reported only to the grabbing client.
///
/// Any active keyboard grab by this client is overridden. If the keyboard is
/// actively grabbed by some other client, `AlreadyGrabbed` is returned. If
/// `grab_window` is not viewable, `GrabNotViewable` is returned. If the keyboard
/// is frozen by an active grab of another client, `GrabFrozen` is returned. If the
/// specified `time` is earlier than the last-keyboard-grab time or later than the
/// current X server time, `GrabInvalidTime` is returned. Otherwise, the
/// last-keyboard-grab time is set to the specified time.
///
/// # Fields
///
/// * `grab_window` - Specifies the window on which the pointer should be grabbed.
/// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
/// reported to the `grab_window`.
/// * `time` - Timestamp to avoid race conditions when running X over the network.
///
/// The special value `XCB_CURRENT_TIME` will be replaced with the current server
/// time.
///
/// # Errors
///
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
///
/// # See
///
/// * GrabPointer: request
///
/// # Example
///
/// ```text
/// /*
///  * Grabs the keyboard actively
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_screen_t *screen) {
///     xcb_grab_keyboard_cookie_t cookie;
///     xcb_grab_keyboard_reply_t *reply;
///
///     cookie = xcb_grab_keyboard(
///         conn,
///         true,                /* report events */
///         screen->root,        /* grab the root window */
///         XCB_CURRENT_TIME,
///         XCB_GRAB_MODE_ASYNC, /* process events as normal, do not require sync */
///         XCB_GRAB_MODE_ASYNC
///     );
///
///     if ((reply = xcb_grab_keyboard_reply(conn, cookie, NULL))) {
///         if (reply->status == XCB_GRAB_STATUS_SUCCESS)
///             printf("successfully grabbed the keyboard\\n");
///
///         free(reply);
///     }
/// }
/// ```
pub fn grab_keyboard<Conn, A, B>(conn: &Conn, owner_events: bool, grab_window: Window, time: Timestamp, pointer_mode: A, keyboard_mode: B) -> Result<Cookie<'_, Conn, GrabKeyboardReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (16) / 4;
    let owner_events_bytes = (owner_events as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let time_bytes = time.serialize();
    let pointer_mode = pointer_mode.into();
    let pointer_mode_bytes = pointer_mode.serialize();
    let keyboard_mode = keyboard_mode.into();
    let keyboard_mode_bytes = keyboard_mode.serialize();
    let request0 = [
        GRAB_KEYBOARD_REQUEST,
        owner_events_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
        pointer_mode_bytes[0],
        keyboard_mode_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabKeyboardReply {
    pub response_type: u8,
    pub status: GrabStatus,
    pub sequence: u16,
    pub length: u32,
}
impl GrabKeyboardReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = GrabKeyboardReply { response_type, status, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GrabKeyboardReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the UngrabKeyboard request
pub const UNGRAB_KEYBOARD_REQUEST: u8 = 32;
pub fn ungrab_keyboard<Conn>(conn: &Conn, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let request0 = [
        UNGRAB_KEYBOARD_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Grab {
    Any = 0,
}
impl From<Grab> for u8 {
    fn from(input: Grab) -> Self {
        match input {
            Grab::Any => 0,
        }
    }
}
impl From<Grab> for Option<u8> {
    fn from(input: Grab) -> Self {
        Some(u8::from(input))
    }
}
impl From<Grab> for u16 {
    fn from(input: Grab) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Grab> for Option<u16> {
    fn from(input: Grab) -> Self {
        Some(u16::from(input))
    }
}
impl From<Grab> for u32 {
    fn from(input: Grab) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Grab> for Option<u32> {
    fn from(input: Grab) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Grab {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Grab::Any),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Grab {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Grab {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the GrabKey request
pub const GRAB_KEY_REQUEST: u8 = 33;
/// Grab keyboard key(s).
///
/// Establishes a passive grab on the keyboard. In the future, the keyboard is
/// actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
/// the time at which the key was pressed (as transmitted in the KeyPress event),
/// and the KeyPress event is reported if all of the following conditions are true:
///
/// The keyboard is not grabbed and the specified key (which can itself be a
/// modifier key) is logically pressed when the specified modifier keys are
/// logically down, and no other modifier keys are logically down.
///
/// Either the grab_window is an ancestor of (or is) the focus window, or the
/// grab_window is a descendant of the focus window and contains the pointer.
///
/// A passive grab on the same key combination does not exist on any ancestor of
/// grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
/// automatically when the logical state of the keyboard has the specified key released (independent of the
/// logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
///
/// Note that the logical state of a device (as seen by client applications) may lag the physical state if
/// device event processing is frozen.
///
/// A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
/// have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
/// all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
/// and max_keycode in the connection setup, or a BadValue error results.
///
/// If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
/// error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
/// results (no grabs are established) if there is a conflicting grab for any combination.
///
/// # Fields
///
/// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
/// cursor.
/// * `grab_window` - Specifies the window on which the pointer should be grabbed.
/// * `key` - The keycode of the key to grab.
///
/// The special value `XCB_GRAB_ANY` means grab any key.
/// * `modifiers` - The modifiers to grab.
///
/// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
/// possible modifier combinations.
/// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
/// reported to the `grab_window`.
///
/// # Errors
///
/// * `Access` - Another client has already issued a GrabKey with the same button/key
///combination on the same window.
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `window` does not exist.
///
/// # See
///
/// * GrabKeyboard: request
pub fn grab_key<Conn, A, B>(conn: &Conn, owner_events: bool, grab_window: Window, modifiers: u16, key: Keycode, pointer_mode: A, keyboard_mode: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (16) / 4;
    let owner_events_bytes = (owner_events as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let modifiers_bytes = modifiers.serialize();
    let key_bytes = key.serialize();
    let pointer_mode = pointer_mode.into();
    let pointer_mode_bytes = pointer_mode.serialize();
    let keyboard_mode = keyboard_mode.into();
    let keyboard_mode_bytes = keyboard_mode.serialize();
    let request0 = [
        GRAB_KEY_REQUEST,
        owner_events_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        modifiers_bytes[0],
        modifiers_bytes[1],
        key_bytes[0],
        pointer_mode_bytes[0],
        keyboard_mode_bytes[0],
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UngrabKey request
pub const UNGRAB_KEY_REQUEST: u8 = 34;
/// release a key combination.
///
/// Releases the key combination on `grab_window` if you grabbed it using
/// `xcb_grab_key` before.
///
/// # Fields
///
/// * `grab_window` - The window on which the grabbed key combination will be released.
/// * `key` - The keycode of the specified key combination.
///
/// Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
/// * `modifiers` - The modifiers of the specified key combination.
///
/// Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
/// with every possible modifier combination.
///
/// # Errors
///
/// * `Value` - TODO: reasons?
/// * `Window` - The specified `grab_window` does not exist.
///
/// # See
///
/// * GrabKey: request
/// * xev: program
pub fn ungrab_key<Conn>(conn: &Conn, key: Keycode, grab_window: Window, modifiers: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12) / 4;
    let key_bytes = key.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let grab_window_bytes = grab_window.serialize();
    let modifiers_bytes = modifiers.serialize();
    let request0 = [
        UNGRAB_KEY_REQUEST,
        key_bytes[0],
        length_bytes[0],
        length_bytes[1],
        grab_window_bytes[0],
        grab_window_bytes[1],
        grab_window_bytes[2],
        grab_window_bytes[3],
        modifiers_bytes[0],
        modifiers_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `AsyncBoth` - For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
/// processing for both devices continues normally. If a device is frozen twice by
/// the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
/// has no effect unless both pointer and keyboard are frozen by the client.
/// * `AsyncKeyboard` - For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
/// processing continues normally. If the keyboard is frozen twice by the client on
/// behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
/// no effect if the keyboard is not frozen by the client, but the keyboard need
/// not be grabbed by the client.
/// * `AsyncPointer` - For AsyncPointer, if the pointer is frozen by the client, pointer event
/// processing continues normally. If the pointer is frozen twice by the client on
/// behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
/// effect if the pointer is not frozen by the client, but the pointer need not be
/// grabbed by the client.
///
/// TODO: rewrite this in more understandable terms.
/// * `ReplayKeyboard` - For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
/// frozen as the result of an event having been sent to the client (either from
/// the activation of a GrabKey or from a previous AllowEvents with mode
/// SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
/// and that event is completely reprocessed, this time ignoring any passive grabs
/// at or above (towards the root) the grab-window of the grab just released. The
/// request has no effect if the keyboard is not grabbed by the client or if the
/// keyboard is not frozen as the result of an event.
/// * `ReplayPointer` - For ReplayPointer, if the pointer is actively grabbed by the client and is
/// frozen as the result of an event having been sent to the client (either from
/// the activation of a GrabButton or from a previous AllowEvents with mode
/// SyncPointer but not from a GrabPointer), then the pointer grab is released and
/// that event is completely reprocessed, this time ignoring any passive grabs at
/// or above (towards the root) the grab-window of the grab just released. The
/// request has no effect if the pointer is not grabbed by the client or if the
/// pointer is not frozen as the result of an event.
/// * `SyncBoth` - For SyncBoth, if both pointer and keyboard are frozen by the client, event
/// processing (for both devices) continues normally until the next ButtonPress,
/// ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
/// grabbed device (button event for the pointer, key event for the keyboard), at
/// which time the devices again appear to freeze. However, if the reported event
/// causes the grab to be released, then the devices do not freeze (but if the
/// other device is still grabbed, then a subsequent event for it will still cause
/// both devices to freeze). SyncBoth has no effect unless both pointer and
/// keyboard are frozen by the client. If the pointer or keyboard is frozen twice
/// by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
/// subsequent freeze for SyncBoth will only freeze each device once).
/// * `SyncKeyboard` - For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
/// keyboard event processing continues normally until the next KeyPress or
/// KeyRelease event is reported to the client, at which time the keyboard again
/// appears to freeze. However, if the reported event causes the keyboard grab to
/// be released, then the keyboard does not freeze. SyncKeyboard has no effect if
/// the keyboard is not frozen by the client or if the keyboard is not grabbed by
/// the client.
/// * `SyncPointer` - For SyncPointer, if the pointer is frozen and actively grabbed by the client,
/// pointer event processing continues normally until the next ButtonPress or
/// ButtonRelease event is reported to the client, at which time the pointer again
/// appears to freeze. However, if the reported event causes the pointer grab to be
/// released, then the pointer does not freeze. SyncPointer has no effect if the
/// pointer is not frozen by the client or if the pointer is not grabbed by the
/// client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Allow {
    AsyncPointer = 0,
    SyncPointer = 1,
    ReplayPointer = 2,
    AsyncKeyboard = 3,
    SyncKeyboard = 4,
    ReplayKeyboard = 5,
    AsyncBoth = 6,
    SyncBoth = 7,
}
impl From<Allow> for u8 {
    fn from(input: Allow) -> Self {
        match input {
            Allow::AsyncPointer => 0,
            Allow::SyncPointer => 1,
            Allow::ReplayPointer => 2,
            Allow::AsyncKeyboard => 3,
            Allow::SyncKeyboard => 4,
            Allow::ReplayKeyboard => 5,
            Allow::AsyncBoth => 6,
            Allow::SyncBoth => 7,
        }
    }
}
impl From<Allow> for Option<u8> {
    fn from(input: Allow) -> Self {
        Some(u8::from(input))
    }
}
impl From<Allow> for u16 {
    fn from(input: Allow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Allow> for Option<u16> {
    fn from(input: Allow) -> Self {
        Some(u16::from(input))
    }
}
impl From<Allow> for u32 {
    fn from(input: Allow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Allow> for Option<u32> {
    fn from(input: Allow) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Allow {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Allow::AsyncPointer),
            1 => Ok(Allow::SyncPointer),
            2 => Ok(Allow::ReplayPointer),
            3 => Ok(Allow::AsyncKeyboard),
            4 => Ok(Allow::SyncKeyboard),
            5 => Ok(Allow::ReplayKeyboard),
            6 => Ok(Allow::AsyncBoth),
            7 => Ok(Allow::SyncBoth),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Allow {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Allow {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the AllowEvents request
pub const ALLOW_EVENTS_REQUEST: u8 = 35;
/// release queued events.
///
/// Releases queued events if the client has caused a device (pointer/keyboard) to
/// freeze due to grabbing it actively. This request has no effect if `time` is
/// earlier than the last-grab time of the most recent active grab for this client
/// or if `time` is later than the current X server time.
///
/// # Fields
///
/// * `time` - Timestamp to avoid race conditions when running X over the network.
///
/// The special value `XCB_CURRENT_TIME` will be replaced with the current server
/// time.
///
/// # Errors
///
/// * `Value` - You specified an invalid `mode`.
pub fn allow_events<Conn, A>(conn: &Conn, mode: A, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (8) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let time_bytes = time.serialize();
    let request0 = [
        ALLOW_EVENTS_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GrabServer request
pub const GRAB_SERVER_REQUEST: u8 = 36;
pub fn grab_server<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GRAB_SERVER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UngrabServer request
pub const UNGRAB_SERVER_REQUEST: u8 = 37;
pub fn ungrab_server<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        UNGRAB_SERVER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryPointer request
pub const QUERY_POINTER_REQUEST: u8 = 38;
/// get pointer coordinates.
///
/// Gets the root window the pointer is logically on and the pointer coordinates
/// relative to the root window's origin.
///
/// # Fields
///
/// * `window` - A window to check if the pointer is on the same screen as `window` (see the
/// `same_screen` field in the reply).
///
/// # Errors
///
/// * `Window` - The specified `window` does not exist.
pub fn query_pointer<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, QueryPointerReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        QUERY_POINTER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `child` - The child window containing the pointer, if any, if `same_screen` is true. If
/// `same_screen` is false, `XCB_NONE` is returned.
/// * `mask` - The current logical state of the modifier keys and the buttons. Note that the
/// logical state of a device (as seen by means of the protocol) may lag the
/// physical state if device event processing is frozen.
/// * `root` - The root window the pointer is logically on.
/// * `root_x` - The pointer X position, relative to `root`.
/// * `root_y` - The pointer Y position, relative to `root`.
/// * `same_screen` - If `same_screen` is False, then the pointer is not on the same screen as the
/// argument window, `child` is None, and `win_x` and `win_y` are zero. If
/// `same_screen` is True, then `win_x` and `win_y` are the pointer coordinates
/// relative to the argument window's origin, and child is the child containing the
/// pointer, if any.
/// * `win_x` - The pointer X coordinate, relative to `child`, if `same_screen` is true. Zero
/// otherwise.
/// * `win_y` - The pointer Y coordinate, relative to `child`, if `same_screen` is true. Zero
/// otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPointerReply {
    pub response_type: u8,
    pub same_screen: bool,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
}
impl QueryPointerReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (win_x, remaining) = i16::try_parse(remaining)?;
        let (win_y, remaining) = i16::try_parse(remaining)?;
        let (mask, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = QueryPointerReply { response_type, same_screen, sequence, length, root, child, root_x, root_y, win_x, win_y, mask };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryPointerReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timecoord {
    pub time: Timestamp,
    pub x: i16,
    pub y: i16,
}
impl TryParse for Timecoord {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let result = Timecoord { time, x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Timecoord {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Timecoord {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let time_bytes = self.time.serialize();
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        [
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.time.serialize_into(bytes);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
    }
}

/// Opcode for the GetMotionEvents request
pub const GET_MOTION_EVENTS_REQUEST: u8 = 39;
pub fn get_motion_events<Conn>(conn: &Conn, window: Window, start: Timestamp, stop: Timestamp) -> Result<Cookie<'_, Conn, GetMotionEventsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let start_bytes = start.serialize();
    let stop_bytes = stop.serialize();
    let request0 = [
        GET_MOTION_EVENTS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        start_bytes[0],
        start_bytes[1],
        start_bytes[2],
        start_bytes[3],
        stop_bytes[0],
        stop_bytes[1],
        stop_bytes[2],
        stop_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMotionEventsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub events: Vec<Timecoord>,
}
impl GetMotionEventsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (events_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (events, remaining) = crate::x11_utils::parse_list::<Timecoord>(remaining, events_len as usize)?;
        let result = GetMotionEventsReply { response_type, sequence, length, events };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMotionEventsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the TranslateCoordinates request
pub const TRANSLATE_COORDINATES_REQUEST: u8 = 40;
pub fn translate_coordinates<Conn>(conn: &Conn, src_window: Window, dst_window: Window, src_x: i16, src_y: i16) -> Result<Cookie<'_, Conn, TranslateCoordinatesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_window_bytes = src_window.serialize();
    let dst_window_bytes = dst_window.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let request0 = [
        TRANSLATE_COORDINATES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        src_window_bytes[0],
        src_window_bytes[1],
        src_window_bytes[2],
        src_window_bytes[3],
        dst_window_bytes[0],
        dst_window_bytes[1],
        dst_window_bytes[2],
        dst_window_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslateCoordinatesReply {
    pub response_type: u8,
    pub same_screen: bool,
    pub sequence: u16,
    pub length: u32,
    pub child: Window,
    pub dst_x: i16,
    pub dst_y: i16,
}
impl TranslateCoordinatesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (dst_x, remaining) = i16::try_parse(remaining)?;
        let (dst_y, remaining) = i16::try_parse(remaining)?;
        let result = TranslateCoordinatesReply { response_type, same_screen, sequence, length, child, dst_x, dst_y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for TranslateCoordinatesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the WarpPointer request
pub const WARP_POINTER_REQUEST: u8 = 41;
/// move mouse pointer.
///
/// Moves the mouse pointer to the specified position.
///
/// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
/// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
/// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
/// `src_window`.
///
/// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
/// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
/// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
/// relative to the current position of the pointer.
///
/// # Fields
///
/// * `dst_window` - If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
/// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
/// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
/// relative to the current position of the pointer.
/// * `src_window` - If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
/// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
/// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
/// `src_window`.
///
/// # Errors
///
/// * `Window` - TODO: reasons?
///
/// # See
///
/// * SetInputFocus: request
pub fn warp_pointer<Conn>(conn: &Conn, src_window: Window, dst_window: Window, src_x: i16, src_y: i16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (24) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_window_bytes = src_window.serialize();
    let dst_window_bytes = dst_window.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let src_width_bytes = src_width.serialize();
    let src_height_bytes = src_height.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let request0 = [
        WARP_POINTER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        src_window_bytes[0],
        src_window_bytes[1],
        src_window_bytes[2],
        src_window_bytes[3],
        dst_window_bytes[0],
        dst_window_bytes[1],
        dst_window_bytes[2],
        dst_window_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        src_width_bytes[0],
        src_width_bytes[1],
        src_height_bytes[0],
        src_height_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_y_bytes[0],
        dst_y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `FollowKeyboard` - NOT YET DOCUMENTED. Only relevant for the xinput extension.
/// * `None` - The focus reverts to `XCB_NONE`, so no window will have the input focus.
/// * `Parent` - The focus reverts to the parent (or closest viewable ancestor) and the new
/// revert_to value is `XCB_INPUT_FOCUS_NONE`.
/// * `PointerRoot` - The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
/// FocusIn and FocusOut events are generated, but the last-focus-change time is
/// not changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputFocus {
    None = 0,
    PointerRoot = 1,
    Parent = 2,
    FollowKeyboard = 3,
}
impl From<InputFocus> for u8 {
    fn from(input: InputFocus) -> Self {
        match input {
            InputFocus::None => 0,
            InputFocus::PointerRoot => 1,
            InputFocus::Parent => 2,
            InputFocus::FollowKeyboard => 3,
        }
    }
}
impl From<InputFocus> for Option<u8> {
    fn from(input: InputFocus) -> Self {
        Some(u8::from(input))
    }
}
impl From<InputFocus> for u16 {
    fn from(input: InputFocus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<InputFocus> for Option<u16> {
    fn from(input: InputFocus) -> Self {
        Some(u16::from(input))
    }
}
impl From<InputFocus> for u32 {
    fn from(input: InputFocus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<InputFocus> for Option<u32> {
    fn from(input: InputFocus) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for InputFocus {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InputFocus::None),
            1 => Ok(InputFocus::PointerRoot),
            2 => Ok(InputFocus::Parent),
            3 => Ok(InputFocus::FollowKeyboard),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for InputFocus {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for InputFocus {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetInputFocus request
pub const SET_INPUT_FOCUS_REQUEST: u8 = 42;
/// Sets input focus.
///
/// Changes the input focus and the last-focus-change time. If the specified `time`
/// is earlier than the current last-focus-change time, the request is ignored (to
/// avoid race conditions when running X over the network).
///
/// A FocusIn and FocusOut event is generated when focus is changed.
///
/// # Fields
///
/// * `focus` - The window to focus. All keyboard events will be reported to this window. The
/// window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
///
/// If `focus` is `XCB_NONE` (TODO), all keyboard events are
/// discarded until a new focus window is set.
///
/// If `focus` is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
/// screen on which the pointer is on currently.
/// * `revert_to` - Specifies what happens when the `focus` window becomes unviewable (if `focus`
/// is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
/// * `time` - Timestamp to avoid race conditions when running X over the network.
///
/// The special value `XCB_CURRENT_TIME` will be replaced with the current server
/// time.
///
/// # Errors
///
/// * `Match` - The specified `focus` window is not viewable.
/// * `Value` - TODO: Reasons?
/// * `Window` - The specified `focus` window does not exist.
///
/// # See
///
/// * FocusIn: event
/// * FocusOut: event
pub fn set_input_focus<Conn, A>(conn: &Conn, revert_to: A, focus: Window, time: Timestamp) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12) / 4;
    let revert_to = revert_to.into();
    let revert_to_bytes = revert_to.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let focus_bytes = focus.serialize();
    let time_bytes = time.serialize();
    let request0 = [
        SET_INPUT_FOCUS_REQUEST,
        revert_to_bytes[0],
        length_bytes[0],
        length_bytes[1],
        focus_bytes[0],
        focus_bytes[1],
        focus_bytes[2],
        focus_bytes[3],
        time_bytes[0],
        time_bytes[1],
        time_bytes[2],
        time_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetInputFocus request
pub const GET_INPUT_FOCUS_REQUEST: u8 = 43;
pub fn get_input_focus<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetInputFocusReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_INPUT_FOCUS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInputFocusReply {
    pub response_type: u8,
    pub revert_to: InputFocus,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
}
impl GetInputFocusReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (revert_to, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = Window::try_parse(remaining)?;
        let revert_to = revert_to.try_into()?;
        let result = GetInputFocusReply { response_type, revert_to, sequence, length, focus };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetInputFocusReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryKeymap request
pub const QUERY_KEYMAP_REQUEST: u8 = 44;
pub fn query_keymap<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryKeymapReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        QUERY_KEYMAP_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryKeymapReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [u8; 32],
}
impl QueryKeymapReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (keys_0, remaining) = u8::try_parse(remaining)?;
        let (keys_1, remaining) = u8::try_parse(remaining)?;
        let (keys_2, remaining) = u8::try_parse(remaining)?;
        let (keys_3, remaining) = u8::try_parse(remaining)?;
        let (keys_4, remaining) = u8::try_parse(remaining)?;
        let (keys_5, remaining) = u8::try_parse(remaining)?;
        let (keys_6, remaining) = u8::try_parse(remaining)?;
        let (keys_7, remaining) = u8::try_parse(remaining)?;
        let (keys_8, remaining) = u8::try_parse(remaining)?;
        let (keys_9, remaining) = u8::try_parse(remaining)?;
        let (keys_10, remaining) = u8::try_parse(remaining)?;
        let (keys_11, remaining) = u8::try_parse(remaining)?;
        let (keys_12, remaining) = u8::try_parse(remaining)?;
        let (keys_13, remaining) = u8::try_parse(remaining)?;
        let (keys_14, remaining) = u8::try_parse(remaining)?;
        let (keys_15, remaining) = u8::try_parse(remaining)?;
        let (keys_16, remaining) = u8::try_parse(remaining)?;
        let (keys_17, remaining) = u8::try_parse(remaining)?;
        let (keys_18, remaining) = u8::try_parse(remaining)?;
        let (keys_19, remaining) = u8::try_parse(remaining)?;
        let (keys_20, remaining) = u8::try_parse(remaining)?;
        let (keys_21, remaining) = u8::try_parse(remaining)?;
        let (keys_22, remaining) = u8::try_parse(remaining)?;
        let (keys_23, remaining) = u8::try_parse(remaining)?;
        let (keys_24, remaining) = u8::try_parse(remaining)?;
        let (keys_25, remaining) = u8::try_parse(remaining)?;
        let (keys_26, remaining) = u8::try_parse(remaining)?;
        let (keys_27, remaining) = u8::try_parse(remaining)?;
        let (keys_28, remaining) = u8::try_parse(remaining)?;
        let (keys_29, remaining) = u8::try_parse(remaining)?;
        let (keys_30, remaining) = u8::try_parse(remaining)?;
        let (keys_31, remaining) = u8::try_parse(remaining)?;
        let keys = [
            keys_0,
            keys_1,
            keys_2,
            keys_3,
            keys_4,
            keys_5,
            keys_6,
            keys_7,
            keys_8,
            keys_9,
            keys_10,
            keys_11,
            keys_12,
            keys_13,
            keys_14,
            keys_15,
            keys_16,
            keys_17,
            keys_18,
            keys_19,
            keys_20,
            keys_21,
            keys_22,
            keys_23,
            keys_24,
            keys_25,
            keys_26,
            keys_27,
            keys_28,
            keys_29,
            keys_30,
            keys_31,
        ];
        let result = QueryKeymapReply { response_type, sequence, length, keys };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryKeymapReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the OpenFont request
pub const OPEN_FONT_REQUEST: u8 = 45;
/// opens a font.
///
/// Opens any X core font matching the given `name` (for example "-misc-fixed-*").
///
/// Note that X core fonts are deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// # Fields
///
/// * `fid` - The ID with which you will refer to the font, created by `xcb_generate_id`.
/// * `name` - A pattern describing an X core font.
/// * `name_len` - Length (in bytes) of `name`.
///
/// # Errors
///
/// * `Name` - No font matches the given `name`.
///
/// # See
///
/// * xcb_generate_id: function
pub fn open_font<'c, Conn>(conn: &'c Conn, fid: Font, name: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 1 * name.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fid_bytes = fid.serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        OPEN_FONT_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        fid_bytes[0],
        fid_bytes[1],
        fid_bytes[2],
        fid_bytes[3],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CloseFont request
pub const CLOSE_FONT_REQUEST: u8 = 46;
pub fn close_font<Conn>(conn: &Conn, font: Font) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let font_bytes = font.serialize();
    let request0 = [
        CLOSE_FONT_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        font_bytes[0],
        font_bytes[1],
        font_bytes[2],
        font_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FontDraw {
    LeftToRight = 0,
    RightToLeft = 1,
}
impl From<FontDraw> for u8 {
    fn from(input: FontDraw) -> Self {
        match input {
            FontDraw::LeftToRight => 0,
            FontDraw::RightToLeft => 1,
        }
    }
}
impl From<FontDraw> for Option<u8> {
    fn from(input: FontDraw) -> Self {
        Some(u8::from(input))
    }
}
impl From<FontDraw> for u16 {
    fn from(input: FontDraw) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FontDraw> for Option<u16> {
    fn from(input: FontDraw) -> Self {
        Some(u16::from(input))
    }
}
impl From<FontDraw> for u32 {
    fn from(input: FontDraw) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FontDraw> for Option<u32> {
    fn from(input: FontDraw) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for FontDraw {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FontDraw::LeftToRight),
            1 => Ok(FontDraw::RightToLeft),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for FontDraw {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for FontDraw {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fontprop {
    pub name: Atom,
    pub value: u32,
}
impl TryParse for Fontprop {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name, remaining) = Atom::try_parse(remaining)?;
        let (value, remaining) = u32::try_parse(remaining)?;
        let result = Fontprop { name, value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Fontprop {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Fontprop {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let name_bytes = self.name.serialize();
        let value_bytes = self.value.serialize();
        [
            name_bytes[0],
            name_bytes[1],
            name_bytes[2],
            name_bytes[3],
            value_bytes[0],
            value_bytes[1],
            value_bytes[2],
            value_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.name.serialize_into(bytes);
        self.value.serialize_into(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Charinfo {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}
impl TryParse for Charinfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (left_side_bearing, remaining) = i16::try_parse(remaining)?;
        let (right_side_bearing, remaining) = i16::try_parse(remaining)?;
        let (character_width, remaining) = i16::try_parse(remaining)?;
        let (ascent, remaining) = i16::try_parse(remaining)?;
        let (descent, remaining) = i16::try_parse(remaining)?;
        let (attributes, remaining) = u16::try_parse(remaining)?;
        let result = Charinfo { left_side_bearing, right_side_bearing, character_width, ascent, descent, attributes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Charinfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Charinfo {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let left_side_bearing_bytes = self.left_side_bearing.serialize();
        let right_side_bearing_bytes = self.right_side_bearing.serialize();
        let character_width_bytes = self.character_width.serialize();
        let ascent_bytes = self.ascent.serialize();
        let descent_bytes = self.descent.serialize();
        let attributes_bytes = self.attributes.serialize();
        [
            left_side_bearing_bytes[0],
            left_side_bearing_bytes[1],
            right_side_bearing_bytes[0],
            right_side_bearing_bytes[1],
            character_width_bytes[0],
            character_width_bytes[1],
            ascent_bytes[0],
            ascent_bytes[1],
            descent_bytes[0],
            descent_bytes[1],
            attributes_bytes[0],
            attributes_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.left_side_bearing.serialize_into(bytes);
        self.right_side_bearing.serialize_into(bytes);
        self.character_width.serialize_into(bytes);
        self.ascent.serialize_into(bytes);
        self.descent.serialize_into(bytes);
        self.attributes.serialize_into(bytes);
    }
}

/// Opcode for the QueryFont request
pub const QUERY_FONT_REQUEST: u8 = 47;
/// query font metrics.
///
/// Queries information associated with the font.
///
/// # Fields
///
/// * `font` - The fontable (Font or Graphics Context) to query.
pub fn query_font<Conn>(conn: &Conn, font: Fontable) -> Result<Cookie<'_, Conn, QueryFontReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let font_bytes = font.serialize();
    let request0 = [
        QUERY_FONT_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        font_bytes[0],
        font_bytes[1],
        font_bytes[2],
        font_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `all_chars_exist` - flag if all characters have nonzero size
/// * `default_char` - char to print for undefined character
/// * `font_ascent` - baseline to top edge of raster
/// * `font_descent` - baseline to bottom edge of raster
/// * `max_bounds` - maximum bounds over all existing char
/// * `max_char_or_byte2` - last character
/// * `min_bounds` - minimum bounds over all existing char
/// * `min_char_or_byte2` - first character
/// * `properties_len` - how many properties there are
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFontReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub draw_direction: FontDraw,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: bool,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub properties: Vec<Fontprop>,
    pub char_infos: Vec<Charinfo>,
}
impl QueryFontReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (min_bounds, remaining) = Charinfo::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (max_bounds, remaining) = Charinfo::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (min_char_or_byte2, remaining) = u16::try_parse(remaining)?;
        let (max_char_or_byte2, remaining) = u16::try_parse(remaining)?;
        let (default_char, remaining) = u16::try_parse(remaining)?;
        let (properties_len, remaining) = u16::try_parse(remaining)?;
        let (draw_direction, remaining) = u8::try_parse(remaining)?;
        let (min_byte1, remaining) = u8::try_parse(remaining)?;
        let (max_byte1, remaining) = u8::try_parse(remaining)?;
        let (all_chars_exist, remaining) = bool::try_parse(remaining)?;
        let (font_ascent, remaining) = i16::try_parse(remaining)?;
        let (font_descent, remaining) = i16::try_parse(remaining)?;
        let (char_infos_len, remaining) = u32::try_parse(remaining)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<Fontprop>(remaining, properties_len as usize)?;
        let (char_infos, remaining) = crate::x11_utils::parse_list::<Charinfo>(remaining, char_infos_len as usize)?;
        let draw_direction = draw_direction.try_into()?;
        let result = QueryFontReply { response_type, sequence, length, min_bounds, max_bounds, min_char_or_byte2, max_char_or_byte2, default_char, draw_direction, min_byte1, max_byte1, all_chars_exist, font_ascent, font_descent, properties, char_infos };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryFontReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryTextExtents request
pub const QUERY_TEXT_EXTENTS_REQUEST: u8 = 48;
/// get text extents.
///
/// Query text extents from the X11 server. This request returns the bounding box
/// of the specified 16-bit character string in the specified `font` or the font
/// contained in the specified graphics context.
///
/// `font_ascent` is set to the maximum of the ascent metrics of all characters in
/// the string. `font_descent` is set to the maximum of the descent metrics.
/// `overall_width` is set to the sum of the character-width metrics of all
/// characters in the string. For each character in the string, let W be the sum of
/// the character-width metrics of all characters preceding it in the string. Let L
/// be the left-side-bearing metric of the character plus W. Let R be the
/// right-side-bearing metric of the character plus W. The lbearing member is set
/// to the minimum L of all characters in the string. The rbearing member is set to
/// the maximum R.
///
/// For fonts defined with linear indexing rather than 2-byte matrix indexing, each
/// `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
/// most significant byte. If the font has no defined default character, undefined
/// characters in the string are taken to have all zero metrics.
///
/// Characters with all zero metrics are ignored. If the font has no defined
/// default_char, the undefined characters in the string are also ignored.
///
/// # Fields
///
/// * `font` - The `font` to calculate text extents in. You can also pass a graphics context.
/// * `string` - The text to get text extents for.
/// * `string_len` - The number of characters in `string`.
///
/// # Errors
///
/// * `Font` - The specified `font` does not exist.
/// * `GContext` - The specified graphics context does not exist.
pub fn query_text_extents<'c, Conn>(conn: &'c Conn, font: Fontable, string: &[Char2b]) -> Result<Cookie<'c, Conn, QueryTextExtentsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 2 * string.len() + 3) / 4;
    let string_len: u32 = string.len().try_into()?;
    // The following unwrap cannot fail since the value
    // is either 0 or 1. Both fit into an u8.
    let odd_length: u8 = ((string_len) & (1)).try_into().unwrap();
    let odd_length_bytes = odd_length.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let font_bytes = font.serialize();
    let string_bytes = string.serialize();
    let request0 = [
        QUERY_TEXT_EXTENTS_REQUEST,
        odd_length_bytes[0],
        length_bytes[0],
        length_bytes[1],
        font_bytes[0],
        font_bytes[1],
        font_bytes[2],
        font_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&string_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&string_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryTextExtentsReply {
    pub response_type: u8,
    pub draw_direction: FontDraw,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}
impl QueryTextExtentsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (draw_direction, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (font_ascent, remaining) = i16::try_parse(remaining)?;
        let (font_descent, remaining) = i16::try_parse(remaining)?;
        let (overall_ascent, remaining) = i16::try_parse(remaining)?;
        let (overall_descent, remaining) = i16::try_parse(remaining)?;
        let (overall_width, remaining) = i32::try_parse(remaining)?;
        let (overall_left, remaining) = i32::try_parse(remaining)?;
        let (overall_right, remaining) = i32::try_parse(remaining)?;
        let draw_direction = draw_direction.try_into()?;
        let result = QueryTextExtentsReply { response_type, draw_direction, sequence, length, font_ascent, font_descent, overall_ascent, overall_descent, overall_width, overall_left, overall_right };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryTextExtentsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Str {
    pub name: Vec<u8>,
}
impl TryParse for Str {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (name_len, remaining) = u8::try_parse(remaining)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        let result = Str { name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Str {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Str {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(1);
        let name_len = self.name.len() as u8;
        name_len.serialize_into(bytes);
        self.name.serialize_into(bytes);
    }
}

/// Opcode for the ListFonts request
pub const LIST_FONTS_REQUEST: u8 = 49;
/// get matching font names.
///
/// Gets a list of available font names which match the given `pattern`.
///
/// # Fields
///
/// * `max_names` - The maximum number of fonts to be returned.
/// * `pattern` - A font pattern, for example "-misc-fixed-*".
///
/// The asterisk (*) is a wildcard for any number of characters. The question mark
/// (?) is a wildcard for a single character. Use of uppercase or lowercase does
/// not matter.
/// * `pattern_len` - The length (in bytes) of `pattern`.
pub fn list_fonts<'c, Conn>(conn: &'c Conn, max_names: u16, pattern: &[u8]) -> Result<Cookie<'c, Conn, ListFontsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 1 * pattern.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let max_names_bytes = max_names.serialize();
    let pattern_len: u16 = pattern.len().try_into()?;
    let pattern_len_bytes = pattern_len.serialize();
    let request0 = [
        LIST_FONTS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        max_names_bytes[0],
        max_names_bytes[1],
        pattern_len_bytes[0],
        pattern_len_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (pattern).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(pattern), IoSlice::new(&padding1)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `names_len` - The number of font names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFontsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: Vec<Str>,
}
impl ListFontsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (names_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (names, remaining) = crate::x11_utils::parse_list::<Str>(remaining, names_len as usize)?;
        let result = ListFontsReply { response_type, sequence, length, names };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListFontsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListFontsWithInfo request
pub const LIST_FONTS_WITH_INFO_REQUEST: u8 = 50;
/// get matching font names and information.
///
/// Gets a list of available font names which match the given `pattern`.
///
/// # Fields
///
/// * `max_names` - The maximum number of fonts to be returned.
/// * `pattern` - A font pattern, for example "-misc-fixed-*".
///
/// The asterisk (*) is a wildcard for any number of characters. The question mark
/// (?) is a wildcard for a single character. Use of uppercase or lowercase does
/// not matter.
/// * `pattern_len` - The length (in bytes) of `pattern`.
pub fn list_fonts_with_info<'c, Conn>(conn: &'c Conn, max_names: u16, pattern: &[u8]) -> Result<ListFontsWithInfoCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 1 * pattern.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let max_names_bytes = max_names.serialize();
    let pattern_len: u16 = pattern.len().try_into()?;
    let pattern_len_bytes = pattern_len.serialize();
    let request0 = [
        LIST_FONTS_WITH_INFO_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        max_names_bytes[0],
        max_names_bytes[1],
        pattern_len_bytes[0],
        pattern_len_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (pattern).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(ListFontsWithInfoCookie::new(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(pattern), IoSlice::new(&padding1)], Vec::new())?))
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `all_chars_exist` - flag if all characters have nonzero size
/// * `default_char` - char to print for undefined character
/// * `font_ascent` - baseline to top edge of raster
/// * `font_descent` - baseline to bottom edge of raster
/// * `max_bounds` - maximum bounds over all existing char
/// * `max_char_or_byte2` - last character
/// * `min_bounds` - minimum bounds over all existing char
/// * `min_char_or_byte2` - first character
/// * `name_len` - The number of matched font names.
/// * `properties_len` - how many properties there are
/// * `replies_hint` - An indication of how many more fonts will be returned. This is only a hint and
/// may be larger or smaller than the number of fonts actually returned. A zero
/// value does not guarantee that no more fonts will be returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFontsWithInfoReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub draw_direction: FontDraw,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: bool,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub replies_hint: u32,
    pub properties: Vec<Fontprop>,
    pub name: Vec<u8>,
}
impl ListFontsWithInfoReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (name_len, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (min_bounds, remaining) = Charinfo::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (max_bounds, remaining) = Charinfo::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (min_char_or_byte2, remaining) = u16::try_parse(remaining)?;
        let (max_char_or_byte2, remaining) = u16::try_parse(remaining)?;
        let (default_char, remaining) = u16::try_parse(remaining)?;
        let (properties_len, remaining) = u16::try_parse(remaining)?;
        let (draw_direction, remaining) = u8::try_parse(remaining)?;
        let (min_byte1, remaining) = u8::try_parse(remaining)?;
        let (max_byte1, remaining) = u8::try_parse(remaining)?;
        let (all_chars_exist, remaining) = bool::try_parse(remaining)?;
        let (font_ascent, remaining) = i16::try_parse(remaining)?;
        let (font_descent, remaining) = i16::try_parse(remaining)?;
        let (replies_hint, remaining) = u32::try_parse(remaining)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<Fontprop>(remaining, properties_len as usize)?;
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
        let draw_direction = draw_direction.try_into()?;
        let result = ListFontsWithInfoReply { response_type, sequence, length, min_bounds, max_bounds, min_char_or_byte2, max_char_or_byte2, default_char, draw_direction, min_byte1, max_byte1, all_chars_exist, font_ascent, font_descent, replies_hint, properties, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListFontsWithInfoReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetFontPath request
pub const SET_FONT_PATH_REQUEST: u8 = 51;
pub fn set_font_path<'c, Conn>(conn: &'c Conn, font: &[Str]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let font_bytes = font.serialize();
    let length: usize = (8 + font_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let font_qty: u16 = font.len().try_into()?;
    let font_qty_bytes = font_qty.serialize();
    let request0 = [
        SET_FONT_PATH_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        font_qty_bytes[0],
        font_qty_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&font_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&font_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetFontPath request
pub const GET_FONT_PATH_REQUEST: u8 = 52;
pub fn get_font_path<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetFontPathReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_FONT_PATH_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFontPathReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub path: Vec<Str>,
}
impl GetFontPathReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (path_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (path, remaining) = crate::x11_utils::parse_list::<Str>(remaining, path_len as usize)?;
        let result = GetFontPathReply { response_type, sequence, length, path };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetFontPathReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the CreatePixmap request
pub const CREATE_PIXMAP_REQUEST: u8 = 53;
/// Creates a pixmap.
///
/// Creates a pixmap. The pixmap can only be used on the same screen as `drawable`
/// is on and only with drawables of the same `depth`.
///
/// # Fields
///
/// * `depth` - TODO
/// * `drawable` - Drawable to get the screen from.
/// * `height` - The height of the new pixmap.
/// * `pid` - The ID with which you will refer to the new pixmap, created by
/// `xcb_generate_id`.
/// * `width` - The width of the new pixmap.
///
/// # Errors
///
/// * `Alloc` - The X server could not allocate the requested resources (no memory?).
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `Value` - TODO: reasons?
///
/// # See
///
/// * xcb_generate_id: function
pub fn create_pixmap<Conn>(conn: &Conn, depth: u8, pid: Pixmap, drawable: Drawable, width: u16, height: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let depth_bytes = depth.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let pid_bytes = pid.serialize();
    let drawable_bytes = drawable.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        CREATE_PIXMAP_REQUEST,
        depth_bytes[0],
        length_bytes[0],
        length_bytes[1],
        pid_bytes[0],
        pid_bytes[1],
        pid_bytes[2],
        pid_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the FreePixmap request
pub const FREE_PIXMAP_REQUEST: u8 = 54;
/// Destroys a pixmap.
///
/// Deletes the association between the pixmap ID and the pixmap. The pixmap
/// storage will be freed when there are no more references to it.
///
/// # Fields
///
/// * `pixmap` - The pixmap to destroy.
///
/// # Errors
///
/// * `Pixmap` - The specified pixmap does not exist.
pub fn free_pixmap<Conn>(conn: &Conn, pixmap: Pixmap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let pixmap_bytes = pixmap.serialize();
    let request0 = [
        FREE_PIXMAP_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        pixmap_bytes[0],
        pixmap_bytes[1],
        pixmap_bytes[2],
        pixmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `ArcMode` - TODO
/// * `Background` - Background colorpixel.
/// * `CapStyle` - The cap-style defines how the endpoints of a path are drawn:
/// NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
/// endpoint is not drawn.
/// Butt       The result is square at the endpoint (perpendicular to the slope of the line)
/// with no projection beyond.
/// Round      The result is a circular arc with its diameter equal to the line-width, centered
/// on the endpoint; it is equivalent to Butt for line-width zero.
/// Projecting The result is square at the end, but the path continues beyond the endpoint for
/// a distance equal to half the line-width; it is equivalent to Butt for line-width
/// zero.
/// * `ClipMask` - The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
/// bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
/// the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
/// sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
/// depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
/// then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
/// SetClipRectangles request.
/// * `ClipOriginX` - TODO
/// * `ClipOriginY` - TODO
/// * `DashList` - TODO
/// * `DashOffset` - TODO
/// * `FillStyle` - The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
/// requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
/// as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
/// PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
/// or DoubleDash:
/// Solid                     Foreground
/// Tiled                     Tile
/// OpaqueStippled            A tile with the same width and height as stipple but with background
/// everywhere stipple has a zero and with foreground everywhere stipple
/// has a one
/// Stippled                  Foreground masked by stipple
/// For the odd dashes for line requests with line-style DoubleDash:
/// Solid                     Background
/// Tiled                     Same as for even dashes
/// OpaqueStippled            Same as for even dashes
/// Stippled                  Background masked by stipple
/// * `Font` - Which font to use for the `ImageText8` and `ImageText16` requests.
/// * `Foreground` - Foreground colorpixel.
/// * `Function` - TODO: Refer to GX
/// * `GraphicsExposures` - Whether ExposureEvents should be generated (1) or not (0).
///
/// The default is 1.
/// * `JoinStyle` - The join-style defines how corners are drawn for wide lines:
/// Miter               The outer edges of the two lines extend to meet at an angle. However, if the
/// angle is less than 11 degrees, a Bevel join-style is used instead.
/// Round               The result is a circular arc with a diameter equal to the line-width, centered
/// on the joinpoint.
/// Bevel               The result is Butt endpoint styles, and then the triangular notch is filled.
/// * `LineStyle` - The line-style defines which sections of a line are drawn:
/// Solid                The full path of the line is drawn.
/// DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
/// than the odd dashes (see fill-style), with Butt cap-style used where even and
/// odd dashes meet.
/// OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
/// the individual dashes (except NotLast is treated as Butt).
/// * `LineWidth` - The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
/// special value zero, a thin line.
/// * `PlaneMask` - In graphics operations, given a source and destination pixel, the result is
/// computed bitwise on corresponding bits of the pixels; that is, a Boolean
/// operation is performed in each bit plane. The plane-mask restricts the
/// operation to a subset of planes, so the result is:
///
/// ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask))
/// * `Stipple` - The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
/// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
/// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
/// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
/// origin of whatever destination drawable is specified in a graphics request.
/// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
/// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
/// Match error results). For fill-style Stippled (but not fill-style
/// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
/// additional clip mask to be ANDed with the clip-mask.
/// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
/// others.
/// * `SubwindowMode` - For ClipByChildren, both source and destination windows are additionally
/// clipped by all viewable InputOutput children. For IncludeInferiors, neither
/// source nor destination window is
/// clipped by inferiors. This will result in including subwindow contents in the source and drawing
/// through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
/// destination window of one depth with mapped inferiors of differing depth is not illegal, but the
/// semantics is undefined by the core protocol.
/// * `Tile` - The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
/// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
/// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
/// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
/// origin of whatever destination drawable is specified in a graphics request.
/// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
/// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
/// Match error results). For fill-style Stippled (but not fill-style
/// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
/// additional clip mask to be ANDed with the clip-mask.
/// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
/// others.
/// * `TileStippleOriginX` - TODO
/// * `TileStippleOriginY` - TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GC {
    Function = 1 << 0,
    PlaneMask = 1 << 1,
    Foreground = 1 << 2,
    Background = 1 << 3,
    LineWidth = 1 << 4,
    LineStyle = 1 << 5,
    CapStyle = 1 << 6,
    JoinStyle = 1 << 7,
    FillStyle = 1 << 8,
    FillRule = 1 << 9,
    Tile = 1 << 10,
    Stipple = 1 << 11,
    TileStippleOriginX = 1 << 12,
    TileStippleOriginY = 1 << 13,
    Font = 1 << 14,
    SubwindowMode = 1 << 15,
    GraphicsExposures = 1 << 16,
    ClipOriginX = 1 << 17,
    ClipOriginY = 1 << 18,
    ClipMask = 1 << 19,
    DashOffset = 1 << 20,
    DashList = 1 << 21,
    ArcMode = 1 << 22,
}
impl From<GC> for u32 {
    fn from(input: GC) -> Self {
        match input {
            GC::Function => 1 << 0,
            GC::PlaneMask => 1 << 1,
            GC::Foreground => 1 << 2,
            GC::Background => 1 << 3,
            GC::LineWidth => 1 << 4,
            GC::LineStyle => 1 << 5,
            GC::CapStyle => 1 << 6,
            GC::JoinStyle => 1 << 7,
            GC::FillStyle => 1 << 8,
            GC::FillRule => 1 << 9,
            GC::Tile => 1 << 10,
            GC::Stipple => 1 << 11,
            GC::TileStippleOriginX => 1 << 12,
            GC::TileStippleOriginY => 1 << 13,
            GC::Font => 1 << 14,
            GC::SubwindowMode => 1 << 15,
            GC::GraphicsExposures => 1 << 16,
            GC::ClipOriginX => 1 << 17,
            GC::ClipOriginY => 1 << 18,
            GC::ClipMask => 1 << 19,
            GC::DashOffset => 1 << 20,
            GC::DashList => 1 << 21,
            GC::ArcMode => 1 << 22,
        }
    }
}
impl From<GC> for Option<u32> {
    fn from(input: GC) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u32> for GC {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GC::Function),
            2 => Ok(GC::PlaneMask),
            4 => Ok(GC::Foreground),
            8 => Ok(GC::Background),
            16 => Ok(GC::LineWidth),
            32 => Ok(GC::LineStyle),
            64 => Ok(GC::CapStyle),
            128 => Ok(GC::JoinStyle),
            256 => Ok(GC::FillStyle),
            512 => Ok(GC::FillRule),
            1024 => Ok(GC::Tile),
            2048 => Ok(GC::Stipple),
            4096 => Ok(GC::TileStippleOriginX),
            8192 => Ok(GC::TileStippleOriginY),
            16384 => Ok(GC::Font),
            32768 => Ok(GC::SubwindowMode),
            65536 => Ok(GC::GraphicsExposures),
            131_072 => Ok(GC::ClipOriginX),
            262_144 => Ok(GC::ClipOriginY),
            524_288 => Ok(GC::ClipMask),
            1_048_576 => Ok(GC::DashOffset),
            2_097_152 => Ok(GC::DashList),
            4_194_304 => Ok(GC::ArcMode),
            _ => Err(ParseError::ParseError),
        }
    }
}
bitmask_binop!(GC, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GX {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}
impl From<GX> for u8 {
    fn from(input: GX) -> Self {
        match input {
            GX::Clear => 0,
            GX::And => 1,
            GX::AndReverse => 2,
            GX::Copy => 3,
            GX::AndInverted => 4,
            GX::Noop => 5,
            GX::Xor => 6,
            GX::Or => 7,
            GX::Nor => 8,
            GX::Equiv => 9,
            GX::Invert => 10,
            GX::OrReverse => 11,
            GX::CopyInverted => 12,
            GX::OrInverted => 13,
            GX::Nand => 14,
            GX::Set => 15,
        }
    }
}
impl From<GX> for Option<u8> {
    fn from(input: GX) -> Self {
        Some(u8::from(input))
    }
}
impl From<GX> for u16 {
    fn from(input: GX) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GX> for Option<u16> {
    fn from(input: GX) -> Self {
        Some(u16::from(input))
    }
}
impl From<GX> for u32 {
    fn from(input: GX) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<GX> for Option<u32> {
    fn from(input: GX) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for GX {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GX::Clear),
            1 => Ok(GX::And),
            2 => Ok(GX::AndReverse),
            3 => Ok(GX::Copy),
            4 => Ok(GX::AndInverted),
            5 => Ok(GX::Noop),
            6 => Ok(GX::Xor),
            7 => Ok(GX::Or),
            8 => Ok(GX::Nor),
            9 => Ok(GX::Equiv),
            10 => Ok(GX::Invert),
            11 => Ok(GX::OrReverse),
            12 => Ok(GX::CopyInverted),
            13 => Ok(GX::OrInverted),
            14 => Ok(GX::Nand),
            15 => Ok(GX::Set),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for GX {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for GX {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LineStyle {
    Solid = 0,
    OnOffDash = 1,
    DoubleDash = 2,
}
impl From<LineStyle> for u8 {
    fn from(input: LineStyle) -> Self {
        match input {
            LineStyle::Solid => 0,
            LineStyle::OnOffDash => 1,
            LineStyle::DoubleDash => 2,
        }
    }
}
impl From<LineStyle> for Option<u8> {
    fn from(input: LineStyle) -> Self {
        Some(u8::from(input))
    }
}
impl From<LineStyle> for u16 {
    fn from(input: LineStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LineStyle> for Option<u16> {
    fn from(input: LineStyle) -> Self {
        Some(u16::from(input))
    }
}
impl From<LineStyle> for u32 {
    fn from(input: LineStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LineStyle> for Option<u32> {
    fn from(input: LineStyle) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for LineStyle {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LineStyle::Solid),
            1 => Ok(LineStyle::OnOffDash),
            2 => Ok(LineStyle::DoubleDash),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for LineStyle {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for LineStyle {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CapStyle {
    NotLast = 0,
    Butt = 1,
    Round = 2,
    Projecting = 3,
}
impl From<CapStyle> for u8 {
    fn from(input: CapStyle) -> Self {
        match input {
            CapStyle::NotLast => 0,
            CapStyle::Butt => 1,
            CapStyle::Round => 2,
            CapStyle::Projecting => 3,
        }
    }
}
impl From<CapStyle> for Option<u8> {
    fn from(input: CapStyle) -> Self {
        Some(u8::from(input))
    }
}
impl From<CapStyle> for u16 {
    fn from(input: CapStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CapStyle> for Option<u16> {
    fn from(input: CapStyle) -> Self {
        Some(u16::from(input))
    }
}
impl From<CapStyle> for u32 {
    fn from(input: CapStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CapStyle> for Option<u32> {
    fn from(input: CapStyle) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CapStyle {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CapStyle::NotLast),
            1 => Ok(CapStyle::Butt),
            2 => Ok(CapStyle::Round),
            3 => Ok(CapStyle::Projecting),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CapStyle {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CapStyle {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum JoinStyle {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}
impl From<JoinStyle> for u8 {
    fn from(input: JoinStyle) -> Self {
        match input {
            JoinStyle::Miter => 0,
            JoinStyle::Round => 1,
            JoinStyle::Bevel => 2,
        }
    }
}
impl From<JoinStyle> for Option<u8> {
    fn from(input: JoinStyle) -> Self {
        Some(u8::from(input))
    }
}
impl From<JoinStyle> for u16 {
    fn from(input: JoinStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<JoinStyle> for Option<u16> {
    fn from(input: JoinStyle) -> Self {
        Some(u16::from(input))
    }
}
impl From<JoinStyle> for u32 {
    fn from(input: JoinStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<JoinStyle> for Option<u32> {
    fn from(input: JoinStyle) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for JoinStyle {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JoinStyle::Miter),
            1 => Ok(JoinStyle::Round),
            2 => Ok(JoinStyle::Bevel),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for JoinStyle {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for JoinStyle {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FillStyle {
    Solid = 0,
    Tiled = 1,
    Stippled = 2,
    OpaqueStippled = 3,
}
impl From<FillStyle> for u8 {
    fn from(input: FillStyle) -> Self {
        match input {
            FillStyle::Solid => 0,
            FillStyle::Tiled => 1,
            FillStyle::Stippled => 2,
            FillStyle::OpaqueStippled => 3,
        }
    }
}
impl From<FillStyle> for Option<u8> {
    fn from(input: FillStyle) -> Self {
        Some(u8::from(input))
    }
}
impl From<FillStyle> for u16 {
    fn from(input: FillStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FillStyle> for Option<u16> {
    fn from(input: FillStyle) -> Self {
        Some(u16::from(input))
    }
}
impl From<FillStyle> for u32 {
    fn from(input: FillStyle) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FillStyle> for Option<u32> {
    fn from(input: FillStyle) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for FillStyle {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FillStyle::Solid),
            1 => Ok(FillStyle::Tiled),
            2 => Ok(FillStyle::Stippled),
            3 => Ok(FillStyle::OpaqueStippled),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for FillStyle {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for FillStyle {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FillRule {
    EvenOdd = 0,
    Winding = 1,
}
impl From<FillRule> for u8 {
    fn from(input: FillRule) -> Self {
        match input {
            FillRule::EvenOdd => 0,
            FillRule::Winding => 1,
        }
    }
}
impl From<FillRule> for Option<u8> {
    fn from(input: FillRule) -> Self {
        Some(u8::from(input))
    }
}
impl From<FillRule> for u16 {
    fn from(input: FillRule) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FillRule> for Option<u16> {
    fn from(input: FillRule) -> Self {
        Some(u16::from(input))
    }
}
impl From<FillRule> for u32 {
    fn from(input: FillRule) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FillRule> for Option<u32> {
    fn from(input: FillRule) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for FillRule {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FillRule::EvenOdd),
            1 => Ok(FillRule::Winding),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for FillRule {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for FillRule {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SubwindowMode {
    ClipByChildren = 0,
    IncludeInferiors = 1,
}
impl From<SubwindowMode> for u8 {
    fn from(input: SubwindowMode) -> Self {
        match input {
            SubwindowMode::ClipByChildren => 0,
            SubwindowMode::IncludeInferiors => 1,
        }
    }
}
impl From<SubwindowMode> for Option<u8> {
    fn from(input: SubwindowMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<SubwindowMode> for u16 {
    fn from(input: SubwindowMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SubwindowMode> for Option<u16> {
    fn from(input: SubwindowMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<SubwindowMode> for u32 {
    fn from(input: SubwindowMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<SubwindowMode> for Option<u32> {
    fn from(input: SubwindowMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for SubwindowMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubwindowMode::ClipByChildren),
            1 => Ok(SubwindowMode::IncludeInferiors),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for SubwindowMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for SubwindowMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArcMode {
    Chord = 0,
    PieSlice = 1,
}
impl From<ArcMode> for u8 {
    fn from(input: ArcMode) -> Self {
        match input {
            ArcMode::Chord => 0,
            ArcMode::PieSlice => 1,
        }
    }
}
impl From<ArcMode> for Option<u8> {
    fn from(input: ArcMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<ArcMode> for u16 {
    fn from(input: ArcMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ArcMode> for Option<u16> {
    fn from(input: ArcMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<ArcMode> for u32 {
    fn from(input: ArcMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ArcMode> for Option<u32> {
    fn from(input: ArcMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ArcMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArcMode::Chord),
            1 => Ok(ArcMode::PieSlice),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ArcMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ArcMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CreateGC request
pub const CREATE_GC_REQUEST: u8 = 55;
/// Auxiliary and optional information for the create_gc function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateGCAux {
    pub function: Option<u32>,
    pub plane_mask: Option<u32>,
    pub foreground: Option<u32>,
    pub background: Option<u32>,
    pub line_width: Option<u32>,
    pub line_style: Option<u32>,
    pub cap_style: Option<u32>,
    pub join_style: Option<u32>,
    pub fill_style: Option<u32>,
    pub fill_rule: Option<u32>,
    pub tile: Option<Pixmap>,
    pub stipple: Option<Pixmap>,
    pub tile_stipple_x_origin: Option<i32>,
    pub tile_stipple_y_origin: Option<i32>,
    pub font: Option<Font>,
    pub subwindow_mode: Option<u32>,
    pub graphics_exposures: Option<Bool32>,
    pub clip_x_origin: Option<i32>,
    pub clip_y_origin: Option<i32>,
    pub clip_mask: Option<Pixmap>,
    pub dash_offset: Option<u32>,
    pub dashes: Option<u32>,
    pub arc_mode: Option<u32>,
}
impl CreateGCAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.function.is_some() {
            mask |= u32::from(GC::Function);
        }
        if self.plane_mask.is_some() {
            mask |= u32::from(GC::PlaneMask);
        }
        if self.foreground.is_some() {
            mask |= u32::from(GC::Foreground);
        }
        if self.background.is_some() {
            mask |= u32::from(GC::Background);
        }
        if self.line_width.is_some() {
            mask |= u32::from(GC::LineWidth);
        }
        if self.line_style.is_some() {
            mask |= u32::from(GC::LineStyle);
        }
        if self.cap_style.is_some() {
            mask |= u32::from(GC::CapStyle);
        }
        if self.join_style.is_some() {
            mask |= u32::from(GC::JoinStyle);
        }
        if self.fill_style.is_some() {
            mask |= u32::from(GC::FillStyle);
        }
        if self.fill_rule.is_some() {
            mask |= u32::from(GC::FillRule);
        }
        if self.tile.is_some() {
            mask |= u32::from(GC::Tile);
        }
        if self.stipple.is_some() {
            mask |= u32::from(GC::Stipple);
        }
        if self.tile_stipple_x_origin.is_some() {
            mask |= u32::from(GC::TileStippleOriginX);
        }
        if self.tile_stipple_y_origin.is_some() {
            mask |= u32::from(GC::TileStippleOriginY);
        }
        if self.font.is_some() {
            mask |= u32::from(GC::Font);
        }
        if self.subwindow_mode.is_some() {
            mask |= u32::from(GC::SubwindowMode);
        }
        if self.graphics_exposures.is_some() {
            mask |= u32::from(GC::GraphicsExposures);
        }
        if self.clip_x_origin.is_some() {
            mask |= u32::from(GC::ClipOriginX);
        }
        if self.clip_y_origin.is_some() {
            mask |= u32::from(GC::ClipOriginY);
        }
        if self.clip_mask.is_some() {
            mask |= u32::from(GC::ClipMask);
        }
        if self.dash_offset.is_some() {
            mask |= u32::from(GC::DashOffset);
        }
        if self.dashes.is_some() {
            mask |= u32::from(GC::DashList);
        }
        if self.arc_mode.is_some() {
            mask |= u32::from(GC::ArcMode);
        }
        mask
    }
    /// Set the function field of this structure.
    pub fn function<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.function = value.into();
        self
    }
    /// Set the plane_mask field of this structure.
    pub fn plane_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.plane_mask = value.into();
        self
    }
    /// Set the foreground field of this structure.
    pub fn foreground<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.foreground = value.into();
        self
    }
    /// Set the background field of this structure.
    pub fn background<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background = value.into();
        self
    }
    /// Set the line_width field of this structure.
    pub fn line_width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.line_width = value.into();
        self
    }
    /// Set the line_style field of this structure.
    pub fn line_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.line_style = value.into();
        self
    }
    /// Set the cap_style field of this structure.
    pub fn cap_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.cap_style = value.into();
        self
    }
    /// Set the join_style field of this structure.
    pub fn join_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.join_style = value.into();
        self
    }
    /// Set the fill_style field of this structure.
    pub fn fill_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.fill_style = value.into();
        self
    }
    /// Set the fill_rule field of this structure.
    pub fn fill_rule<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.fill_rule = value.into();
        self
    }
    /// Set the tile field of this structure.
    pub fn tile<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.tile = value.into();
        self
    }
    /// Set the stipple field of this structure.
    pub fn stipple<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.stipple = value.into();
        self
    }
    /// Set the tile_stipple_x_origin field of this structure.
    pub fn tile_stipple_x_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.tile_stipple_x_origin = value.into();
        self
    }
    /// Set the tile_stipple_y_origin field of this structure.
    pub fn tile_stipple_y_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.tile_stipple_y_origin = value.into();
        self
    }
    /// Set the font field of this structure.
    pub fn font<I>(mut self, value: I) -> Self where I: Into<Option<Font>> {
        self.font = value.into();
        self
    }
    /// Set the subwindow_mode field of this structure.
    pub fn subwindow_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.subwindow_mode = value.into();
        self
    }
    /// Set the graphics_exposures field of this structure.
    pub fn graphics_exposures<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.graphics_exposures = value.into();
        self
    }
    /// Set the clip_x_origin field of this structure.
    pub fn clip_x_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clip_x_origin = value.into();
        self
    }
    /// Set the clip_y_origin field of this structure.
    pub fn clip_y_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clip_y_origin = value.into();
        self
    }
    /// Set the clip_mask field of this structure.
    pub fn clip_mask<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.clip_mask = value.into();
        self
    }
    /// Set the dash_offset field of this structure.
    pub fn dash_offset<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.dash_offset = value.into();
        self
    }
    /// Set the dashes field of this structure.
    pub fn dashes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.dashes = value.into();
        self
    }
    /// Set the arc_mode field of this structure.
    pub fn arc_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.arc_mode = value.into();
        self
    }
}
impl Serialize for CreateGCAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.function {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.plane_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.foreground {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.background {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.line_width {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.line_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.cap_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.join_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.fill_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.fill_rule {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.stipple {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile_stipple_x_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile_stipple_y_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.font {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.subwindow_mode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.graphics_exposures {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_x_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_y_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dash_offset {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dashes {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.arc_mode {
            value.serialize_into(bytes);
        }
    }
}
/// Creates a graphics context.
///
/// Creates a graphics context. The graphics context can be used with any drawable
/// that has the same root and depth as the specified drawable.
///
/// # Fields
///
/// * `cid` - The ID with which you will refer to the graphics context, created by
/// `xcb_generate_id`.
/// * `drawable` - Drawable to get the root/depth from.
///
/// # Errors
///
/// * `Alloc` - The X server could not allocate the requested resources (no memory?).
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `Font` - TODO: reasons?
/// * `Match` - TODO: reasons?
/// * `Pixmap` - TODO: reasons?
/// * `Value` - TODO: reasons?
///
/// # See
///
/// * xcb_generate_id: function
pub fn create_gc<'c, Conn>(conn: &'c Conn, cid: Gcontext, drawable: Drawable, value_list: &CreateGCAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (16 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cid_bytes = cid.serialize();
    let drawable_bytes = drawable.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CREATE_GC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cid_bytes[0],
        cid_bytes[1],
        cid_bytes[2],
        cid_bytes[3],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeGC request
pub const CHANGE_GC_REQUEST: u8 = 56;
/// Auxiliary and optional information for the change_gc function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeGCAux {
    pub function: Option<u32>,
    pub plane_mask: Option<u32>,
    pub foreground: Option<u32>,
    pub background: Option<u32>,
    pub line_width: Option<u32>,
    pub line_style: Option<u32>,
    pub cap_style: Option<u32>,
    pub join_style: Option<u32>,
    pub fill_style: Option<u32>,
    pub fill_rule: Option<u32>,
    pub tile: Option<Pixmap>,
    pub stipple: Option<Pixmap>,
    pub tile_stipple_x_origin: Option<i32>,
    pub tile_stipple_y_origin: Option<i32>,
    pub font: Option<Font>,
    pub subwindow_mode: Option<u32>,
    pub graphics_exposures: Option<Bool32>,
    pub clip_x_origin: Option<i32>,
    pub clip_y_origin: Option<i32>,
    pub clip_mask: Option<Pixmap>,
    pub dash_offset: Option<u32>,
    pub dashes: Option<u32>,
    pub arc_mode: Option<u32>,
}
impl ChangeGCAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.function.is_some() {
            mask |= u32::from(GC::Function);
        }
        if self.plane_mask.is_some() {
            mask |= u32::from(GC::PlaneMask);
        }
        if self.foreground.is_some() {
            mask |= u32::from(GC::Foreground);
        }
        if self.background.is_some() {
            mask |= u32::from(GC::Background);
        }
        if self.line_width.is_some() {
            mask |= u32::from(GC::LineWidth);
        }
        if self.line_style.is_some() {
            mask |= u32::from(GC::LineStyle);
        }
        if self.cap_style.is_some() {
            mask |= u32::from(GC::CapStyle);
        }
        if self.join_style.is_some() {
            mask |= u32::from(GC::JoinStyle);
        }
        if self.fill_style.is_some() {
            mask |= u32::from(GC::FillStyle);
        }
        if self.fill_rule.is_some() {
            mask |= u32::from(GC::FillRule);
        }
        if self.tile.is_some() {
            mask |= u32::from(GC::Tile);
        }
        if self.stipple.is_some() {
            mask |= u32::from(GC::Stipple);
        }
        if self.tile_stipple_x_origin.is_some() {
            mask |= u32::from(GC::TileStippleOriginX);
        }
        if self.tile_stipple_y_origin.is_some() {
            mask |= u32::from(GC::TileStippleOriginY);
        }
        if self.font.is_some() {
            mask |= u32::from(GC::Font);
        }
        if self.subwindow_mode.is_some() {
            mask |= u32::from(GC::SubwindowMode);
        }
        if self.graphics_exposures.is_some() {
            mask |= u32::from(GC::GraphicsExposures);
        }
        if self.clip_x_origin.is_some() {
            mask |= u32::from(GC::ClipOriginX);
        }
        if self.clip_y_origin.is_some() {
            mask |= u32::from(GC::ClipOriginY);
        }
        if self.clip_mask.is_some() {
            mask |= u32::from(GC::ClipMask);
        }
        if self.dash_offset.is_some() {
            mask |= u32::from(GC::DashOffset);
        }
        if self.dashes.is_some() {
            mask |= u32::from(GC::DashList);
        }
        if self.arc_mode.is_some() {
            mask |= u32::from(GC::ArcMode);
        }
        mask
    }
    /// Set the function field of this structure.
    pub fn function<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.function = value.into();
        self
    }
    /// Set the plane_mask field of this structure.
    pub fn plane_mask<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.plane_mask = value.into();
        self
    }
    /// Set the foreground field of this structure.
    pub fn foreground<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.foreground = value.into();
        self
    }
    /// Set the background field of this structure.
    pub fn background<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.background = value.into();
        self
    }
    /// Set the line_width field of this structure.
    pub fn line_width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.line_width = value.into();
        self
    }
    /// Set the line_style field of this structure.
    pub fn line_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.line_style = value.into();
        self
    }
    /// Set the cap_style field of this structure.
    pub fn cap_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.cap_style = value.into();
        self
    }
    /// Set the join_style field of this structure.
    pub fn join_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.join_style = value.into();
        self
    }
    /// Set the fill_style field of this structure.
    pub fn fill_style<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.fill_style = value.into();
        self
    }
    /// Set the fill_rule field of this structure.
    pub fn fill_rule<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.fill_rule = value.into();
        self
    }
    /// Set the tile field of this structure.
    pub fn tile<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.tile = value.into();
        self
    }
    /// Set the stipple field of this structure.
    pub fn stipple<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.stipple = value.into();
        self
    }
    /// Set the tile_stipple_x_origin field of this structure.
    pub fn tile_stipple_x_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.tile_stipple_x_origin = value.into();
        self
    }
    /// Set the tile_stipple_y_origin field of this structure.
    pub fn tile_stipple_y_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.tile_stipple_y_origin = value.into();
        self
    }
    /// Set the font field of this structure.
    pub fn font<I>(mut self, value: I) -> Self where I: Into<Option<Font>> {
        self.font = value.into();
        self
    }
    /// Set the subwindow_mode field of this structure.
    pub fn subwindow_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.subwindow_mode = value.into();
        self
    }
    /// Set the graphics_exposures field of this structure.
    pub fn graphics_exposures<I>(mut self, value: I) -> Self where I: Into<Option<Bool32>> {
        self.graphics_exposures = value.into();
        self
    }
    /// Set the clip_x_origin field of this structure.
    pub fn clip_x_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clip_x_origin = value.into();
        self
    }
    /// Set the clip_y_origin field of this structure.
    pub fn clip_y_origin<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.clip_y_origin = value.into();
        self
    }
    /// Set the clip_mask field of this structure.
    pub fn clip_mask<I>(mut self, value: I) -> Self where I: Into<Option<Pixmap>> {
        self.clip_mask = value.into();
        self
    }
    /// Set the dash_offset field of this structure.
    pub fn dash_offset<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.dash_offset = value.into();
        self
    }
    /// Set the dashes field of this structure.
    pub fn dashes<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.dashes = value.into();
        self
    }
    /// Set the arc_mode field of this structure.
    pub fn arc_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.arc_mode = value.into();
        self
    }
}
impl Serialize for ChangeGCAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.function {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.plane_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.foreground {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.background {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.line_width {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.line_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.cap_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.join_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.fill_style {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.fill_rule {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.stipple {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile_stipple_x_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.tile_stipple_y_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.font {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.subwindow_mode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.graphics_exposures {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_x_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_y_origin {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.clip_mask {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dash_offset {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.dashes {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.arc_mode {
            value.serialize_into(bytes);
        }
    }
}
/// change graphics context components.
///
/// Changes the components specified by `value_mask` for the specified graphics context.
///
/// # Fields
///
/// * `gc` - The graphics context to change.
/// * `value_list` - Values for each of the components specified in the bitmask `value_mask`. The
/// order has to correspond to the order of possible `value_mask` bits. See the
/// example.
///
/// # Errors
///
/// * `Alloc` - The X server could not allocate the requested resources (no memory?).
/// * `Font` - TODO: reasons?
/// * `GContext` - TODO: reasons?
/// * `Match` - TODO: reasons?
/// * `Pixmap` - TODO: reasons?
/// * `Value` - TODO: reasons?
///
/// # Example
///
/// ```text
/// /*
///  * Changes the foreground color component of the specified graphics context.
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_gcontext_t gc, uint32_t fg, uint32_t bg) {
///     /* C99 allows us to use a compact way of changing a single component: */
///     xcb_change_gc(conn, gc, XCB_GC_FOREGROUND, (uint32_t[]){ fg });
///
///     /* The more explicit way. Beware that the order of values is important! */
///     uint32_t mask = 0;
///     mask |= XCB_GC_FOREGROUND;
///     mask |= XCB_GC_BACKGROUND;
///
///     uint32_t values[] = {
///         fg,
///         bg
///     };
///     xcb_change_gc(conn, gc, mask, values);
///     xcb_flush(conn);
/// }
/// ```
pub fn change_gc<'c, Conn>(conn: &'c Conn, gc: Gcontext, value_list: &ChangeGCAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (12 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let gc_bytes = gc.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CHANGE_GC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the CopyGC request
pub const COPY_GC_REQUEST: u8 = 57;
pub fn copy_gc<Conn>(conn: &Conn, src_gc: Gcontext, dst_gc: Gcontext, value_mask: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_gc_bytes = src_gc.serialize();
    let dst_gc_bytes = dst_gc.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        COPY_GC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        src_gc_bytes[0],
        src_gc_bytes[1],
        src_gc_bytes[2],
        src_gc_bytes[3],
        dst_gc_bytes[0],
        dst_gc_bytes[1],
        dst_gc_bytes[2],
        dst_gc_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetDashes request
pub const SET_DASHES_REQUEST: u8 = 58;
pub fn set_dashes<'c, Conn>(conn: &'c Conn, gc: Gcontext, dash_offset: u16, dashes: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 1 * dashes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let gc_bytes = gc.serialize();
    let dash_offset_bytes = dash_offset.serialize();
    let dashes_len: u16 = dashes.len().try_into()?;
    let dashes_len_bytes = dashes_len.serialize();
    let request0 = [
        SET_DASHES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        dash_offset_bytes[0],
        dash_offset_bytes[1],
        dashes_len_bytes[0],
        dashes_len_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (dashes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(dashes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClipOrdering {
    Unsorted = 0,
    YSorted = 1,
    YXSorted = 2,
    YXBanded = 3,
}
impl From<ClipOrdering> for u8 {
    fn from(input: ClipOrdering) -> Self {
        match input {
            ClipOrdering::Unsorted => 0,
            ClipOrdering::YSorted => 1,
            ClipOrdering::YXSorted => 2,
            ClipOrdering::YXBanded => 3,
        }
    }
}
impl From<ClipOrdering> for Option<u8> {
    fn from(input: ClipOrdering) -> Self {
        Some(u8::from(input))
    }
}
impl From<ClipOrdering> for u16 {
    fn from(input: ClipOrdering) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClipOrdering> for Option<u16> {
    fn from(input: ClipOrdering) -> Self {
        Some(u16::from(input))
    }
}
impl From<ClipOrdering> for u32 {
    fn from(input: ClipOrdering) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClipOrdering> for Option<u32> {
    fn from(input: ClipOrdering) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ClipOrdering {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ClipOrdering::Unsorted),
            1 => Ok(ClipOrdering::YSorted),
            2 => Ok(ClipOrdering::YXSorted),
            3 => Ok(ClipOrdering::YXBanded),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ClipOrdering {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ClipOrdering {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetClipRectangles request
pub const SET_CLIP_RECTANGLES_REQUEST: u8 = 59;
pub fn set_clip_rectangles<'c, Conn, A>(conn: &'c Conn, ordering: A, gc: Gcontext, clip_x_origin: i16, clip_y_origin: i16, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12 + 8 * rectangles.len() + 3) / 4;
    let ordering = ordering.into();
    let ordering_bytes = ordering.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let gc_bytes = gc.serialize();
    let clip_x_origin_bytes = clip_x_origin.serialize();
    let clip_y_origin_bytes = clip_y_origin.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        SET_CLIP_RECTANGLES_REQUEST,
        ordering_bytes[0],
        length_bytes[0],
        length_bytes[1],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        clip_x_origin_bytes[0],
        clip_x_origin_bytes[1],
        clip_y_origin_bytes[0],
        clip_y_origin_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the FreeGC request
pub const FREE_GC_REQUEST: u8 = 60;
/// Destroys a graphics context.
///
/// Destroys the specified `gc` and all associated storage.
///
/// # Fields
///
/// * `gc` - The graphics context to destroy.
///
/// # Errors
///
/// * `GContext` - The specified graphics context does not exist.
pub fn free_gc<Conn>(conn: &Conn, gc: Gcontext) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let gc_bytes = gc.serialize();
    let request0 = [
        FREE_GC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ClearArea request
pub const CLEAR_AREA_REQUEST: u8 = 61;
pub fn clear_area<Conn>(conn: &Conn, exposures: bool, window: Window, x: i16, y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let exposures_bytes = (exposures as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        CLEAR_AREA_REQUEST,
        exposures_bytes[0],
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CopyArea request
pub const COPY_AREA_REQUEST: u8 = 62;
/// copy areas.
///
/// Copies the specified rectangle from `src_drawable` to `dst_drawable`.
///
/// # Fields
///
/// * `dst_drawable` - The destination drawable (Window or Pixmap).
/// * `dst_x` - The destination X coordinate.
/// * `dst_y` - The destination Y coordinate.
/// * `gc` - The graphics context to use.
/// * `height` - The height of the area to copy (in pixels).
/// * `src_drawable` - The source drawable (Window or Pixmap).
/// * `src_x` - The source X coordinate.
/// * `src_y` - The source Y coordinate.
/// * `width` - The width of the area to copy (in pixels).
///
/// # Errors
///
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `GContext` - The specified graphics context does not exist.
/// * `Match` - `src_drawable` has a different root or depth than `dst_drawable`.
pub fn copy_area<Conn>(conn: &Conn, src_drawable: Drawable, dst_drawable: Drawable, gc: Gcontext, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (28) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_drawable_bytes = src_drawable.serialize();
    let dst_drawable_bytes = dst_drawable.serialize();
    let gc_bytes = gc.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        COPY_AREA_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        src_drawable_bytes[0],
        src_drawable_bytes[1],
        src_drawable_bytes[2],
        src_drawable_bytes[3],
        dst_drawable_bytes[0],
        dst_drawable_bytes[1],
        dst_drawable_bytes[2],
        dst_drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_y_bytes[0],
        dst_y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CopyPlane request
pub const COPY_PLANE_REQUEST: u8 = 63;
pub fn copy_plane<Conn>(conn: &Conn, src_drawable: Drawable, dst_drawable: Drawable, gc: Gcontext, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16, bit_plane: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (32) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let src_drawable_bytes = src_drawable.serialize();
    let dst_drawable_bytes = dst_drawable.serialize();
    let gc_bytes = gc.serialize();
    let src_x_bytes = src_x.serialize();
    let src_y_bytes = src_y.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let bit_plane_bytes = bit_plane.serialize();
    let request0 = [
        COPY_PLANE_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        src_drawable_bytes[0],
        src_drawable_bytes[1],
        src_drawable_bytes[2],
        src_drawable_bytes[3],
        dst_drawable_bytes[0],
        dst_drawable_bytes[1],
        dst_drawable_bytes[2],
        dst_drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        src_x_bytes[0],
        src_x_bytes[1],
        src_y_bytes[0],
        src_y_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_y_bytes[0],
        dst_y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        bit_plane_bytes[0],
        bit_plane_bytes[1],
        bit_plane_bytes[2],
        bit_plane_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `Origin` - Treats all coordinates as relative to the origin.
/// * `Previous` - Treats all coordinates after the first as relative to the previous coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CoordMode {
    Origin = 0,
    Previous = 1,
}
impl From<CoordMode> for u8 {
    fn from(input: CoordMode) -> Self {
        match input {
            CoordMode::Origin => 0,
            CoordMode::Previous => 1,
        }
    }
}
impl From<CoordMode> for Option<u8> {
    fn from(input: CoordMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<CoordMode> for u16 {
    fn from(input: CoordMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CoordMode> for Option<u16> {
    fn from(input: CoordMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<CoordMode> for u32 {
    fn from(input: CoordMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CoordMode> for Option<u32> {
    fn from(input: CoordMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CoordMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CoordMode::Origin),
            1 => Ok(CoordMode::Previous),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CoordMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CoordMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the PolyPoint request
pub const POLY_POINT_REQUEST: u8 = 64;
pub fn poly_point<'c, Conn, A>(conn: &'c Conn, coordinate_mode: A, drawable: Drawable, gc: Gcontext, points: &[Point]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12 + 4 * points.len() + 3) / 4;
    let coordinate_mode = coordinate_mode.into();
    let coordinate_mode_bytes = coordinate_mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let points_bytes = points.serialize();
    let request0 = [
        POLY_POINT_REQUEST,
        coordinate_mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&points_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&points_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyLine request
pub const POLY_LINE_REQUEST: u8 = 65;
/// draw lines.
///
/// Draws `points_len`-1 lines between each pair of points (point[i], point[i+1])
/// in the `points` array. The lines are drawn in the order listed in the array.
/// They join correctly at all intermediate points, and if the first and last
/// points coincide, the first and last lines also join correctly. For any given
/// line, a pixel is not drawn more than once. If thin (zero line-width) lines
/// intersect, the intersecting pixels are drawn multiple times. If wide lines
/// intersect, the intersecting pixels are drawn only once, as though the entire
/// request were a single, filled shape.
///
/// # Fields
///
/// * `drawable` - The drawable to draw the line(s) on.
/// * `gc` - The graphics context to use.
/// * `points` - An array of points.
/// * `points_len` - The number of `xcb_point_t` structures in `points`.
///
/// # Errors
///
/// * `Drawable` - TODO: reasons?
/// * `GContext` - TODO: reasons?
/// * `Match` - TODO: reasons?
/// * `Value` - TODO: reasons?
///
/// # Example
///
/// ```text
/// /*
///  * Draw a straight line.
///  *
///  */
/// void my_example(xcb_connection_t *conn, xcb_drawable_t drawable, xcb_gcontext_t gc) {
///     xcb_poly_line(conn, XCB_COORD_MODE_ORIGIN, drawable, gc, 2,
///                   (xcb_point_t[]) { {10, 10}, {100, 10} });
///     xcb_flush(conn);
/// }
/// ```
pub fn poly_line<'c, Conn, A>(conn: &'c Conn, coordinate_mode: A, drawable: Drawable, gc: Gcontext, points: &[Point]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12 + 4 * points.len() + 3) / 4;
    let coordinate_mode = coordinate_mode.into();
    let coordinate_mode_bytes = coordinate_mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let points_bytes = points.serialize();
    let request0 = [
        POLY_LINE_REQUEST,
        coordinate_mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&points_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&points_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}
impl TryParse for Segment {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x1, remaining) = i16::try_parse(remaining)?;
        let (y1, remaining) = i16::try_parse(remaining)?;
        let (x2, remaining) = i16::try_parse(remaining)?;
        let (y2, remaining) = i16::try_parse(remaining)?;
        let result = Segment { x1, y1, x2, y2 };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Segment {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Segment {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let x1_bytes = self.x1.serialize();
        let y1_bytes = self.y1.serialize();
        let x2_bytes = self.x2.serialize();
        let y2_bytes = self.y2.serialize();
        [
            x1_bytes[0],
            x1_bytes[1],
            y1_bytes[0],
            y1_bytes[1],
            x2_bytes[0],
            x2_bytes[1],
            y2_bytes[0],
            y2_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.x1.serialize_into(bytes);
        self.y1.serialize_into(bytes);
        self.x2.serialize_into(bytes);
        self.y2.serialize_into(bytes);
    }
}

/// Opcode for the PolySegment request
pub const POLY_SEGMENT_REQUEST: u8 = 66;
/// draw lines.
///
/// Draws multiple, unconnected lines. For each segment, a line is drawn between
/// (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
/// `xcb_segment_t` structures and does not perform joining at coincident
/// endpoints. For any given line, a pixel is not drawn more than once. If lines
/// intersect, the intersecting pixels are drawn multiple times.
///
/// TODO: include the xcb_segment_t data structure
///
/// TODO: an example
///
/// # Fields
///
/// * `drawable` - A drawable (Window or Pixmap) to draw on.
/// * `gc` - The graphics context to use.
///
/// TODO: document which attributes of a gc are used
/// * `segments` - An array of `xcb_segment_t` structures.
/// * `segments_len` - The number of `xcb_segment_t` structures in `segments`.
///
/// # Errors
///
/// * `Drawable` - The specified `drawable` does not exist.
/// * `GContext` - The specified `gc` does not exist.
/// * `Match` - TODO: reasons?
pub fn poly_segment<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, segments: &[Segment]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 8 * segments.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let segments_bytes = segments.serialize();
    let request0 = [
        POLY_SEGMENT_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&segments_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&segments_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyRectangle request
pub const POLY_RECTANGLE_REQUEST: u8 = 67;
pub fn poly_rectangle<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 8 * rectangles.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        POLY_RECTANGLE_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyArc request
pub const POLY_ARC_REQUEST: u8 = 68;
pub fn poly_arc<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, arcs: &[Arc]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 12 * arcs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let arcs_bytes = arcs.serialize();
    let request0 = [
        POLY_ARC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&arcs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&arcs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PolyShape {
    Complex = 0,
    Nonconvex = 1,
    Convex = 2,
}
impl From<PolyShape> for u8 {
    fn from(input: PolyShape) -> Self {
        match input {
            PolyShape::Complex => 0,
            PolyShape::Nonconvex => 1,
            PolyShape::Convex => 2,
        }
    }
}
impl From<PolyShape> for Option<u8> {
    fn from(input: PolyShape) -> Self {
        Some(u8::from(input))
    }
}
impl From<PolyShape> for u16 {
    fn from(input: PolyShape) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyShape> for Option<u16> {
    fn from(input: PolyShape) -> Self {
        Some(u16::from(input))
    }
}
impl From<PolyShape> for u32 {
    fn from(input: PolyShape) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PolyShape> for Option<u32> {
    fn from(input: PolyShape) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PolyShape {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PolyShape::Complex),
            1 => Ok(PolyShape::Nonconvex),
            2 => Ok(PolyShape::Convex),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PolyShape {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PolyShape {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the FillPoly request
pub const FILL_POLY_REQUEST: u8 = 69;
pub fn fill_poly<'c, Conn, A, B>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, shape: A, coordinate_mode: B, points: &[Point]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (16 + 4 * points.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let shape = shape.into();
    let shape_bytes = shape.serialize();
    let coordinate_mode = coordinate_mode.into();
    let coordinate_mode_bytes = coordinate_mode.serialize();
    let points_bytes = points.serialize();
    let request0 = [
        FILL_POLY_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        shape_bytes[0],
        coordinate_mode_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&points_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&points_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyFillRectangle request
pub const POLY_FILL_RECTANGLE_REQUEST: u8 = 70;
/// Fills rectangles.
///
/// Fills the specified rectangle(s) in the order listed in the array. For any
/// given rectangle, each pixel is not drawn more than once. If rectangles
/// intersect, the intersecting pixels are drawn multiple times.
///
/// # Fields
///
/// * `drawable` - The drawable (Window or Pixmap) to draw on.
/// * `gc` - The graphics context to use.
///
/// The following graphics context components are used: function, plane-mask,
/// fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
///
/// The following graphics context mode-dependent components are used:
/// foreground, background, tile, stipple, tile-stipple-x-origin, and
/// tile-stipple-y-origin.
/// * `rectangles` - The rectangles to fill.
/// * `rectangles_len` - The number of `xcb_rectangle_t` structures in `rectangles`.
///
/// # Errors
///
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `GContext` - The specified graphics context does not exist.
/// * `Match` - TODO: reasons?
pub fn poly_fill_rectangle<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 8 * rectangles.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let rectangles_bytes = rectangles.serialize();
    let request0 = [
        POLY_FILL_RECTANGLE_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&rectangles_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&rectangles_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyFillArc request
pub const POLY_FILL_ARC_REQUEST: u8 = 71;
pub fn poly_fill_arc<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, arcs: &[Arc]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 12 * arcs.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let arcs_bytes = arcs.serialize();
    let request0 = [
        POLY_FILL_ARC_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&arcs_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&arcs_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageFormat {
    XYBitmap = 0,
    XYPixmap = 1,
    ZPixmap = 2,
}
impl From<ImageFormat> for u8 {
    fn from(input: ImageFormat) -> Self {
        match input {
            ImageFormat::XYBitmap => 0,
            ImageFormat::XYPixmap => 1,
            ImageFormat::ZPixmap => 2,
        }
    }
}
impl From<ImageFormat> for Option<u8> {
    fn from(input: ImageFormat) -> Self {
        Some(u8::from(input))
    }
}
impl From<ImageFormat> for u16 {
    fn from(input: ImageFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormat> for Option<u16> {
    fn from(input: ImageFormat) -> Self {
        Some(u16::from(input))
    }
}
impl From<ImageFormat> for u32 {
    fn from(input: ImageFormat) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ImageFormat> for Option<u32> {
    fn from(input: ImageFormat) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ImageFormat {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageFormat::XYBitmap),
            1 => Ok(ImageFormat::XYPixmap),
            2 => Ok(ImageFormat::ZPixmap),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ImageFormat {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ImageFormat {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the PutImage request
pub const PUT_IMAGE_REQUEST: u8 = 72;
pub fn put_image<'c, Conn, A>(conn: &'c Conn, format: A, drawable: Drawable, gc: Gcontext, width: u16, height: u16, dst_x: i16, dst_y: i16, left_pad: u8, depth: u8, data: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (24 + 1 * data.len() + 3) / 4;
    let format = format.into();
    let format_bytes = format.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let dst_x_bytes = dst_x.serialize();
    let dst_y_bytes = dst_y.serialize();
    let left_pad_bytes = left_pad.serialize();
    let depth_bytes = depth.serialize();
    let request0 = [
        PUT_IMAGE_REQUEST,
        format_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        dst_x_bytes[0],
        dst_x_bytes[1],
        dst_y_bytes[0],
        dst_y_bytes[1],
        left_pad_bytes[0],
        depth_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (data).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(data), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetImage request
pub const GET_IMAGE_REQUEST: u8 = 73;
pub fn get_image<Conn, A>(conn: &Conn, format: A, drawable: Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32) -> Result<Cookie<'_, Conn, GetImageReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (20) / 4;
    let format = format.into();
    let format_bytes = format.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let plane_mask_bytes = plane_mask.serialize();
    let request0 = [
        GET_IMAGE_REQUEST,
        format_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
        plane_mask_bytes[0],
        plane_mask_bytes[1],
        plane_mask_bytes[2],
        plane_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetImageReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub visual: Visualid,
    pub data: Vec<u8>,
}
impl GetImageReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (visual, remaining) = Visualid::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (data, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (length as usize) * (4))?;
        let result = GetImageReply { response_type, depth, sequence, visual, data };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetImageReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the PolyText8 request
pub const POLY_TEXT8_REQUEST: u8 = 74;
pub fn poly_text8<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, x: i16, y: i16, items: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16 + 1 * items.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        POLY_TEXT8_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (items).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(items), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the PolyText16 request
pub const POLY_TEXT16_REQUEST: u8 = 75;
pub fn poly_text16<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, x: i16, y: i16, items: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16 + 1 * items.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        POLY_TEXT16_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (items).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(items), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ImageText8 request
pub const IMAGE_TEXT8_REQUEST: u8 = 76;
/// Draws text.
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// # Fields
///
/// * `drawable` - The drawable (Window or Pixmap) to draw text on.
/// * `gc` - The graphics context to use.
///
/// The following graphics context components are used: plane-mask, foreground,
/// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
/// * `string` - The string to draw. Only the first 255 characters are relevant due to the data
/// type of `string_len`.
/// * `string_len` - The length of the `string`. Note that this parameter limited by 255 due to
/// using 8 bits!
/// * `x` - The x coordinate of the first character, relative to the origin of `drawable`.
/// * `y` - The y coordinate of the first character, relative to the origin of `drawable`.
///
/// # Errors
///
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `GContext` - The specified graphics context does not exist.
/// * `Match` - TODO: reasons?
///
/// # See
///
/// * ImageText16: request
pub fn image_text8<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, x: i16, y: i16, string: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16 + 1 * string.len() + 3) / 4;
    let string_len: u8 = string.len().try_into()?;
    let string_len_bytes = string_len.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        IMAGE_TEXT8_REQUEST,
        string_len_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (string).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(string), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ImageText16 request
pub const IMAGE_TEXT16_REQUEST: u8 = 77;
/// Draws text.
///
/// Fills the destination rectangle with the background pixel from `gc`, then
/// paints the text with the foreground pixel from `gc`. The upper-left corner of
/// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
/// the height is font-ascent + font-descent. The overall-width, font-ascent and
/// font-descent are as returned by `xcb_query_text_extents` (TODO).
///
/// Note that using X core fonts is deprecated (but still supported) in favor of
/// client-side rendering using Xft.
///
/// # Fields
///
/// * `drawable` - The drawable (Window or Pixmap) to draw text on.
/// * `gc` - The graphics context to use.
///
/// The following graphics context components are used: plane-mask, foreground,
/// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
/// * `string` - The string to draw. Only the first 255 characters are relevant due to the data
/// type of `string_len`. Every character uses 2 bytes (hence the 16 in this
/// request's name).
/// * `string_len` - The length of the `string` in characters. Note that this parameter limited by
/// 255 due to using 8 bits!
/// * `x` - The x coordinate of the first character, relative to the origin of `drawable`.
/// * `y` - The y coordinate of the first character, relative to the origin of `drawable`.
///
/// # Errors
///
/// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
/// * `GContext` - The specified graphics context does not exist.
/// * `Match` - TODO: reasons?
///
/// # See
///
/// * ImageText8: request
pub fn image_text16<'c, Conn>(conn: &'c Conn, drawable: Drawable, gc: Gcontext, x: i16, y: i16, string: &[Char2b]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16 + 2 * string.len() + 3) / 4;
    let string_len: u8 = string.len().try_into()?;
    let string_len_bytes = string_len.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let gc_bytes = gc.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let string_bytes = string.serialize();
    let request0 = [
        IMAGE_TEXT16_REQUEST,
        string_len_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        gc_bytes[0],
        gc_bytes[1],
        gc_bytes[2],
        gc_bytes[3],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&string_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&string_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColormapAlloc {
    None = 0,
    All = 1,
}
impl From<ColormapAlloc> for u8 {
    fn from(input: ColormapAlloc) -> Self {
        match input {
            ColormapAlloc::None => 0,
            ColormapAlloc::All => 1,
        }
    }
}
impl From<ColormapAlloc> for Option<u8> {
    fn from(input: ColormapAlloc) -> Self {
        Some(u8::from(input))
    }
}
impl From<ColormapAlloc> for u16 {
    fn from(input: ColormapAlloc) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapAlloc> for Option<u16> {
    fn from(input: ColormapAlloc) -> Self {
        Some(u16::from(input))
    }
}
impl From<ColormapAlloc> for u32 {
    fn from(input: ColormapAlloc) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColormapAlloc> for Option<u32> {
    fn from(input: ColormapAlloc) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ColormapAlloc {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColormapAlloc::None),
            1 => Ok(ColormapAlloc::All),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ColormapAlloc {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ColormapAlloc {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CreateColormap request
pub const CREATE_COLORMAP_REQUEST: u8 = 78;
pub fn create_colormap<Conn, A>(conn: &Conn, alloc: A, mid: Colormap, window: Window, visual: Visualid) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (16) / 4;
    let alloc = alloc.into();
    let alloc_bytes = alloc.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let mid_bytes = mid.serialize();
    let window_bytes = window.serialize();
    let visual_bytes = visual.serialize();
    let request0 = [
        CREATE_COLORMAP_REQUEST,
        alloc_bytes[0],
        length_bytes[0],
        length_bytes[1],
        mid_bytes[0],
        mid_bytes[1],
        mid_bytes[2],
        mid_bytes[3],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        visual_bytes[0],
        visual_bytes[1],
        visual_bytes[2],
        visual_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the FreeColormap request
pub const FREE_COLORMAP_REQUEST: u8 = 79;
pub fn free_colormap<Conn>(conn: &Conn, cmap: Colormap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let request0 = [
        FREE_COLORMAP_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CopyColormapAndFree request
pub const COPY_COLORMAP_AND_FREE_REQUEST: u8 = 80;
pub fn copy_colormap_and_free<Conn>(conn: &Conn, mid: Colormap, src_cmap: Colormap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let mid_bytes = mid.serialize();
    let src_cmap_bytes = src_cmap.serialize();
    let request0 = [
        COPY_COLORMAP_AND_FREE_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        mid_bytes[0],
        mid_bytes[1],
        mid_bytes[2],
        mid_bytes[3],
        src_cmap_bytes[0],
        src_cmap_bytes[1],
        src_cmap_bytes[2],
        src_cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the InstallColormap request
pub const INSTALL_COLORMAP_REQUEST: u8 = 81;
pub fn install_colormap<Conn>(conn: &Conn, cmap: Colormap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let request0 = [
        INSTALL_COLORMAP_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the UninstallColormap request
pub const UNINSTALL_COLORMAP_REQUEST: u8 = 82;
pub fn uninstall_colormap<Conn>(conn: &Conn, cmap: Colormap) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let request0 = [
        UNINSTALL_COLORMAP_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ListInstalledColormaps request
pub const LIST_INSTALLED_COLORMAPS_REQUEST: u8 = 83;
pub fn list_installed_colormaps<Conn>(conn: &Conn, window: Window) -> Result<Cookie<'_, Conn, ListInstalledColormapsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let request0 = [
        LIST_INSTALLED_COLORMAPS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListInstalledColormapsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps: Vec<Colormap>,
}
impl ListInstalledColormapsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (cmaps_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (cmaps, remaining) = crate::x11_utils::parse_list::<Colormap>(remaining, cmaps_len as usize)?;
        let result = ListInstalledColormapsReply { response_type, sequence, length, cmaps };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListInstalledColormapsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the AllocColor request
pub const ALLOC_COLOR_REQUEST: u8 = 84;
/// Allocate a color.
///
/// Allocates a read-only colormap entry corresponding to the closest RGB value
/// supported by the hardware. If you are using TrueColor, you can take a shortcut
/// and directly calculate the color pixel value to avoid the round trip. But, for
/// example, on 16-bit color setups (VNC), you can easily get the closest supported
/// RGB value to the RGB value you are specifying.
///
/// # Fields
///
/// * `blue` - The blue value of your color.
/// * `cmap` - TODO
/// * `green` - The green value of your color.
/// * `red` - The red value of your color.
///
/// # Errors
///
/// * `Colormap` - The specified colormap `cmap` does not exist.
pub fn alloc_color<Conn>(conn: &Conn, cmap: Colormap, red: u16, green: u16, blue: u16) -> Result<Cookie<'_, Conn, AllocColorReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let red_bytes = red.serialize();
    let green_bytes = green.serialize();
    let blue_bytes = blue.serialize();
    let request0 = [
        ALLOC_COLOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        red_bytes[0],
        red_bytes[1],
        green_bytes[0],
        green_bytes[1],
        blue_bytes[0],
        blue_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pixel: u32,
}
impl AllocColorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (red, remaining) = u16::try_parse(remaining)?;
        let (green, remaining) = u16::try_parse(remaining)?;
        let (blue, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (pixel, remaining) = u32::try_parse(remaining)?;
        let result = AllocColorReply { response_type, sequence, length, red, green, blue, pixel };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AllocColorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the AllocNamedColor request
pub const ALLOC_NAMED_COLOR_REQUEST: u8 = 85;
pub fn alloc_named_color<'c, Conn>(conn: &'c Conn, cmap: Colormap, name: &[u8]) -> Result<Cookie<'c, Conn, AllocNamedColorReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 1 * name.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        ALLOC_NAMED_COLOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocNamedColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
impl AllocNamedColorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pixel, remaining) = u32::try_parse(remaining)?;
        let (exact_red, remaining) = u16::try_parse(remaining)?;
        let (exact_green, remaining) = u16::try_parse(remaining)?;
        let (exact_blue, remaining) = u16::try_parse(remaining)?;
        let (visual_red, remaining) = u16::try_parse(remaining)?;
        let (visual_green, remaining) = u16::try_parse(remaining)?;
        let (visual_blue, remaining) = u16::try_parse(remaining)?;
        let result = AllocNamedColorReply { response_type, sequence, length, pixel, exact_red, exact_green, exact_blue, visual_red, visual_green, visual_blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AllocNamedColorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the AllocColorCells request
pub const ALLOC_COLOR_CELLS_REQUEST: u8 = 86;
pub fn alloc_color_cells<Conn>(conn: &Conn, contiguous: bool, cmap: Colormap, colors: u16, planes: u16) -> Result<Cookie<'_, Conn, AllocColorCellsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12) / 4;
    let contiguous_bytes = (contiguous as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let colors_bytes = colors.serialize();
    let planes_bytes = planes.serialize();
    let request0 = [
        ALLOC_COLOR_CELLS_REQUEST,
        contiguous_bytes[0],
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        colors_bytes[0],
        colors_bytes[1],
        planes_bytes[0],
        planes_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocColorCellsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels: Vec<u32>,
    pub masks: Vec<u32>,
}
impl AllocColorCellsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pixels_len, remaining) = u16::try_parse(remaining)?;
        let (masks_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (pixels, remaining) = crate::x11_utils::parse_list::<u32>(remaining, pixels_len as usize)?;
        let (masks, remaining) = crate::x11_utils::parse_list::<u32>(remaining, masks_len as usize)?;
        let result = AllocColorCellsReply { response_type, sequence, length, pixels, masks };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AllocColorCellsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the AllocColorPlanes request
pub const ALLOC_COLOR_PLANES_REQUEST: u8 = 87;
pub fn alloc_color_planes<Conn>(conn: &Conn, contiguous: bool, cmap: Colormap, colors: u16, reds: u16, greens: u16, blues: u16) -> Result<Cookie<'_, Conn, AllocColorPlanesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16) / 4;
    let contiguous_bytes = (contiguous as u8).serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let colors_bytes = colors.serialize();
    let reds_bytes = reds.serialize();
    let greens_bytes = greens.serialize();
    let blues_bytes = blues.serialize();
    let request0 = [
        ALLOC_COLOR_PLANES_REQUEST,
        contiguous_bytes[0],
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        colors_bytes[0],
        colors_bytes[1],
        reds_bytes[0],
        reds_bytes[1],
        greens_bytes[0],
        greens_bytes[1],
        blues_bytes[0],
        blues_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocColorPlanesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pixels: Vec<u32>,
}
impl AllocColorPlanesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (pixels_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (red_mask, remaining) = u32::try_parse(remaining)?;
        let (green_mask, remaining) = u32::try_parse(remaining)?;
        let (blue_mask, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(8..).ok_or(ParseError::ParseError)?;
        let (pixels, remaining) = crate::x11_utils::parse_list::<u32>(remaining, pixels_len as usize)?;
        let result = AllocColorPlanesReply { response_type, sequence, length, red_mask, green_mask, blue_mask, pixels };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AllocColorPlanesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the FreeColors request
pub const FREE_COLORS_REQUEST: u8 = 88;
pub fn free_colors<'c, Conn>(conn: &'c Conn, cmap: Colormap, plane_mask: u32, pixels: &[u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 4 * pixels.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let plane_mask_bytes = plane_mask.serialize();
    let pixels_bytes = pixels.serialize();
    let request0 = [
        FREE_COLORS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        plane_mask_bytes[0],
        plane_mask_bytes[1],
        plane_mask_bytes[2],
        plane_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&pixels_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&pixels_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorFlag {
    Red = 1 << 0,
    Green = 1 << 1,
    Blue = 1 << 2,
}
impl From<ColorFlag> for u8 {
    fn from(input: ColorFlag) -> Self {
        match input {
            ColorFlag::Red => 1 << 0,
            ColorFlag::Green => 1 << 1,
            ColorFlag::Blue => 1 << 2,
        }
    }
}
impl From<ColorFlag> for Option<u8> {
    fn from(input: ColorFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<ColorFlag> for u16 {
    fn from(input: ColorFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColorFlag> for Option<u16> {
    fn from(input: ColorFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<ColorFlag> for u32 {
    fn from(input: ColorFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ColorFlag> for Option<u32> {
    fn from(input: ColorFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ColorFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ColorFlag::Red),
            2 => Ok(ColorFlag::Green),
            4 => Ok(ColorFlag::Blue),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ColorFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ColorFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ColorFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coloritem {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
}
impl TryParse for Coloritem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (pixel, remaining) = u32::try_parse(remaining)?;
        let (red, remaining) = u16::try_parse(remaining)?;
        let (green, remaining) = u16::try_parse(remaining)?;
        let (blue, remaining) = u16::try_parse(remaining)?;
        let (flags, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = Coloritem { pixel, red, green, blue, flags };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Coloritem {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Coloritem {
    type Bytes = [u8; 12];
    fn serialize(&self) -> Self::Bytes {
        let pixel_bytes = self.pixel.serialize();
        let red_bytes = self.red.serialize();
        let green_bytes = self.green.serialize();
        let blue_bytes = self.blue.serialize();
        let flags_bytes = self.flags.serialize();
        [
            pixel_bytes[0],
            pixel_bytes[1],
            pixel_bytes[2],
            pixel_bytes[3],
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            flags_bytes[0],
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.pixel.serialize_into(bytes);
        self.red.serialize_into(bytes);
        self.green.serialize_into(bytes);
        self.blue.serialize_into(bytes);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
    }
}

/// Opcode for the StoreColors request
pub const STORE_COLORS_REQUEST: u8 = 89;
pub fn store_colors<'c, Conn>(conn: &'c Conn, cmap: Colormap, items: &[Coloritem]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 12 * items.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let items_bytes = items.serialize();
    let request0 = [
        STORE_COLORS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&items_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&items_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the StoreNamedColor request
pub const STORE_NAMED_COLOR_REQUEST: u8 = 90;
pub fn store_named_color<'c, Conn>(conn: &'c Conn, flags: u8, cmap: Colormap, pixel: u32, name: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (16 + 1 * name.len() + 3) / 4;
    let flags_bytes = flags.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let pixel_bytes = pixel.serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        STORE_NAMED_COLOR_REQUEST,
        flags_bytes[0],
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        pixel_bytes[0],
        pixel_bytes[1],
        pixel_bytes[2],
        pixel_bytes[3],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl TryParse for Rgb {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (red, remaining) = u16::try_parse(remaining)?;
        let (green, remaining) = u16::try_parse(remaining)?;
        let (blue, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let result = Rgb { red, green, blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Rgb {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Rgb {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let red_bytes = self.red.serialize();
        let green_bytes = self.green.serialize();
        let blue_bytes = self.blue.serialize();
        [
            red_bytes[0],
            red_bytes[1],
            green_bytes[0],
            green_bytes[1],
            blue_bytes[0],
            blue_bytes[1],
            0,
            0,
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.red.serialize_into(bytes);
        self.green.serialize_into(bytes);
        self.blue.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 2]);
    }
}

/// Opcode for the QueryColors request
pub const QUERY_COLORS_REQUEST: u8 = 91;
pub fn query_colors<'c, Conn>(conn: &'c Conn, cmap: Colormap, pixels: &[u32]) -> Result<Cookie<'c, Conn, QueryColorsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 4 * pixels.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let pixels_bytes = pixels.serialize();
    let request0 = [
        QUERY_COLORS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&pixels_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(&pixels_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryColorsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors: Vec<Rgb>,
}
impl QueryColorsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (colors_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (colors, remaining) = crate::x11_utils::parse_list::<Rgb>(remaining, colors_len as usize)?;
        let result = QueryColorsReply { response_type, sequence, length, colors };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryColorsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the LookupColor request
pub const LOOKUP_COLOR_REQUEST: u8 = 92;
pub fn lookup_color<'c, Conn>(conn: &'c Conn, cmap: Colormap, name: &[u8]) -> Result<Cookie<'c, Conn, LookupColorReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 1 * name.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cmap_bytes = cmap.serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        LOOKUP_COLOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cmap_bytes[0],
        cmap_bytes[1],
        cmap_bytes[2],
        cmap_bytes[3],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LookupColorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}
impl LookupColorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (exact_red, remaining) = u16::try_parse(remaining)?;
        let (exact_green, remaining) = u16::try_parse(remaining)?;
        let (exact_blue, remaining) = u16::try_parse(remaining)?;
        let (visual_red, remaining) = u16::try_parse(remaining)?;
        let (visual_green, remaining) = u16::try_parse(remaining)?;
        let (visual_blue, remaining) = u16::try_parse(remaining)?;
        let result = LookupColorReply { response_type, sequence, length, exact_red, exact_green, exact_blue, visual_red, visual_green, visual_blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for LookupColorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixmapEnum {
    None = 0,
}
impl From<PixmapEnum> for u8 {
    fn from(input: PixmapEnum) -> Self {
        match input {
            PixmapEnum::None => 0,
        }
    }
}
impl From<PixmapEnum> for Option<u8> {
    fn from(input: PixmapEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<PixmapEnum> for u16 {
    fn from(input: PixmapEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PixmapEnum> for Option<u16> {
    fn from(input: PixmapEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<PixmapEnum> for u32 {
    fn from(input: PixmapEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<PixmapEnum> for Option<u32> {
    fn from(input: PixmapEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for PixmapEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PixmapEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for PixmapEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for PixmapEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CreateCursor request
pub const CREATE_CURSOR_REQUEST: u8 = 93;
pub fn create_cursor<Conn>(conn: &Conn, cid: Cursor, source: Pixmap, mask: Pixmap, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16, x: u16, y: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (32) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cid_bytes = cid.serialize();
    let source_bytes = source.serialize();
    let mask_bytes = mask.serialize();
    let fore_red_bytes = fore_red.serialize();
    let fore_green_bytes = fore_green.serialize();
    let fore_blue_bytes = fore_blue.serialize();
    let back_red_bytes = back_red.serialize();
    let back_green_bytes = back_green.serialize();
    let back_blue_bytes = back_blue.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        CREATE_CURSOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cid_bytes[0],
        cid_bytes[1],
        cid_bytes[2],
        cid_bytes[3],
        source_bytes[0],
        source_bytes[1],
        source_bytes[2],
        source_bytes[3],
        mask_bytes[0],
        mask_bytes[1],
        mask_bytes[2],
        mask_bytes[3],
        fore_red_bytes[0],
        fore_red_bytes[1],
        fore_green_bytes[0],
        fore_green_bytes[1],
        fore_blue_bytes[0],
        fore_blue_bytes[1],
        back_red_bytes[0],
        back_red_bytes[1],
        back_green_bytes[0],
        back_green_bytes[1],
        back_blue_bytes[0],
        back_blue_bytes[1],
        x_bytes[0],
        x_bytes[1],
        y_bytes[0],
        y_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FontEnum {
    None = 0,
}
impl From<FontEnum> for u8 {
    fn from(input: FontEnum) -> Self {
        match input {
            FontEnum::None => 0,
        }
    }
}
impl From<FontEnum> for Option<u8> {
    fn from(input: FontEnum) -> Self {
        Some(u8::from(input))
    }
}
impl From<FontEnum> for u16 {
    fn from(input: FontEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FontEnum> for Option<u16> {
    fn from(input: FontEnum) -> Self {
        Some(u16::from(input))
    }
}
impl From<FontEnum> for u32 {
    fn from(input: FontEnum) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<FontEnum> for Option<u32> {
    fn from(input: FontEnum) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for FontEnum {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FontEnum::None),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for FontEnum {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for FontEnum {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the CreateGlyphCursor request
pub const CREATE_GLYPH_CURSOR_REQUEST: u8 = 94;
/// create cursor.
///
/// Creates a cursor from a font glyph. X provides a set of standard cursor shapes
/// in a special font named cursor. Applications are encouraged to use this
/// interface for their cursors because the font can be customized for the
/// individual display type.
///
/// All pixels which are set to 1 in the source will use the foreground color (as
/// specified by `fore_red`, `fore_green` and `fore_blue`). All pixels set to 0
/// will use the background color (as specified by `back_red`, `back_green` and
/// `back_blue`).
///
/// # Fields
///
/// * `back_blue` - The blue value of the background color.
/// * `back_green` - The green value of the background color.
/// * `back_red` - The red value of the background color.
/// * `cid` - The ID with which you will refer to the cursor, created by `xcb_generate_id`.
/// * `fore_blue` - The blue value of the foreground color.
/// * `fore_green` - The green value of the foreground color.
/// * `fore_red` - The red value of the foreground color.
/// * `mask_char` - The glyph of `mask_font` to use as a mask: Pixels which are set to 1 define
/// which source pixels are displayed. All pixels which are set to 0 are not
/// displayed.
/// * `mask_font` - In which font to look for the mask glyph.
/// * `source_char` - The glyph of `source_font` to use.
/// * `source_font` - In which font to look for the cursor glyph.
///
/// # Errors
///
/// * `Alloc` - The X server could not allocate the requested resources (no memory?).
/// * `Font` - The specified `source_font` or `mask_font` does not exist.
/// * `Value` - Either `source_char` or `mask_char` are not defined in `source_font` or `mask_font`, respectively.
pub fn create_glyph_cursor<Conn>(conn: &Conn, cid: Cursor, source_font: Font, mask_font: Font, source_char: u16, mask_char: u16, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (32) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cid_bytes = cid.serialize();
    let source_font_bytes = source_font.serialize();
    let mask_font_bytes = mask_font.serialize();
    let source_char_bytes = source_char.serialize();
    let mask_char_bytes = mask_char.serialize();
    let fore_red_bytes = fore_red.serialize();
    let fore_green_bytes = fore_green.serialize();
    let fore_blue_bytes = fore_blue.serialize();
    let back_red_bytes = back_red.serialize();
    let back_green_bytes = back_green.serialize();
    let back_blue_bytes = back_blue.serialize();
    let request0 = [
        CREATE_GLYPH_CURSOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cid_bytes[0],
        cid_bytes[1],
        cid_bytes[2],
        cid_bytes[3],
        source_font_bytes[0],
        source_font_bytes[1],
        source_font_bytes[2],
        source_font_bytes[3],
        mask_font_bytes[0],
        mask_font_bytes[1],
        mask_font_bytes[2],
        mask_font_bytes[3],
        source_char_bytes[0],
        source_char_bytes[1],
        mask_char_bytes[0],
        mask_char_bytes[1],
        fore_red_bytes[0],
        fore_red_bytes[1],
        fore_green_bytes[0],
        fore_green_bytes[1],
        fore_blue_bytes[0],
        fore_blue_bytes[1],
        back_red_bytes[0],
        back_red_bytes[1],
        back_green_bytes[0],
        back_green_bytes[1],
        back_blue_bytes[0],
        back_blue_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the FreeCursor request
pub const FREE_CURSOR_REQUEST: u8 = 95;
/// Deletes a cursor.
///
/// Deletes the association between the cursor resource ID and the specified
/// cursor. The cursor is freed when no other resource references it.
///
/// # Fields
///
/// * `cursor` - The cursor to destroy.
///
/// # Errors
///
/// * `Cursor` - The specified cursor does not exist.
pub fn free_cursor<Conn>(conn: &Conn, cursor: Cursor) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cursor_bytes = cursor.serialize();
    let request0 = [
        FREE_CURSOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the RecolorCursor request
pub const RECOLOR_CURSOR_REQUEST: u8 = 96;
pub fn recolor_cursor<Conn>(conn: &Conn, cursor: Cursor, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (20) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let cursor_bytes = cursor.serialize();
    let fore_red_bytes = fore_red.serialize();
    let fore_green_bytes = fore_green.serialize();
    let fore_blue_bytes = fore_blue.serialize();
    let back_red_bytes = back_red.serialize();
    let back_green_bytes = back_green.serialize();
    let back_blue_bytes = back_blue.serialize();
    let request0 = [
        RECOLOR_CURSOR_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        cursor_bytes[0],
        cursor_bytes[1],
        cursor_bytes[2],
        cursor_bytes[3],
        fore_red_bytes[0],
        fore_red_bytes[1],
        fore_green_bytes[0],
        fore_green_bytes[1],
        fore_blue_bytes[0],
        fore_blue_bytes[1],
        back_red_bytes[0],
        back_red_bytes[1],
        back_green_bytes[0],
        back_green_bytes[1],
        back_blue_bytes[0],
        back_blue_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QueryShapeOf {
    LargestCursor = 0,
    FastestTile = 1,
    FastestStipple = 2,
}
impl From<QueryShapeOf> for u8 {
    fn from(input: QueryShapeOf) -> Self {
        match input {
            QueryShapeOf::LargestCursor => 0,
            QueryShapeOf::FastestTile => 1,
            QueryShapeOf::FastestStipple => 2,
        }
    }
}
impl From<QueryShapeOf> for Option<u8> {
    fn from(input: QueryShapeOf) -> Self {
        Some(u8::from(input))
    }
}
impl From<QueryShapeOf> for u16 {
    fn from(input: QueryShapeOf) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<QueryShapeOf> for Option<u16> {
    fn from(input: QueryShapeOf) -> Self {
        Some(u16::from(input))
    }
}
impl From<QueryShapeOf> for u32 {
    fn from(input: QueryShapeOf) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<QueryShapeOf> for Option<u32> {
    fn from(input: QueryShapeOf) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for QueryShapeOf {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QueryShapeOf::LargestCursor),
            1 => Ok(QueryShapeOf::FastestTile),
            2 => Ok(QueryShapeOf::FastestStipple),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for QueryShapeOf {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for QueryShapeOf {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the QueryBestSize request
pub const QUERY_BEST_SIZE_REQUEST: u8 = 97;
pub fn query_best_size<Conn, A>(conn: &Conn, class: A, drawable: Drawable, width: u16, height: u16) -> Result<Cookie<'_, Conn, QueryBestSizeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (12) / 4;
    let class = class.into();
    let class_bytes = class.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let width_bytes = width.serialize();
    let height_bytes = height.serialize();
    let request0 = [
        QUERY_BEST_SIZE_REQUEST,
        class_bytes[0],
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        width_bytes[0],
        width_bytes[1],
        height_bytes[0],
        height_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryBestSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
}
impl QueryBestSizeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let result = QueryBestSizeReply { response_type, sequence, length, width, height };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryBestSizeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the QueryExtension request
pub const QUERY_EXTENSION_REQUEST: u8 = 98;
/// check if extension is present.
///
/// Determines if the specified extension is present on this X11 server.
///
/// Every extension has a unique `major_opcode` to identify requests, the minor
/// opcodes and request formats are extension-specific. If the extension provides
/// events and errors, the `first_event` and `first_error` fields in the reply are
/// set accordingly.
///
/// There should rarely be a need to use this request directly, XCB provides the
/// `xcb_get_extension_data` function instead.
///
/// # Fields
///
/// * `name` - The name of the extension to query, for example "RANDR". This is case
/// sensitive!
/// * `name_len` - The length of `name` in bytes.
///
/// # See
///
/// * xcb_get_extension_data: function
/// * xdpyinfo: program
pub fn query_extension<'c, Conn>(conn: &'c Conn, name: &[u8]) -> Result<Cookie<'c, Conn, QueryExtensionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 1 * name.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let name_len: u16 = name.len().try_into()?;
    let name_len_bytes = name_len.serialize();
    let request0 = [
        QUERY_EXTENSION_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        name_len_bytes[0],
        name_len_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (name).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(name), IoSlice::new(&padding1)], Vec::new())?)
}
/// BRIEF DESCRIPTION MISSING.
///
///
/// # Fields
///
/// * `first_error` - The first error code, if any.
/// * `first_event` - The first event code, if any.
/// * `major_opcode` - The major opcode for requests.
/// * `present` - Whether the extension is present on this X11 server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryExtensionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: bool,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}
impl QueryExtensionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (present, remaining) = bool::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let (first_event, remaining) = u8::try_parse(remaining)?;
        let (first_error, remaining) = u8::try_parse(remaining)?;
        let result = QueryExtensionReply { response_type, sequence, length, present, major_opcode, first_event, first_error };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryExtensionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ListExtensions request
pub const LIST_EXTENSIONS_REQUEST: u8 = 99;
pub fn list_extensions<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListExtensionsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        LIST_EXTENSIONS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListExtensionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: Vec<Str>,
}
impl ListExtensionsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (names_len, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (names, remaining) = crate::x11_utils::parse_list::<Str>(remaining, names_len as usize)?;
        let result = ListExtensionsReply { response_type, sequence, length, names };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListExtensionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ChangeKeyboardMapping request
pub const CHANGE_KEYBOARD_MAPPING_REQUEST: u8 = 100;
pub fn change_keyboard_mapping<'c, Conn>(conn: &'c Conn, keycode_count: u8, first_keycode: Keycode, keysyms_per_keycode: u8, keysyms: &[Keysym]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8 + 4 * keysyms.len() + 3) / 4;
    let keycode_count_bytes = keycode_count.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let first_keycode_bytes = first_keycode.serialize();
    let keysyms_per_keycode_bytes = keysyms_per_keycode.serialize();
    assert_eq!(keysyms.len(), (keycode_count as usize) * (keysyms_per_keycode as usize), "Argument keysyms has an incorrect length");
    let keysyms_bytes = keysyms.serialize();
    let request0 = [
        CHANGE_KEYBOARD_MAPPING_REQUEST,
        keycode_count_bytes[0],
        length_bytes[0],
        length_bytes[1],
        first_keycode_bytes[0],
        keysyms_per_keycode_bytes[0],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&keysyms_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&keysyms_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetKeyboardMapping request
pub const GET_KEYBOARD_MAPPING_REQUEST: u8 = 101;
pub fn get_keyboard_mapping<Conn>(conn: &Conn, first_keycode: Keycode, count: u8) -> Result<Cookie<'_, Conn, GetKeyboardMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let first_keycode_bytes = first_keycode.serialize();
    let count_bytes = count.serialize();
    let request0 = [
        GET_KEYBOARD_MAPPING_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        first_keycode_bytes[0],
        count_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetKeyboardMappingReply {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub keysyms: Vec<Keysym>,
}
impl GetKeyboardMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (keysyms_per_keycode, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (keysyms, remaining) = crate::x11_utils::parse_list::<Keysym>(remaining, length as usize)?;
        let result = GetKeyboardMappingReply { response_type, keysyms_per_keycode, sequence, keysyms };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKeyboardMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KB {
    KeyClickPercent = 1 << 0,
    BellPercent = 1 << 1,
    BellPitch = 1 << 2,
    BellDuration = 1 << 3,
    Led = 1 << 4,
    LedMode = 1 << 5,
    Key = 1 << 6,
    AutoRepeatMode = 1 << 7,
}
impl From<KB> for u8 {
    fn from(input: KB) -> Self {
        match input {
            KB::KeyClickPercent => 1 << 0,
            KB::BellPercent => 1 << 1,
            KB::BellPitch => 1 << 2,
            KB::BellDuration => 1 << 3,
            KB::Led => 1 << 4,
            KB::LedMode => 1 << 5,
            KB::Key => 1 << 6,
            KB::AutoRepeatMode => 1 << 7,
        }
    }
}
impl From<KB> for Option<u8> {
    fn from(input: KB) -> Self {
        Some(u8::from(input))
    }
}
impl From<KB> for u16 {
    fn from(input: KB) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<KB> for Option<u16> {
    fn from(input: KB) -> Self {
        Some(u16::from(input))
    }
}
impl From<KB> for u32 {
    fn from(input: KB) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<KB> for Option<u32> {
    fn from(input: KB) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for KB {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KB::KeyClickPercent),
            2 => Ok(KB::BellPercent),
            4 => Ok(KB::BellPitch),
            8 => Ok(KB::BellDuration),
            16 => Ok(KB::Led),
            32 => Ok(KB::LedMode),
            64 => Ok(KB::Key),
            128 => Ok(KB::AutoRepeatMode),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for KB {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for KB {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(KB, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LedMode {
    Off = 0,
    On = 1,
}
impl From<LedMode> for u8 {
    fn from(input: LedMode) -> Self {
        match input {
            LedMode::Off => 0,
            LedMode::On => 1,
        }
    }
}
impl From<LedMode> for Option<u8> {
    fn from(input: LedMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<LedMode> for u16 {
    fn from(input: LedMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LedMode> for Option<u16> {
    fn from(input: LedMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<LedMode> for u32 {
    fn from(input: LedMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<LedMode> for Option<u32> {
    fn from(input: LedMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for LedMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LedMode::Off),
            1 => Ok(LedMode::On),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for LedMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for LedMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoRepeatMode {
    Off = 0,
    On = 1,
    Default = 2,
}
impl From<AutoRepeatMode> for u8 {
    fn from(input: AutoRepeatMode) -> Self {
        match input {
            AutoRepeatMode::Off => 0,
            AutoRepeatMode::On => 1,
            AutoRepeatMode::Default => 2,
        }
    }
}
impl From<AutoRepeatMode> for Option<u8> {
    fn from(input: AutoRepeatMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<AutoRepeatMode> for u16 {
    fn from(input: AutoRepeatMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AutoRepeatMode> for Option<u16> {
    fn from(input: AutoRepeatMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<AutoRepeatMode> for u32 {
    fn from(input: AutoRepeatMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AutoRepeatMode> for Option<u32> {
    fn from(input: AutoRepeatMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for AutoRepeatMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AutoRepeatMode::Off),
            1 => Ok(AutoRepeatMode::On),
            2 => Ok(AutoRepeatMode::Default),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for AutoRepeatMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for AutoRepeatMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeKeyboardControl request
pub const CHANGE_KEYBOARD_CONTROL_REQUEST: u8 = 102;
/// Auxiliary and optional information for the change_keyboard_control function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeKeyboardControlAux {
    pub key_click_percent: Option<i32>,
    pub bell_percent: Option<i32>,
    pub bell_pitch: Option<i32>,
    pub bell_duration: Option<i32>,
    pub led: Option<u32>,
    pub led_mode: Option<u32>,
    pub key: Option<Keycode32>,
    pub auto_repeat_mode: Option<u32>,
}
impl ChangeKeyboardControlAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.key_click_percent.is_some() {
            mask |= u32::from(KB::KeyClickPercent);
        }
        if self.bell_percent.is_some() {
            mask |= u32::from(KB::BellPercent);
        }
        if self.bell_pitch.is_some() {
            mask |= u32::from(KB::BellPitch);
        }
        if self.bell_duration.is_some() {
            mask |= u32::from(KB::BellDuration);
        }
        if self.led.is_some() {
            mask |= u32::from(KB::Led);
        }
        if self.led_mode.is_some() {
            mask |= u32::from(KB::LedMode);
        }
        if self.key.is_some() {
            mask |= u32::from(KB::Key);
        }
        if self.auto_repeat_mode.is_some() {
            mask |= u32::from(KB::AutoRepeatMode);
        }
        mask
    }
    /// Set the key_click_percent field of this structure.
    pub fn key_click_percent<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.key_click_percent = value.into();
        self
    }
    /// Set the bell_percent field of this structure.
    pub fn bell_percent<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.bell_percent = value.into();
        self
    }
    /// Set the bell_pitch field of this structure.
    pub fn bell_pitch<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.bell_pitch = value.into();
        self
    }
    /// Set the bell_duration field of this structure.
    pub fn bell_duration<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.bell_duration = value.into();
        self
    }
    /// Set the led field of this structure.
    pub fn led<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.led = value.into();
        self
    }
    /// Set the led_mode field of this structure.
    pub fn led_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.led_mode = value.into();
        self
    }
    /// Set the key field of this structure.
    pub fn key<I>(mut self, value: I) -> Self where I: Into<Option<Keycode32>> {
        self.key = value.into();
        self
    }
    /// Set the auto_repeat_mode field of this structure.
    pub fn auto_repeat_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.auto_repeat_mode = value.into();
        self
    }
}
impl Serialize for ChangeKeyboardControlAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.key_click_percent {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bell_percent {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bell_pitch {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.bell_duration {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.led {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.led_mode {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.key {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.auto_repeat_mode {
            value.serialize_into(bytes);
        }
    }
}
pub fn change_keyboard_control<'c, Conn>(conn: &'c Conn, value_list: &ChangeKeyboardControlAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (8 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        CHANGE_KEYBOARD_CONTROL_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetKeyboardControl request
pub const GET_KEYBOARD_CONTROL_REQUEST: u8 = 103;
pub fn get_keyboard_control<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetKeyboardControlReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_KEYBOARD_CONTROL_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetKeyboardControlReply {
    pub response_type: u8,
    pub global_auto_repeat: AutoRepeatMode,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub auto_repeats: [u8; 32],
}
impl GetKeyboardControlReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (global_auto_repeat, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (led_mask, remaining) = u32::try_parse(remaining)?;
        let (key_click_percent, remaining) = u8::try_parse(remaining)?;
        let (bell_percent, remaining) = u8::try_parse(remaining)?;
        let (bell_pitch, remaining) = u16::try_parse(remaining)?;
        let (bell_duration, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (auto_repeats_0, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_1, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_2, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_3, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_4, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_5, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_6, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_7, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_8, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_9, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_10, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_11, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_12, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_13, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_14, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_15, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_16, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_17, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_18, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_19, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_20, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_21, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_22, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_23, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_24, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_25, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_26, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_27, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_28, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_29, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_30, remaining) = u8::try_parse(remaining)?;
        let (auto_repeats_31, remaining) = u8::try_parse(remaining)?;
        let auto_repeats = [
            auto_repeats_0,
            auto_repeats_1,
            auto_repeats_2,
            auto_repeats_3,
            auto_repeats_4,
            auto_repeats_5,
            auto_repeats_6,
            auto_repeats_7,
            auto_repeats_8,
            auto_repeats_9,
            auto_repeats_10,
            auto_repeats_11,
            auto_repeats_12,
            auto_repeats_13,
            auto_repeats_14,
            auto_repeats_15,
            auto_repeats_16,
            auto_repeats_17,
            auto_repeats_18,
            auto_repeats_19,
            auto_repeats_20,
            auto_repeats_21,
            auto_repeats_22,
            auto_repeats_23,
            auto_repeats_24,
            auto_repeats_25,
            auto_repeats_26,
            auto_repeats_27,
            auto_repeats_28,
            auto_repeats_29,
            auto_repeats_30,
            auto_repeats_31,
        ];
        let global_auto_repeat = global_auto_repeat.try_into()?;
        let result = GetKeyboardControlReply { response_type, global_auto_repeat, sequence, length, led_mask, key_click_percent, bell_percent, bell_pitch, bell_duration, auto_repeats };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetKeyboardControlReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the Bell request
pub const BELL_REQUEST: u8 = 104;
pub fn bell<Conn>(conn: &Conn, percent: i8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let percent_bytes = percent.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        BELL_REQUEST,
        percent_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ChangePointerControl request
pub const CHANGE_POINTER_CONTROL_REQUEST: u8 = 105;
pub fn change_pointer_control<Conn>(conn: &Conn, acceleration_numerator: i16, acceleration_denominator: i16, threshold: i16, do_acceleration: bool, do_threshold: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let acceleration_numerator_bytes = acceleration_numerator.serialize();
    let acceleration_denominator_bytes = acceleration_denominator.serialize();
    let threshold_bytes = threshold.serialize();
    let do_acceleration_bytes = (do_acceleration as u8).serialize();
    let do_threshold_bytes = (do_threshold as u8).serialize();
    let request0 = [
        CHANGE_POINTER_CONTROL_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        acceleration_numerator_bytes[0],
        acceleration_numerator_bytes[1],
        acceleration_denominator_bytes[0],
        acceleration_denominator_bytes[1],
        threshold_bytes[0],
        threshold_bytes[1],
        do_acceleration_bytes[0],
        do_threshold_bytes[0],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetPointerControl request
pub const GET_POINTER_CONTROL_REQUEST: u8 = 106;
pub fn get_pointer_control<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPointerControlReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_POINTER_CONTROL_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPointerControlReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: u16,
    pub acceleration_denominator: u16,
    pub threshold: u16,
}
impl GetPointerControlReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (acceleration_numerator, remaining) = u16::try_parse(remaining)?;
        let (acceleration_denominator, remaining) = u16::try_parse(remaining)?;
        let (threshold, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let result = GetPointerControlReply { response_type, sequence, length, acceleration_numerator, acceleration_denominator, threshold };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPointerControlReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Blanking {
    NotPreferred = 0,
    Preferred = 1,
    Default = 2,
}
impl From<Blanking> for u8 {
    fn from(input: Blanking) -> Self {
        match input {
            Blanking::NotPreferred => 0,
            Blanking::Preferred => 1,
            Blanking::Default => 2,
        }
    }
}
impl From<Blanking> for Option<u8> {
    fn from(input: Blanking) -> Self {
        Some(u8::from(input))
    }
}
impl From<Blanking> for u16 {
    fn from(input: Blanking) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Blanking> for Option<u16> {
    fn from(input: Blanking) -> Self {
        Some(u16::from(input))
    }
}
impl From<Blanking> for u32 {
    fn from(input: Blanking) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Blanking> for Option<u32> {
    fn from(input: Blanking) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Blanking {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Blanking::NotPreferred),
            1 => Ok(Blanking::Preferred),
            2 => Ok(Blanking::Default),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Blanking {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Blanking {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Exposures {
    NotAllowed = 0,
    Allowed = 1,
    Default = 2,
}
impl From<Exposures> for u8 {
    fn from(input: Exposures) -> Self {
        match input {
            Exposures::NotAllowed => 0,
            Exposures::Allowed => 1,
            Exposures::Default => 2,
        }
    }
}
impl From<Exposures> for Option<u8> {
    fn from(input: Exposures) -> Self {
        Some(u8::from(input))
    }
}
impl From<Exposures> for u16 {
    fn from(input: Exposures) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Exposures> for Option<u16> {
    fn from(input: Exposures) -> Self {
        Some(u16::from(input))
    }
}
impl From<Exposures> for u32 {
    fn from(input: Exposures) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Exposures> for Option<u32> {
    fn from(input: Exposures) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Exposures {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Exposures::NotAllowed),
            1 => Ok(Exposures::Allowed),
            2 => Ok(Exposures::Default),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Exposures {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Exposures {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetScreenSaver request
pub const SET_SCREEN_SAVER_REQUEST: u8 = 107;
pub fn set_screen_saver<Conn, A, B>(conn: &Conn, timeout: i16, interval: i16, prefer_blanking: A, allow_exposures: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let timeout_bytes = timeout.serialize();
    let interval_bytes = interval.serialize();
    let prefer_blanking = prefer_blanking.into();
    let prefer_blanking_bytes = prefer_blanking.serialize();
    let allow_exposures = allow_exposures.into();
    let allow_exposures_bytes = allow_exposures.serialize();
    let request0 = [
        SET_SCREEN_SAVER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        timeout_bytes[0],
        timeout_bytes[1],
        interval_bytes[0],
        interval_bytes[1],
        prefer_blanking_bytes[0],
        allow_exposures_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetScreenSaver request
pub const GET_SCREEN_SAVER_REQUEST: u8 = 108;
pub fn get_screen_saver<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetScreenSaverReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_SCREEN_SAVER_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetScreenSaverReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: u16,
    pub interval: u16,
    pub prefer_blanking: Blanking,
    pub allow_exposures: Exposures,
}
impl GetScreenSaverReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (timeout, remaining) = u16::try_parse(remaining)?;
        let (interval, remaining) = u16::try_parse(remaining)?;
        let (prefer_blanking, remaining) = u8::try_parse(remaining)?;
        let (allow_exposures, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(18..).ok_or(ParseError::ParseError)?;
        let prefer_blanking = prefer_blanking.try_into()?;
        let allow_exposures = allow_exposures.try_into()?;
        let result = GetScreenSaverReply { response_type, sequence, length, timeout, interval, prefer_blanking, allow_exposures };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetScreenSaverReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HostMode {
    Insert = 0,
    Delete = 1,
}
impl From<HostMode> for u8 {
    fn from(input: HostMode) -> Self {
        match input {
            HostMode::Insert => 0,
            HostMode::Delete => 1,
        }
    }
}
impl From<HostMode> for Option<u8> {
    fn from(input: HostMode) -> Self {
        Some(u8::from(input))
    }
}
impl From<HostMode> for u16 {
    fn from(input: HostMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HostMode> for Option<u16> {
    fn from(input: HostMode) -> Self {
        Some(u16::from(input))
    }
}
impl From<HostMode> for u32 {
    fn from(input: HostMode) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<HostMode> for Option<u32> {
    fn from(input: HostMode) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for HostMode {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HostMode::Insert),
            1 => Ok(HostMode::Delete),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for HostMode {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for HostMode {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Family {
    Internet = 0,
    DECnet = 1,
    Chaos = 2,
    ServerInterpreted = 5,
    Internet6 = 6,
}
impl From<Family> for u8 {
    fn from(input: Family) -> Self {
        match input {
            Family::Internet => 0,
            Family::DECnet => 1,
            Family::Chaos => 2,
            Family::ServerInterpreted => 5,
            Family::Internet6 => 6,
        }
    }
}
impl From<Family> for Option<u8> {
    fn from(input: Family) -> Self {
        Some(u8::from(input))
    }
}
impl From<Family> for u16 {
    fn from(input: Family) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Family> for Option<u16> {
    fn from(input: Family) -> Self {
        Some(u16::from(input))
    }
}
impl From<Family> for u32 {
    fn from(input: Family) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Family> for Option<u32> {
    fn from(input: Family) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Family {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Family::Internet),
            1 => Ok(Family::DECnet),
            2 => Ok(Family::Chaos),
            5 => Ok(Family::ServerInterpreted),
            6 => Ok(Family::Internet6),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Family {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Family {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ChangeHosts request
pub const CHANGE_HOSTS_REQUEST: u8 = 109;
pub fn change_hosts<'c, Conn, A, B>(conn: &'c Conn, mode: A, family: B, address: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>, B: Into<u8>
{
    let length: usize = (8 + 1 * address.len() + 3) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let family = family.into();
    let family_bytes = family.serialize();
    let address_len: u16 = address.len().try_into()?;
    let address_len_bytes = address_len.serialize();
    let request0 = [
        CHANGE_HOSTS_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
        family_bytes[0],
        0,
        address_len_bytes[0],
        address_len_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (address).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(address), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    pub family: Family,
    pub address: Vec<u8>,
}
impl TryParse for Host {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (family, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (address_len, remaining) = u16::try_parse(remaining)?;
        let (address, remaining) = crate::x11_utils::parse_list::<u8>(remaining, address_len as usize)?;
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let family = family.try_into()?;
        let result = Host { family, address };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Host {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Host {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        u8::from(self.family).serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        let address_len = self.address.len() as u16;
        address_len.serialize_into(bytes);
        self.address.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}

/// Opcode for the ListHosts request
pub const LIST_HOSTS_REQUEST: u8 = 110;
pub fn list_hosts<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListHostsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        LIST_HOSTS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListHostsReply {
    pub response_type: u8,
    pub mode: AccessControl,
    pub sequence: u16,
    pub length: u32,
    pub hosts: Vec<Host>,
}
impl ListHostsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (mode, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (hosts_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (hosts, remaining) = crate::x11_utils::parse_list::<Host>(remaining, hosts_len as usize)?;
        let mode = mode.try_into()?;
        let result = ListHostsReply { response_type, mode, sequence, length, hosts };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListHostsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessControl {
    Disable = 0,
    Enable = 1,
}
impl From<AccessControl> for u8 {
    fn from(input: AccessControl) -> Self {
        match input {
            AccessControl::Disable => 0,
            AccessControl::Enable => 1,
        }
    }
}
impl From<AccessControl> for Option<u8> {
    fn from(input: AccessControl) -> Self {
        Some(u8::from(input))
    }
}
impl From<AccessControl> for u16 {
    fn from(input: AccessControl) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AccessControl> for Option<u16> {
    fn from(input: AccessControl) -> Self {
        Some(u16::from(input))
    }
}
impl From<AccessControl> for u32 {
    fn from(input: AccessControl) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<AccessControl> for Option<u32> {
    fn from(input: AccessControl) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for AccessControl {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccessControl::Disable),
            1 => Ok(AccessControl::Enable),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for AccessControl {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for AccessControl {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetAccessControl request
pub const SET_ACCESS_CONTROL_REQUEST: u8 = 111;
pub fn set_access_control<Conn, A>(conn: &Conn, mode: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (4) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        SET_ACCESS_CONTROL_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CloseDown {
    DestroyAll = 0,
    RetainPermanent = 1,
    RetainTemporary = 2,
}
impl From<CloseDown> for u8 {
    fn from(input: CloseDown) -> Self {
        match input {
            CloseDown::DestroyAll => 0,
            CloseDown::RetainPermanent => 1,
            CloseDown::RetainTemporary => 2,
        }
    }
}
impl From<CloseDown> for Option<u8> {
    fn from(input: CloseDown) -> Self {
        Some(u8::from(input))
    }
}
impl From<CloseDown> for u16 {
    fn from(input: CloseDown) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CloseDown> for Option<u16> {
    fn from(input: CloseDown) -> Self {
        Some(u16::from(input))
    }
}
impl From<CloseDown> for u32 {
    fn from(input: CloseDown) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CloseDown> for Option<u32> {
    fn from(input: CloseDown) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CloseDown {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CloseDown::DestroyAll),
            1 => Ok(CloseDown::RetainPermanent),
            2 => Ok(CloseDown::RetainTemporary),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CloseDown {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CloseDown {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetCloseDownMode request
pub const SET_CLOSE_DOWN_MODE_REQUEST: u8 = 112;
pub fn set_close_down_mode<Conn, A>(conn: &Conn, mode: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (4) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        SET_CLOSE_DOWN_MODE_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Kill {
    AllTemporary = 0,
}
impl From<Kill> for u8 {
    fn from(input: Kill) -> Self {
        match input {
            Kill::AllTemporary => 0,
        }
    }
}
impl From<Kill> for Option<u8> {
    fn from(input: Kill) -> Self {
        Some(u8::from(input))
    }
}
impl From<Kill> for u16 {
    fn from(input: Kill) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Kill> for Option<u16> {
    fn from(input: Kill) -> Self {
        Some(u16::from(input))
    }
}
impl From<Kill> for u32 {
    fn from(input: Kill) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Kill> for Option<u32> {
    fn from(input: Kill) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Kill {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Kill::AllTemporary),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for Kill {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Kill {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the KillClient request
pub const KILL_CLIENT_REQUEST: u8 = 113;
/// kills a client.
///
/// Forces a close down of the client that created the specified `resource`.
///
/// # Fields
///
/// * `resource` - Any resource belonging to the client (for example a Window), used to identify
/// the client connection.
///
/// The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
/// that have terminated in `RetainTemporary` (TODO) are destroyed.
///
/// # Errors
///
/// * `Value` - The specified `resource` does not exist.
///
/// # See
///
/// * xkill: program
pub fn kill_client<Conn>(conn: &Conn, resource: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let resource_bytes = resource.serialize();
    let request0 = [
        KILL_CLIENT_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        resource_bytes[0],
        resource_bytes[1],
        resource_bytes[2],
        resource_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the RotateProperties request
pub const ROTATE_PROPERTIES_REQUEST: u8 = 114;
pub fn rotate_properties<'c, Conn>(conn: &'c Conn, window: Window, delta: i16, atoms: &[Atom]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (12 + 4 * atoms.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let window_bytes = window.serialize();
    let atoms_len: u16 = atoms.len().try_into()?;
    let atoms_len_bytes = atoms_len.serialize();
    let delta_bytes = delta.serialize();
    let atoms_bytes = atoms.serialize();
    let request0 = [
        ROTATE_PROPERTIES_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        atoms_len_bytes[0],
        atoms_len_bytes[1],
        delta_bytes[0],
        delta_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&atoms_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&atoms_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScreenSaver {
    Reset = 0,
    Active = 1,
}
impl From<ScreenSaver> for u8 {
    fn from(input: ScreenSaver) -> Self {
        match input {
            ScreenSaver::Reset => 0,
            ScreenSaver::Active => 1,
        }
    }
}
impl From<ScreenSaver> for Option<u8> {
    fn from(input: ScreenSaver) -> Self {
        Some(u8::from(input))
    }
}
impl From<ScreenSaver> for u16 {
    fn from(input: ScreenSaver) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScreenSaver> for Option<u16> {
    fn from(input: ScreenSaver) -> Self {
        Some(u16::from(input))
    }
}
impl From<ScreenSaver> for u32 {
    fn from(input: ScreenSaver) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ScreenSaver> for Option<u32> {
    fn from(input: ScreenSaver) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ScreenSaver {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScreenSaver::Reset),
            1 => Ok(ScreenSaver::Active),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ScreenSaver {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ScreenSaver {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the ForceScreenSaver request
pub const FORCE_SCREEN_SAVER_REQUEST: u8 = 115;
pub fn force_screen_saver<Conn, A>(conn: &Conn, mode: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized, A: Into<u8>
{
    let length: usize = (4) / 4;
    let mode = mode.into();
    let mode_bytes = mode.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        FORCE_SCREEN_SAVER_REQUEST,
        mode_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MappingStatus {
    Success = 0,
    Busy = 1,
    Failure = 2,
}
impl From<MappingStatus> for u8 {
    fn from(input: MappingStatus) -> Self {
        match input {
            MappingStatus::Success => 0,
            MappingStatus::Busy => 1,
            MappingStatus::Failure => 2,
        }
    }
}
impl From<MappingStatus> for Option<u8> {
    fn from(input: MappingStatus) -> Self {
        Some(u8::from(input))
    }
}
impl From<MappingStatus> for u16 {
    fn from(input: MappingStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MappingStatus> for Option<u16> {
    fn from(input: MappingStatus) -> Self {
        Some(u16::from(input))
    }
}
impl From<MappingStatus> for u32 {
    fn from(input: MappingStatus) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MappingStatus> for Option<u32> {
    fn from(input: MappingStatus) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for MappingStatus {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MappingStatus::Success),
            1 => Ok(MappingStatus::Busy),
            2 => Ok(MappingStatus::Failure),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for MappingStatus {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for MappingStatus {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetPointerMapping request
pub const SET_POINTER_MAPPING_REQUEST: u8 = 116;
pub fn set_pointer_mapping<'c, Conn>(conn: &'c Conn, map: &[u8]) -> Result<Cookie<'c, Conn, SetPointerMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4 + 1 * map.len() + 3) / 4;
    let map_len: u8 = map.len().try_into()?;
    let map_len_bytes = map_len.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        SET_POINTER_MAPPING_REQUEST,
        map_len_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (map).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(map), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPointerMappingReply {
    pub response_type: u8,
    pub status: MappingStatus,
    pub sequence: u16,
    pub length: u32,
}
impl SetPointerMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = SetPointerMappingReply { response_type, status, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetPointerMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPointerMapping request
pub const GET_POINTER_MAPPING_REQUEST: u8 = 117;
pub fn get_pointer_mapping<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPointerMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_POINTER_MAPPING_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPointerMappingReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: Vec<u8>,
}
impl GetPointerMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (map_len, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (map, remaining) = crate::x11_utils::parse_list::<u8>(remaining, map_len as usize)?;
        let result = GetPointerMappingReply { response_type, sequence, length, map };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPointerMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MapIndex {
    Shift = 0,
    Lock = 1,
    Control = 2,
    M1 = 3,
    M2 = 4,
    M3 = 5,
    M4 = 6,
    M5 = 7,
}
impl From<MapIndex> for u8 {
    fn from(input: MapIndex) -> Self {
        match input {
            MapIndex::Shift => 0,
            MapIndex::Lock => 1,
            MapIndex::Control => 2,
            MapIndex::M1 => 3,
            MapIndex::M2 => 4,
            MapIndex::M3 => 5,
            MapIndex::M4 => 6,
            MapIndex::M5 => 7,
        }
    }
}
impl From<MapIndex> for Option<u8> {
    fn from(input: MapIndex) -> Self {
        Some(u8::from(input))
    }
}
impl From<MapIndex> for u16 {
    fn from(input: MapIndex) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapIndex> for Option<u16> {
    fn from(input: MapIndex) -> Self {
        Some(u16::from(input))
    }
}
impl From<MapIndex> for u32 {
    fn from(input: MapIndex) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<MapIndex> for Option<u32> {
    fn from(input: MapIndex) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for MapIndex {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MapIndex::Shift),
            1 => Ok(MapIndex::Lock),
            2 => Ok(MapIndex::Control),
            3 => Ok(MapIndex::M1),
            4 => Ok(MapIndex::M2),
            5 => Ok(MapIndex::M3),
            6 => Ok(MapIndex::M4),
            7 => Ok(MapIndex::M5),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for MapIndex {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for MapIndex {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

/// Opcode for the SetModifierMapping request
pub const SET_MODIFIER_MAPPING_REQUEST: u8 = 118;
pub fn set_modifier_mapping<'c, Conn>(conn: &'c Conn, keycodes: &[Keycode]) -> Result<Cookie<'c, Conn, SetModifierMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4 + 1 * keycodes.len() + 3) / 4;
    assert_eq!(0, keycodes.len() % 8, "Argument keycodes_per_modifier has an incorrect length, must be a multiple of 8");
    let keycodes_per_modifier = u8::try_from(keycodes.len() / 8).unwrap();
    let keycodes_per_modifier_bytes = keycodes_per_modifier.serialize();
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        SET_MODIFIER_MAPPING_REQUEST,
        keycodes_per_modifier_bytes[0],
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (keycodes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(keycodes), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetModifierMappingReply {
    pub response_type: u8,
    pub status: MappingStatus,
    pub sequence: u16,
    pub length: u32,
}
impl SetModifierMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (status, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let status = status.try_into()?;
        let result = SetModifierMappingReply { response_type, status, sequence, length };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for SetModifierMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetModifierMapping request
pub const GET_MODIFIER_MAPPING_REQUEST: u8 = 119;
pub fn get_modifier_mapping<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetModifierMappingReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        GET_MODIFIER_MAPPING_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetModifierMappingReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes: Vec<Keycode>,
}
impl GetModifierMappingReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (keycodes_per_modifier, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let (keycodes, remaining) = crate::x11_utils::parse_list::<Keycode>(remaining, (keycodes_per_modifier as usize) * (8))?;
        let result = GetModifierMappingReply { response_type, sequence, length, keycodes };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetModifierMappingReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the NoOperation request
pub const NO_OPERATION_REQUEST: u8 = 127;
pub fn no_operation<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        NO_OPERATION_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    /// Creates a window.
    ///
    /// Creates an unmapped window as child of the specified `parent` window. A
    /// CreateNotify event will be generated. The new window is placed on top in the
    /// stacking order with respect to siblings.
    ///
    /// The coordinate system has the X axis horizontal and the Y axis vertical with
    /// the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
    /// of pixels, and coincide with pixel centers. Each window and pixmap has its own
    /// coordinate system. For a window, the origin is inside the border at the inside,
    /// upper-left corner.
    ///
    /// The created window is not yet displayed (mapped), call `xcb_map_window` to
    /// display it.
    ///
    /// The created window will initially use the same cursor as its parent.
    ///
    /// # Fields
    ///
    /// * `border_width` - TODO:
    ///
    /// Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
    /// * `depth` - Specifies the new window's depth (TODO: what unit?).
    ///
    /// The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
    /// `parent` window.
    /// * `height` - The height of the new window.
    /// * `parent` - The parent window of the new window.
    /// * `visual` - Specifies the id for the new window's visual.
    ///
    /// The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
    /// `parent` window.
    /// * `wid` - The ID with which you will refer to the new window, created by
    /// `xcb_generate_id`.
    /// * `width` - The width of the new window.
    /// * `x` - The X coordinate of the new window.
    /// * `y` - The Y coordinate of the new window.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not allocate the requested resources (no memory?).
    /// * `Colormap` - TODO: reasons?
    /// * `Cursor` - TODO: reasons?
    /// * `Match` - TODO: reasons?
    /// * `Pixmap` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    /// * `Window` - TODO: reasons?
    ///
    /// # See
    ///
    /// * CreateNotify: event
    /// * MapWindow: request
    /// * xcb_generate_id: function
    fn create_window<'c, A>(&'c self, depth: u8, wid: Window, parent: Window, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: A, visual: Visualid, value_list: &CreateWindowAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u16>
    {
        create_window(self, depth, wid, parent, x, y, width, height, border_width, class, visual, value_list)
    }

    /// change window attributes.
    ///
    /// Changes the attributes specified by `value_mask` for the specified `window`.
    ///
    /// # Fields
    ///
    /// * `value_list` - Values for each of the attributes specified in the bitmask `value_mask`. The
    /// order has to correspond to the order of possible `value_mask` bits. See the
    /// example.
    /// * `window` - The window to change.
    ///
    /// # Errors
    ///
    /// * `Access` - TODO: reasons?
    /// * `Colormap` - TODO: reasons?
    /// * `Cursor` - TODO: reasons?
    /// * `Match` - TODO: reasons?
    /// * `Pixmap` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    fn change_window_attributes<'c>(&'c self, window: Window, value_list: &ChangeWindowAttributesAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_window_attributes(self, window, value_list)
    }

    /// Gets window attributes.
    ///
    /// Gets the current attributes for the specified `window`.
    ///
    /// # Fields
    ///
    /// * `window` - The window to get the attributes from.
    ///
    /// # Errors
    ///
    /// * `Drawable` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    fn get_window_attributes(&self, window: Window) -> Result<Cookie<'_, Self, GetWindowAttributesReply>, ConnectionError>
    {
        get_window_attributes(self, window)
    }

    /// Destroys a window.
    ///
    /// Destroys the specified window and all of its subwindows. A DestroyNotify event
    /// is generated for each destroyed window (a DestroyNotify event is first generated
    /// for any given window's inferiors). If the window was mapped, it will be
    /// automatically unmapped before destroying.
    ///
    /// Calling DestroyWindow on the root window will do nothing.
    ///
    /// # Fields
    ///
    /// * `window` - The window to destroy.
    ///
    /// # Errors
    ///
    /// * `Window` - The specified window does not exist.
    ///
    /// # See
    ///
    /// * DestroyNotify: event
    /// * MapWindow: request
    /// * UnmapWindow: request
    fn destroy_window(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_window(self, window)
    }

    fn destroy_subwindows(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        destroy_subwindows(self, window)
    }

    /// Changes a client's save set.
    ///
    /// TODO: explain what the save set is for.
    ///
    /// This function either adds or removes the specified window to the client's (your
    /// application's) save set.
    ///
    /// # Fields
    ///
    /// * `mode` - Insert to add the specified window to the save set or Delete to delete it from the save set.
    /// * `window` - The window to add or delete to/from your save set.
    ///
    /// # Errors
    ///
    /// * `Match` - You created the specified window. This does not make sense, you can only add
    ///windows created by other clients to your save set.
    /// * `Value` - You specified an invalid mode.
    /// * `Window` - The specified window does not exist.
    ///
    /// # See
    ///
    /// * ReparentWindow: request
    fn change_save_set<A>(&self, mode: A, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        change_save_set(self, mode, window)
    }

    /// Reparents a window.
    ///
    /// Makes the specified window a child of the specified parent window. If the
    /// window is mapped, it will automatically be unmapped before reparenting and
    /// re-mapped after reparenting. The window is placed in the stacking order on top
    /// with respect to sibling windows.
    ///
    /// After reparenting, a ReparentNotify event is generated.
    ///
    /// # Fields
    ///
    /// * `parent` - The new parent of the window.
    /// * `window` - The window to reparent.
    /// * `x` - The X position of the window within its new parent.
    /// * `y` - The Y position of the window within its new parent.
    ///
    /// # Errors
    ///
    /// * `Match` - The new parent window is not on the same screen as the old parent window.
    ///
    ///The new parent window is the specified window or an inferior of the specified window.
    ///
    ///The new parent is InputOnly and the window is not.
    ///
    ///The specified window has a ParentRelative background and the new parent window is not the same depth as the specified window.
    /// * `Window` - The specified window does not exist.
    ///
    /// # See
    ///
    /// * MapWindow: request
    /// * ReparentNotify: event
    /// * UnmapWindow: request
    fn reparent_window(&self, window: Window, parent: Window, x: i16, y: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        reparent_window(self, window, parent, x, y)
    }

    /// Makes a window visible.
    ///
    /// Maps the specified window. This means making the window visible (as long as its
    /// parent is visible).
    ///
    /// This MapWindow request will be translated to a MapRequest request if a window
    /// manager is running. The window manager then decides to either map the window or
    /// not. Set the override-redirect window attribute to true if you want to bypass
    /// this mechanism.
    ///
    /// If the window manager decides to map the window (or if no window manager is
    /// running), a MapNotify event is generated.
    ///
    /// If the window becomes viewable and no earlier contents for it are remembered,
    /// the X server tiles the window with its background. If the window's background
    /// is undefined, the existing screen contents are not altered, and the X server
    /// generates zero or more Expose events.
    ///
    /// If the window type is InputOutput, an Expose event will be generated when the
    /// window becomes visible. The normal response to an Expose event should be to
    /// repaint the window.
    ///
    /// # Fields
    ///
    /// * `window` - The window to make visible.
    ///
    /// # Errors
    ///
    /// * `Match` - The specified window does not exist.
    ///
    /// # See
    ///
    /// * Expose: event
    /// * MapNotify: event
    /// * UnmapWindow: request
    fn map_window(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        map_window(self, window)
    }

    fn map_subwindows(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        map_subwindows(self, window)
    }

    /// Makes a window invisible.
    ///
    /// Unmaps the specified window. This means making the window invisible (and all
    /// its child windows).
    ///
    /// Unmapping a window leads to the `UnmapNotify` event being generated. Also,
    /// `Expose` events are generated for formerly obscured windows.
    ///
    /// # Fields
    ///
    /// * `window` - The window to make invisible.
    ///
    /// # Errors
    ///
    /// * `Window` - The specified window does not exist.
    ///
    /// # See
    ///
    /// * Expose: event
    /// * MapWindow: request
    /// * UnmapNotify: event
    fn unmap_window(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        unmap_window(self, window)
    }

    fn unmap_subwindows(&self, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        unmap_subwindows(self, window)
    }

    /// Configures window attributes.
    ///
    /// Configures a window's size, position, border width and stacking order.
    ///
    /// # Fields
    ///
    /// * `value_list` - New values, corresponding to the attributes in value_mask. The order has to
    /// correspond to the order of possible `value_mask` bits. See the example.
    /// * `value_mask` - Bitmask of attributes to change.
    /// * `window` - The window to configure.
    ///
    /// # Errors
    ///
    /// * `Match` - You specified a Sibling without also specifying StackMode or the window is not
    ///actually a Sibling.
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified window does not exist. TODO: any other reason?
    ///
    /// # See
    ///
    /// * Expose: event
    /// * MapNotify: event
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Configures the given window to the left upper corner
    ///  * with a size of 1024x768 pixels.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *c, xcb_window_t window) {
    ///     uint16_t mask = 0;
    ///
    ///     mask |= XCB_CONFIG_WINDOW_X;
    ///     mask |= XCB_CONFIG_WINDOW_Y;
    ///     mask |= XCB_CONFIG_WINDOW_WIDTH;
    ///     mask |= XCB_CONFIG_WINDOW_HEIGHT;
    ///
    ///     const uint32_t values[] = {
    ///         0,    /* x */
    ///         0,    /* y */
    ///         1024, /* width */
    ///         768   /* height */
    ///     };
    ///
    ///     xcb_configure_window(c, window, mask, values);
    ///     xcb_flush(c);
    /// }
    /// ```
    fn configure_window<'c>(&'c self, window: Window, value_list: &ConfigureWindowAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_window(self, window, value_list)
    }

    /// Change window stacking order.
    ///
    /// If `direction` is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
    /// any) will be raised to the top of the stack.
    ///
    /// If `direction` is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
    /// be lowered to the bottom of the stack.
    ///
    /// # Fields
    ///
    /// * `window` - The window to raise/lower (depending on `direction`).
    ///
    /// # Errors
    ///
    /// * `Value` - The specified `direction` is invalid.
    /// * `Window` - The specified `window` does not exist.
    fn circulate_window<A>(&self, direction: A, window: Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        circulate_window(self, direction, window)
    }

    /// Get current window geometry.
    ///
    /// Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
    ///
    /// # Fields
    ///
    /// * `drawable` - The drawable (`Window` or `Pixmap`) of which the geometry will be received.
    ///
    /// # Errors
    ///
    /// * `Drawable` - TODO: reasons?
    /// * `Window` - TODO: reasons?
    ///
    /// # See
    ///
    /// * xwininfo: program
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Displays the x and y position of the given window.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *c, xcb_window_t window) {
    ///     xcb_get_geometry_cookie_t cookie;
    ///     xcb_get_geometry_reply_t *reply;
    ///
    ///     cookie = xcb_get_geometry(c, window);
    ///     /* ... do other work here if possible ... */
    ///     if ((reply = xcb_get_geometry_reply(c, cookie, NULL))) {
    ///         printf("This window is at %d, %d\\n", reply->x, reply->y);
    ///     }
    ///     free(reply);
    /// }
    /// ```
    fn get_geometry(&self, drawable: Drawable) -> Result<Cookie<'_, Self, GetGeometryReply>, ConnectionError>
    {
        get_geometry(self, drawable)
    }

    /// query the window tree.
    ///
    /// Gets the root window ID, parent window ID and list of children windows for the
    /// specified `window`. The children are listed in bottom-to-top stacking order.
    ///
    /// # Fields
    ///
    /// * `window` - The `window` to query.
    ///
    /// # See
    ///
    /// * xwininfo: program
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Displays the root, parent and children of the specified window.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_window_t window) {
    ///     xcb_query_tree_cookie_t cookie;
    ///     xcb_query_tree_reply_t *reply;
    ///
    ///     cookie = xcb_query_tree(conn, window);
    ///     if ((reply = xcb_query_tree_reply(conn, cookie, NULL))) {
    ///         printf("root = 0x%08x\\n", reply->root);
    ///         printf("parent = 0x%08x\\n", reply->parent);
    ///
    ///         xcb_window_t *children = xcb_query_tree_children(reply);
    ///         for (int i = 0; i < xcb_query_tree_children_length(reply); i++)
    ///             printf("child window = 0x%08x\\n", children[i]);
    ///
    ///         free(reply);
    ///     }
    /// }
    /// ```
    fn query_tree(&self, window: Window) -> Result<Cookie<'_, Self, QueryTreeReply>, ConnectionError>
    {
        query_tree(self, window)
    }

    /// Get atom identifier by name.
    ///
    /// Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
    /// name. Atoms are used in protocols like EWMH, for example to store window titles
    /// (`_NET_WM_NAME` atom) as property of a window.
    ///
    /// If `only_if_exists` is 0, the atom will be created if it does not already exist.
    /// If `only_if_exists` is 1, `XCB_ATOM_NONE` will be returned if the atom does
    /// not yet exist.
    ///
    /// # Fields
    ///
    /// * `name` - The name of the atom.
    /// * `name_len` - The length of the following `name`.
    /// * `only_if_exists` - Return a valid atom id only if the atom already exists.
    ///
    /// # Errors
    ///
    /// * `Alloc` - TODO: reasons?
    /// * `Value` - A value other than 0 or 1 was specified for `only_if_exists`.
    ///
    /// # See
    ///
    /// * GetAtomName: request
    /// * xlsatoms: program
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Resolves the _NET_WM_NAME atom.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *c) {
    ///     xcb_intern_atom_cookie_t cookie;
    ///     xcb_intern_atom_reply_t *reply;
    ///
    ///     cookie = xcb_intern_atom(c, 0, strlen("_NET_WM_NAME"), "_NET_WM_NAME");
    ///     /* ... do other work here if possible ... */
    ///     if ((reply = xcb_intern_atom_reply(c, cookie, NULL))) {
    ///         printf("The _NET_WM_NAME atom has ID %u\n", reply->atom);
    ///         free(reply);
    ///     }
    /// }
    /// ```
    fn intern_atom<'c>(&'c self, only_if_exists: bool, name: &[u8]) -> Result<Cookie<'c, Self, InternAtomReply>, ConnectionError>
    {
        intern_atom(self, only_if_exists, name)
    }

    fn get_atom_name(&self, atom: Atom) -> Result<Cookie<'_, Self, GetAtomNameReply>, ConnectionError>
    {
        get_atom_name(self, atom)
    }

    /// Changes a window property.
    ///
    /// Sets or updates a property on the specified `window`. Properties are for
    /// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
    /// Protocols such as EWMH also use properties - for example EWMH defines the
    /// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
    ///
    /// # Fields
    ///
    /// * `data` - The property data.
    /// * `data_len` - Specifies the number of elements (see `format`).
    /// * `format` - Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
    /// 32-bit quantities. Possible values are 8, 16 and 32. This information allows
    /// the X server to correctly perform byte-swap operations as necessary.
    /// * `property` - The property you want to change (an atom).
    /// * `type` - The type of the property you want to change (an atom).
    /// * `window` - The window whose property you want to change.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not store the property (no memory?).
    /// * `Atom` - `property` or `type` do not refer to a valid atom.
    /// * `Match` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    ///
    /// # See
    ///
    /// * InternAtom: request
    /// * xprop: program
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Sets the WM_NAME property of the window to "XCB Example".
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_window_t window) {
    ///     xcb_change_property(conn,
    ///         XCB_PROP_MODE_REPLACE,
    ///         window,
    ///         XCB_ATOM_WM_NAME,
    ///         XCB_ATOM_STRING,
    ///         8,
    ///         strlen("XCB Example"),
    ///         "XCB Example");
    ///     xcb_flush(conn);
    /// }
    /// ```
    fn change_property<'c, A, B, C>(&'c self, mode: A, window: Window, property: B, type_: C, format: u8, data_len: u32, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>, B: Into<Atom>, C: Into<Atom>
    {
        change_property(self, mode, window, property, type_, format, data_len, data)
    }

    fn delete_property(&self, window: Window, property: Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_property(self, window, property)
    }

    /// Gets a window property.
    ///
    /// Gets the specified `property` from the specified `window`. Properties are for
    /// example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
    /// Protocols such as EWMH also use properties - for example EWMH defines the
    /// window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
    ///
    /// TODO: talk about `type`
    ///
    /// TODO: talk about `delete`
    ///
    /// TODO: talk about the offset/length thing. what's a valid use case?
    ///
    /// # Fields
    ///
    /// * `delete` - Whether the property should actually be deleted. For deleting a property, the
    /// specified `type` has to match the actual property type.
    /// * `long_length` - Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
    /// set `long_length` to 4, you will receive 16 bytes of data).
    /// * `long_offset` - Specifies the offset (in 32-bit multiples) in the specified property where the
    /// data is to be retrieved.
    /// * `property` - The property you want to get (an atom).
    /// * `type` - The type of the property you want to get (an atom).
    /// * `window` - The window whose property you want to get.
    ///
    /// # Errors
    ///
    /// * `Atom` - `property` or `type` do not refer to a valid atom.
    /// * `Value` - The specified `long_offset` is beyond the actual property length (e.g. the
    ///property has a length of 3 bytes and you are setting `long_offset` to 1,
    ///resulting in a byte offset of 4).
    /// * `Window` - The specified `window` does not exist.
    ///
    /// # See
    ///
    /// * InternAtom: request
    /// * xprop: program
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Prints the WM_NAME property of the window.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *c, xcb_window_t window) {
    ///     xcb_get_property_cookie_t cookie;
    ///     xcb_get_property_reply_t *reply;
    ///
    ///     /* These atoms are predefined in the X11 protocol. */
    ///     xcb_atom_t property = XCB_ATOM_WM_NAME;
    ///     xcb_atom_t type = XCB_ATOM_STRING;
    ///
    ///     // TODO: a reasonable long_length for WM_NAME?
    ///     cookie = xcb_get_property(c, 0, window, property, type, 0, 0);
    ///     if ((reply = xcb_get_property_reply(c, cookie, NULL))) {
    ///         int len = xcb_get_property_value_length(reply);
    ///         if (len == 0) {
    ///             printf("TODO\\n");
    ///             free(reply);
    ///             return;
    ///         }
    ///         printf("WM_NAME is %.*s\\n", len,
    ///                (char*)xcb_get_property_value(reply));
    ///     }
    ///     free(reply);
    /// }
    /// ```
    fn get_property(&self, delete: bool, window: Window, property: Atom, type_: Atom, long_offset: u32, long_length: u32) -> Result<Cookie<'_, Self, GetPropertyReply>, ConnectionError>
    {
        get_property(self, delete, window, property, type_, long_offset, long_length)
    }

    fn list_properties(&self, window: Window) -> Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>
    {
        list_properties(self, window)
    }

    /// Sets the owner of a selection.
    ///
    /// Makes `window` the owner of the selection `selection` and updates the
    /// last-change time of the specified selection.
    ///
    /// TODO: briefly explain what a selection is.
    ///
    /// # Fields
    ///
    /// * `owner` - The new owner of the selection.
    ///
    /// The special value `XCB_NONE` means that the selection will have no owner.
    /// * `selection` - The selection.
    /// * `time` - Timestamp to avoid race conditions when running X over the network.
    ///
    /// The selection will not be changed if `time` is earlier than the current
    /// last-change time of the `selection` or is later than the current X server time.
    /// Otherwise, the last-change time is set to the specified time.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    ///
    /// # Errors
    ///
    /// * `Atom` - `selection` does not refer to a valid atom.
    ///
    /// # See
    ///
    /// * SetSelectionOwner: request
    fn set_selection_owner(&self, owner: Window, selection: Atom, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_selection_owner(self, owner, selection, time)
    }

    /// Gets the owner of a selection.
    ///
    /// Gets the owner of the specified selection.
    ///
    /// TODO: briefly explain what a selection is.
    ///
    /// # Fields
    ///
    /// * `selection` - The selection.
    ///
    /// # Errors
    ///
    /// * `Atom` - `selection` does not refer to a valid atom.
    ///
    /// # See
    ///
    /// * SetSelectionOwner: request
    fn get_selection_owner(&self, selection: Atom) -> Result<Cookie<'_, Self, GetSelectionOwnerReply>, ConnectionError>
    {
        get_selection_owner(self, selection)
    }

    fn convert_selection(&self, requestor: Window, selection: Atom, target: Atom, property: Atom, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        convert_selection(self, requestor, selection, target, property, time)
    }

    /// send an event.
    ///
    /// Identifies the `destination` window, determines which clients should receive
    /// the specified event and ignores any active grabs.
    ///
    /// The `event` must be one of the core events or an event defined by an extension,
    /// so that the X server can correctly byte-swap the contents as necessary. The
    /// contents of `event` are otherwise unaltered and unchecked except for the
    /// `send_event` field which is forced to 'true'.
    ///
    /// # Fields
    ///
    /// * `destination` - The window to send this event to. Every client which selects any event within
    /// `event_mask` on `destination` will get the event.
    ///
    /// The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
    /// that contains the mouse pointer.
    ///
    /// The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
    /// has the keyboard focus.
    /// * `event` - The event to send to the specified `destination`.
    /// * `event_mask` - Event_mask for determining which clients should receive the specified event.
    /// See `destination` and `propagate`.
    /// * `propagate` - If `propagate` is true and no clients have selected any event on `destination`,
    /// the destination is replaced with the closest ancestor of `destination` for
    /// which some client has selected a type in `event_mask` and for which no
    /// intervening window has that type in its do-not-propagate-mask. If no such
    /// window exists or if the window is an ancestor of the focus window and
    /// `InputFocus` was originally specified as the destination, the event is not sent
    /// to any clients. Otherwise, the event is reported to every client selecting on
    /// the final destination any of the types specified in `event_mask`.
    ///
    /// # Errors
    ///
    /// * `Value` - The given `event` is neither a core event nor an event defined by an extension.
    /// * `Window` - The specified `destination` window does not exist.
    ///
    /// # See
    ///
    /// * ConfigureNotify: event
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Tell the given window that it was configured to a size of 800x600 pixels.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_window_t window) {
    ///     /* Every X11 event is 32 bytes long. Therefore, XCB will copy 32 bytes.
    ///      * In order to properly initialize these bytes, we allocate 32 bytes even
    ///      * though we only need less for an xcb_configure_notify_event_t */
    ///     xcb_configure_notify_event_t *event = calloc(32, 1);
    ///
    ///     event->event = window;
    ///     event->window = window;
    ///     event->response_type = XCB_CONFIGURE_NOTIFY;
    ///
    ///     event->x = 0;
    ///     event->y = 0;
    ///     event->width = 800;
    ///     event->height = 600;
    ///
    ///     event->border_width = 0;
    ///     event->above_sibling = XCB_NONE;
    ///     event->override_redirect = false;
    ///
    ///     xcb_send_event(conn, false, window, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    ///                    (char*)event);
    ///     xcb_flush(conn);
    ///     free(event);
    /// }
    /// ```
    fn send_event<A>(&self, propagate: bool, destination: Window, event_mask: u32, event: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<[u8; 32]>
    {
        send_event(self, propagate, destination, event_mask, event)
    }

    /// Grab the pointer.
    ///
    /// Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
    ///
    /// # Fields
    ///
    /// * `confine_to` - Specifies the window to confine the pointer in (the user will not be able to
    /// move the pointer out of that window).
    ///
    /// The special value `XCB_NONE` means don't confine the pointer.
    /// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
    /// cursor.
    /// * `event_mask` - Specifies which pointer events are reported to the client.
    ///
    /// TODO: which values?
    /// * `grab_window` - Specifies the window on which the pointer should be grabbed.
    /// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    /// * `time` - The time argument allows you to avoid certain circumstances that come up if
    /// applications take a long time to respond or if there are long network delays.
    /// Consider a situation where you have two applications, both of which normally
    /// grab the pointer when clicked on. If both applications specify the timestamp
    /// from the event, the second application may wake up faster and successfully grab
    /// the pointer before the first application. The first application then will get
    /// an indication that the other application grabbed the pointer before its request
    /// was processed.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    ///
    /// # Errors
    ///
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    ///
    /// # See
    ///
    /// * GrabKeyboard: request
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Grabs the pointer actively
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_screen_t *screen, xcb_cursor_t cursor) {
    ///     xcb_grab_pointer_cookie_t cookie;
    ///     xcb_grab_pointer_reply_t *reply;
    ///
    ///     cookie = xcb_grab_pointer(
    ///         conn,
    ///         false,               /* get all pointer events specified by the following mask */
    ///         screen->root,        /* grab the root window */
    ///         XCB_NONE,            /* which events to let through */
    ///         XCB_GRAB_MODE_ASYNC, /* pointer events should continue as normal */
    ///         XCB_GRAB_MODE_ASYNC, /* keyboard mode */
    ///         XCB_NONE,            /* confine_to = in which window should the cursor stay */
    ///         cursor,              /* we change the cursor to whatever the user wanted */
    ///         XCB_CURRENT_TIME
    ///     );
    ///
    ///     if ((reply = xcb_grab_pointer_reply(conn, cookie, NULL))) {
    ///         if (reply->status == XCB_GRAB_STATUS_SUCCESS)
    ///             printf("successfully grabbed the pointer\\n");
    ///         free(preply);
    ///     }
    /// }
    /// ```
    fn grab_pointer<A, B>(&self, owner_events: bool, grab_window: Window, event_mask: u16, pointer_mode: A, keyboard_mode: B, confine_to: Window, cursor: Cursor, time: Timestamp) -> Result<Cookie<'_, Self, GrabPointerReply>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_pointer(self, owner_events, grab_window, event_mask, pointer_mode, keyboard_mode, confine_to, cursor, time)
    }

    /// release the pointer.
    ///
    /// Releases the pointer and any queued events if you actively grabbed the pointer
    /// before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
    /// press.
    ///
    /// EnterNotify and LeaveNotify events are generated.
    ///
    /// # Fields
    ///
    /// * `name` - A pattern describing an X core font.
    /// * `name_len` - Length (in bytes) of `name`.
    /// * `time` - Timestamp to avoid race conditions when running X over the network.
    ///
    /// The pointer will not be released if `time` is earlier than the
    /// last-pointer-grab time or later than the current X server time.
    ///
    /// # See
    ///
    /// * EnterNotify: event
    /// * GrabButton: request
    /// * GrabPointer: request
    /// * LeaveNotify: event
    fn ungrab_pointer(&self, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_pointer(self, time)
    }

    /// Grab pointer button(s).
    ///
    /// This request establishes a passive grab. The pointer is actively grabbed as
    /// described in GrabPointer, the last-pointer-grab time is set to the time at
    /// which the button was pressed (as transmitted in the ButtonPress event), and the
    /// ButtonPress event is reported if all of the following conditions are true:
    ///
    /// The pointer is not grabbed and the specified button is logically pressed when
    /// the specified modifier keys are logically down, and no other buttons or
    /// modifier keys are logically down.
    ///
    /// The grab-window contains the pointer.
    ///
    /// The confine-to window (if any) is viewable.
    ///
    /// A passive grab on the same button/key combination does not exist on any
    /// ancestor of grab-window.
    ///
    /// The interpretation of the remaining arguments is the same as for GrabPointer.
    /// The active grab is terminated automatically when the logical state of the
    /// pointer has all buttons released, independent of the logical state of modifier
    /// keys. Note that the logical state of a device (as seen by means of the
    /// protocol) may lag the physical state if device event processing is frozen. This
    /// request overrides all previous passive grabs by the same client on the same
    /// button/key combinations on the same window. A modifier of AnyModifier is
    /// equivalent to issuing the request for all possible modifier combinations
    /// (including the combination of no modifiers). It is not required that all
    /// specified modifiers have currently assigned keycodes. A button of AnyButton is
    /// equivalent to issuing the request for all possible buttons. Otherwise, it is
    /// not required that the button specified currently be assigned to a physical
    /// button.
    ///
    /// An Access error is generated if some other client has already issued a
    /// GrabButton request with the same button/key combination on the same window.
    /// When using AnyModifier or AnyButton, the request fails completely (no grabs are
    /// established), and an Access error is generated if there is a conflicting grab
    /// for any combination. The request has no effect on an active grab.
    ///
    /// # Fields
    ///
    /// * `confine_to` - Specifies the window to confine the pointer in (the user will not be able to
    /// move the pointer out of that window).
    ///
    /// The special value `XCB_NONE` means don't confine the pointer.
    /// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
    /// cursor.
    /// * `event_mask` - Specifies which pointer events are reported to the client.
    ///
    /// TODO: which values?
    /// * `grab_window` - Specifies the window on which the pointer should be grabbed.
    /// * `modifiers` - The modifiers to grab.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
    /// possible modifier combinations.
    /// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    ///
    /// # Errors
    ///
    /// * `Access` - Another client has already issued a GrabButton with the same button/key
    ///combination on the same window.
    /// * `Cursor` - The specified `cursor` does not exist.
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    fn grab_button<A, B, C>(&self, owner_events: bool, grab_window: Window, event_mask: u16, pointer_mode: A, keyboard_mode: B, confine_to: Window, cursor: Cursor, button: C, modifiers: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>, C: Into<u8>
    {
        grab_button(self, owner_events, grab_window, event_mask, pointer_mode, keyboard_mode, confine_to, cursor, button, modifiers)
    }

    fn ungrab_button<A>(&self, button: A, grab_window: Window, modifiers: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        ungrab_button(self, button, grab_window, modifiers)
    }

    fn change_active_pointer_grab(&self, cursor: Cursor, time: Timestamp, event_mask: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_active_pointer_grab(self, cursor, time, event_mask)
    }

    /// Grab the keyboard.
    ///
    /// Actively grabs control of the keyboard and generates FocusIn and FocusOut
    /// events. Further key events are reported only to the grabbing client.
    ///
    /// Any active keyboard grab by this client is overridden. If the keyboard is
    /// actively grabbed by some other client, `AlreadyGrabbed` is returned. If
    /// `grab_window` is not viewable, `GrabNotViewable` is returned. If the keyboard
    /// is frozen by an active grab of another client, `GrabFrozen` is returned. If the
    /// specified `time` is earlier than the last-keyboard-grab time or later than the
    /// current X server time, `GrabInvalidTime` is returned. Otherwise, the
    /// last-keyboard-grab time is set to the specified time.
    ///
    /// # Fields
    ///
    /// * `grab_window` - Specifies the window on which the pointer should be grabbed.
    /// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    /// * `time` - Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    ///
    /// # Errors
    ///
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    ///
    /// # See
    ///
    /// * GrabPointer: request
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Grabs the keyboard actively
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_screen_t *screen) {
    ///     xcb_grab_keyboard_cookie_t cookie;
    ///     xcb_grab_keyboard_reply_t *reply;
    ///
    ///     cookie = xcb_grab_keyboard(
    ///         conn,
    ///         true,                /* report events */
    ///         screen->root,        /* grab the root window */
    ///         XCB_CURRENT_TIME,
    ///         XCB_GRAB_MODE_ASYNC, /* process events as normal, do not require sync */
    ///         XCB_GRAB_MODE_ASYNC
    ///     );
    ///
    ///     if ((reply = xcb_grab_keyboard_reply(conn, cookie, NULL))) {
    ///         if (reply->status == XCB_GRAB_STATUS_SUCCESS)
    ///             printf("successfully grabbed the keyboard\\n");
    ///
    ///         free(reply);
    ///     }
    /// }
    /// ```
    fn grab_keyboard<A, B>(&self, owner_events: bool, grab_window: Window, time: Timestamp, pointer_mode: A, keyboard_mode: B) -> Result<Cookie<'_, Self, GrabKeyboardReply>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_keyboard(self, owner_events, grab_window, time, pointer_mode, keyboard_mode)
    }

    fn ungrab_keyboard(&self, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_keyboard(self, time)
    }

    /// Grab keyboard key(s).
    ///
    /// Establishes a passive grab on the keyboard. In the future, the keyboard is
    /// actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
    /// the time at which the key was pressed (as transmitted in the KeyPress event),
    /// and the KeyPress event is reported if all of the following conditions are true:
    ///
    /// The keyboard is not grabbed and the specified key (which can itself be a
    /// modifier key) is logically pressed when the specified modifier keys are
    /// logically down, and no other modifier keys are logically down.
    ///
    /// Either the grab_window is an ancestor of (or is) the focus window, or the
    /// grab_window is a descendant of the focus window and contains the pointer.
    ///
    /// A passive grab on the same key combination does not exist on any ancestor of
    /// grab_window.
    ///
    /// The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
    /// automatically when the logical state of the keyboard has the specified key released (independent of the
    /// logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
    ///
    /// Note that the logical state of a device (as seen by client applications) may lag the physical state if
    /// device event processing is frozen.
    ///
    /// A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
    /// have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
    /// all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
    /// and max_keycode in the connection setup, or a BadValue error results.
    ///
    /// If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
    /// error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
    /// results (no grabs are established) if there is a conflicting grab for any combination.
    ///
    /// # Fields
    ///
    /// * `cursor` - Specifies the cursor that should be displayed or `XCB_NONE` to not change the
    /// cursor.
    /// * `grab_window` - Specifies the window on which the pointer should be grabbed.
    /// * `key` - The keycode of the key to grab.
    ///
    /// The special value `XCB_GRAB_ANY` means grab any key.
    /// * `modifiers` - The modifiers to grab.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
    /// possible modifier combinations.
    /// * `owner_events` - If 1, the `grab_window` will still get the pointer events. If 0, events are not
    /// reported to the `grab_window`.
    ///
    /// # Errors
    ///
    /// * `Access` - Another client has already issued a GrabKey with the same button/key
    ///combination on the same window.
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `window` does not exist.
    ///
    /// # See
    ///
    /// * GrabKeyboard: request
    fn grab_key<A, B>(&self, owner_events: bool, grab_window: Window, modifiers: u16, key: Keycode, pointer_mode: A, keyboard_mode: B) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        grab_key(self, owner_events, grab_window, modifiers, key, pointer_mode, keyboard_mode)
    }

    /// release a key combination.
    ///
    /// Releases the key combination on `grab_window` if you grabbed it using
    /// `xcb_grab_key` before.
    ///
    /// # Fields
    ///
    /// * `grab_window` - The window on which the grabbed key combination will be released.
    /// * `key` - The keycode of the specified key combination.
    ///
    /// Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
    /// * `modifiers` - The modifiers of the specified key combination.
    ///
    /// Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
    /// with every possible modifier combination.
    ///
    /// # Errors
    ///
    /// * `Value` - TODO: reasons?
    /// * `Window` - The specified `grab_window` does not exist.
    ///
    /// # See
    ///
    /// * GrabKey: request
    /// * xev: program
    fn ungrab_key(&self, key: Keycode, grab_window: Window, modifiers: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_key(self, key, grab_window, modifiers)
    }

    /// release queued events.
    ///
    /// Releases queued events if the client has caused a device (pointer/keyboard) to
    /// freeze due to grabbing it actively. This request has no effect if `time` is
    /// earlier than the last-grab time of the most recent active grab for this client
    /// or if `time` is later than the current X server time.
    ///
    /// # Fields
    ///
    /// * `time` - Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    ///
    /// # Errors
    ///
    /// * `Value` - You specified an invalid `mode`.
    fn allow_events<A>(&self, mode: A, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        allow_events(self, mode, time)
    }

    fn grab_server(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        grab_server(self)
    }

    fn ungrab_server(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        ungrab_server(self)
    }

    /// get pointer coordinates.
    ///
    /// Gets the root window the pointer is logically on and the pointer coordinates
    /// relative to the root window's origin.
    ///
    /// # Fields
    ///
    /// * `window` - A window to check if the pointer is on the same screen as `window` (see the
    /// `same_screen` field in the reply).
    ///
    /// # Errors
    ///
    /// * `Window` - The specified `window` does not exist.
    fn query_pointer(&self, window: Window) -> Result<Cookie<'_, Self, QueryPointerReply>, ConnectionError>
    {
        query_pointer(self, window)
    }

    fn get_motion_events(&self, window: Window, start: Timestamp, stop: Timestamp) -> Result<Cookie<'_, Self, GetMotionEventsReply>, ConnectionError>
    {
        get_motion_events(self, window, start, stop)
    }

    fn translate_coordinates(&self, src_window: Window, dst_window: Window, src_x: i16, src_y: i16) -> Result<Cookie<'_, Self, TranslateCoordinatesReply>, ConnectionError>
    {
        translate_coordinates(self, src_window, dst_window, src_x, src_y)
    }

    /// move mouse pointer.
    ///
    /// Moves the mouse pointer to the specified position.
    ///
    /// If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
    /// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
    /// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
    /// `src_window`.
    ///
    /// If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
    /// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
    /// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
    /// relative to the current position of the pointer.
    ///
    /// # Fields
    ///
    /// * `dst_window` - If `dst_window` is not `XCB_NONE` (TODO), the pointer will be moved to the
    /// offsets (`dst_x`, `dst_y`) relative to `dst_window`. If `dst_window` is
    /// `XCB_NONE` (TODO), the pointer will be moved by the offsets (`dst_x`, `dst_y`)
    /// relative to the current position of the pointer.
    /// * `src_window` - If `src_window` is not `XCB_NONE` (TODO), the move will only take place if the
    /// pointer is inside `src_window` and within the rectangle specified by (`src_x`,
    /// `src_y`, `src_width`, `src_height`). The rectangle coordinates are relative to
    /// `src_window`.
    ///
    /// # Errors
    ///
    /// * `Window` - TODO: reasons?
    ///
    /// # See
    ///
    /// * SetInputFocus: request
    fn warp_pointer(&self, src_window: Window, dst_window: Window, src_x: i16, src_y: i16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        warp_pointer(self, src_window, dst_window, src_x, src_y, src_width, src_height, dst_x, dst_y)
    }

    /// Sets input focus.
    ///
    /// Changes the input focus and the last-focus-change time. If the specified `time`
    /// is earlier than the current last-focus-change time, the request is ignored (to
    /// avoid race conditions when running X over the network).
    ///
    /// A FocusIn and FocusOut event is generated when focus is changed.
    ///
    /// # Fields
    ///
    /// * `focus` - The window to focus. All keyboard events will be reported to this window. The
    /// window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
    ///
    /// If `focus` is `XCB_NONE` (TODO), all keyboard events are
    /// discarded until a new focus window is set.
    ///
    /// If `focus` is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
    /// screen on which the pointer is on currently.
    /// * `revert_to` - Specifies what happens when the `focus` window becomes unviewable (if `focus`
    /// is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
    /// * `time` - Timestamp to avoid race conditions when running X over the network.
    ///
    /// The special value `XCB_CURRENT_TIME` will be replaced with the current server
    /// time.
    ///
    /// # Errors
    ///
    /// * `Match` - The specified `focus` window is not viewable.
    /// * `Value` - TODO: Reasons?
    /// * `Window` - The specified `focus` window does not exist.
    ///
    /// # See
    ///
    /// * FocusIn: event
    /// * FocusOut: event
    fn set_input_focus<A>(&self, revert_to: A, focus: Window, time: Timestamp) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        set_input_focus(self, revert_to, focus, time)
    }

    fn get_input_focus(&self) -> Result<Cookie<'_, Self, GetInputFocusReply>, ConnectionError>
    {
        get_input_focus(self)
    }

    fn query_keymap(&self) -> Result<Cookie<'_, Self, QueryKeymapReply>, ConnectionError>
    {
        query_keymap(self)
    }

    /// opens a font.
    ///
    /// Opens any X core font matching the given `name` (for example "-misc-fixed-*").
    ///
    /// Note that X core fonts are deprecated (but still supported) in favor of
    /// client-side rendering using Xft.
    ///
    /// # Fields
    ///
    /// * `fid` - The ID with which you will refer to the font, created by `xcb_generate_id`.
    /// * `name` - A pattern describing an X core font.
    /// * `name_len` - Length (in bytes) of `name`.
    ///
    /// # Errors
    ///
    /// * `Name` - No font matches the given `name`.
    ///
    /// # See
    ///
    /// * xcb_generate_id: function
    fn open_font<'c>(&'c self, fid: Font, name: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        open_font(self, fid, name)
    }

    fn close_font(&self, font: Font) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        close_font(self, font)
    }

    /// query font metrics.
    ///
    /// Queries information associated with the font.
    ///
    /// # Fields
    ///
    /// * `font` - The fontable (Font or Graphics Context) to query.
    fn query_font(&self, font: Fontable) -> Result<Cookie<'_, Self, QueryFontReply>, ConnectionError>
    {
        query_font(self, font)
    }

    /// get text extents.
    ///
    /// Query text extents from the X11 server. This request returns the bounding box
    /// of the specified 16-bit character string in the specified `font` or the font
    /// contained in the specified graphics context.
    ///
    /// `font_ascent` is set to the maximum of the ascent metrics of all characters in
    /// the string. `font_descent` is set to the maximum of the descent metrics.
    /// `overall_width` is set to the sum of the character-width metrics of all
    /// characters in the string. For each character in the string, let W be the sum of
    /// the character-width metrics of all characters preceding it in the string. Let L
    /// be the left-side-bearing metric of the character plus W. Let R be the
    /// right-side-bearing metric of the character plus W. The lbearing member is set
    /// to the minimum L of all characters in the string. The rbearing member is set to
    /// the maximum R.
    ///
    /// For fonts defined with linear indexing rather than 2-byte matrix indexing, each
    /// `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
    /// most significant byte. If the font has no defined default character, undefined
    /// characters in the string are taken to have all zero metrics.
    ///
    /// Characters with all zero metrics are ignored. If the font has no defined
    /// default_char, the undefined characters in the string are also ignored.
    ///
    /// # Fields
    ///
    /// * `font` - The `font` to calculate text extents in. You can also pass a graphics context.
    /// * `string` - The text to get text extents for.
    /// * `string_len` - The number of characters in `string`.
    ///
    /// # Errors
    ///
    /// * `Font` - The specified `font` does not exist.
    /// * `GContext` - The specified graphics context does not exist.
    fn query_text_extents<'c>(&'c self, font: Fontable, string: &[Char2b]) -> Result<Cookie<'c, Self, QueryTextExtentsReply>, ConnectionError>
    {
        query_text_extents(self, font, string)
    }

    /// get matching font names.
    ///
    /// Gets a list of available font names which match the given `pattern`.
    ///
    /// # Fields
    ///
    /// * `max_names` - The maximum number of fonts to be returned.
    /// * `pattern` - A font pattern, for example "-misc-fixed-*".
    ///
    /// The asterisk (*) is a wildcard for any number of characters. The question mark
    /// (?) is a wildcard for a single character. Use of uppercase or lowercase does
    /// not matter.
    /// * `pattern_len` - The length (in bytes) of `pattern`.
    fn list_fonts<'c>(&'c self, max_names: u16, pattern: &[u8]) -> Result<Cookie<'c, Self, ListFontsReply>, ConnectionError>
    {
        list_fonts(self, max_names, pattern)
    }

    /// get matching font names and information.
    ///
    /// Gets a list of available font names which match the given `pattern`.
    ///
    /// # Fields
    ///
    /// * `max_names` - The maximum number of fonts to be returned.
    /// * `pattern` - A font pattern, for example "-misc-fixed-*".
    ///
    /// The asterisk (*) is a wildcard for any number of characters. The question mark
    /// (?) is a wildcard for a single character. Use of uppercase or lowercase does
    /// not matter.
    /// * `pattern_len` - The length (in bytes) of `pattern`.
    fn list_fonts_with_info<'c>(&'c self, max_names: u16, pattern: &[u8]) -> Result<ListFontsWithInfoCookie<'c, Self>, ConnectionError>
    {
        list_fonts_with_info(self, max_names, pattern)
    }

    fn set_font_path<'c>(&'c self, font: &[Str]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_font_path(self, font)
    }

    fn get_font_path(&self) -> Result<Cookie<'_, Self, GetFontPathReply>, ConnectionError>
    {
        get_font_path(self)
    }

    /// Creates a pixmap.
    ///
    /// Creates a pixmap. The pixmap can only be used on the same screen as `drawable`
    /// is on and only with drawables of the same `depth`.
    ///
    /// # Fields
    ///
    /// * `depth` - TODO
    /// * `drawable` - Drawable to get the screen from.
    /// * `height` - The height of the new pixmap.
    /// * `pid` - The ID with which you will refer to the new pixmap, created by
    /// `xcb_generate_id`.
    /// * `width` - The width of the new pixmap.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not allocate the requested resources (no memory?).
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `Value` - TODO: reasons?
    ///
    /// # See
    ///
    /// * xcb_generate_id: function
    fn create_pixmap(&self, depth: u8, pid: Pixmap, drawable: Drawable, width: u16, height: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_pixmap(self, depth, pid, drawable, width, height)
    }

    /// Destroys a pixmap.
    ///
    /// Deletes the association between the pixmap ID and the pixmap. The pixmap
    /// storage will be freed when there are no more references to it.
    ///
    /// # Fields
    ///
    /// * `pixmap` - The pixmap to destroy.
    ///
    /// # Errors
    ///
    /// * `Pixmap` - The specified pixmap does not exist.
    fn free_pixmap(&self, pixmap: Pixmap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_pixmap(self, pixmap)
    }

    /// Creates a graphics context.
    ///
    /// Creates a graphics context. The graphics context can be used with any drawable
    /// that has the same root and depth as the specified drawable.
    ///
    /// # Fields
    ///
    /// * `cid` - The ID with which you will refer to the graphics context, created by
    /// `xcb_generate_id`.
    /// * `drawable` - Drawable to get the root/depth from.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not allocate the requested resources (no memory?).
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `Font` - TODO: reasons?
    /// * `Match` - TODO: reasons?
    /// * `Pixmap` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    ///
    /// # See
    ///
    /// * xcb_generate_id: function
    fn create_gc<'c>(&'c self, cid: Gcontext, drawable: Drawable, value_list: &CreateGCAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_gc(self, cid, drawable, value_list)
    }

    /// change graphics context components.
    ///
    /// Changes the components specified by `value_mask` for the specified graphics context.
    ///
    /// # Fields
    ///
    /// * `gc` - The graphics context to change.
    /// * `value_list` - Values for each of the components specified in the bitmask `value_mask`. The
    /// order has to correspond to the order of possible `value_mask` bits. See the
    /// example.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not allocate the requested resources (no memory?).
    /// * `Font` - TODO: reasons?
    /// * `GContext` - TODO: reasons?
    /// * `Match` - TODO: reasons?
    /// * `Pixmap` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Changes the foreground color component of the specified graphics context.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_gcontext_t gc, uint32_t fg, uint32_t bg) {
    ///     /* C99 allows us to use a compact way of changing a single component: */
    ///     xcb_change_gc(conn, gc, XCB_GC_FOREGROUND, (uint32_t[]){ fg });
    ///
    ///     /* The more explicit way. Beware that the order of values is important! */
    ///     uint32_t mask = 0;
    ///     mask |= XCB_GC_FOREGROUND;
    ///     mask |= XCB_GC_BACKGROUND;
    ///
    ///     uint32_t values[] = {
    ///         fg,
    ///         bg
    ///     };
    ///     xcb_change_gc(conn, gc, mask, values);
    ///     xcb_flush(conn);
    /// }
    /// ```
    fn change_gc<'c>(&'c self, gc: Gcontext, value_list: &ChangeGCAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_gc(self, gc, value_list)
    }

    fn copy_gc(&self, src_gc: Gcontext, dst_gc: Gcontext, value_mask: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_gc(self, src_gc, dst_gc, value_mask)
    }

    fn set_dashes<'c>(&'c self, gc: Gcontext, dash_offset: u16, dashes: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_dashes(self, gc, dash_offset, dashes)
    }

    fn set_clip_rectangles<'c, A>(&'c self, ordering: A, gc: Gcontext, clip_x_origin: i16, clip_y_origin: i16, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        set_clip_rectangles(self, ordering, gc, clip_x_origin, clip_y_origin, rectangles)
    }

    /// Destroys a graphics context.
    ///
    /// Destroys the specified `gc` and all associated storage.
    ///
    /// # Fields
    ///
    /// * `gc` - The graphics context to destroy.
    ///
    /// # Errors
    ///
    /// * `GContext` - The specified graphics context does not exist.
    fn free_gc(&self, gc: Gcontext) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_gc(self, gc)
    }

    fn clear_area(&self, exposures: bool, window: Window, x: i16, y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        clear_area(self, exposures, window, x, y, width, height)
    }

    /// copy areas.
    ///
    /// Copies the specified rectangle from `src_drawable` to `dst_drawable`.
    ///
    /// # Fields
    ///
    /// * `dst_drawable` - The destination drawable (Window or Pixmap).
    /// * `dst_x` - The destination X coordinate.
    /// * `dst_y` - The destination Y coordinate.
    /// * `gc` - The graphics context to use.
    /// * `height` - The height of the area to copy (in pixels).
    /// * `src_drawable` - The source drawable (Window or Pixmap).
    /// * `src_x` - The source X coordinate.
    /// * `src_y` - The source Y coordinate.
    /// * `width` - The width of the area to copy (in pixels).
    ///
    /// # Errors
    ///
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `GContext` - The specified graphics context does not exist.
    /// * `Match` - `src_drawable` has a different root or depth than `dst_drawable`.
    fn copy_area(&self, src_drawable: Drawable, dst_drawable: Drawable, gc: Gcontext, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_area(self, src_drawable, dst_drawable, gc, src_x, src_y, dst_x, dst_y, width, height)
    }

    fn copy_plane(&self, src_drawable: Drawable, dst_drawable: Drawable, gc: Gcontext, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16, bit_plane: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_plane(self, src_drawable, dst_drawable, gc, src_x, src_y, dst_x, dst_y, width, height, bit_plane)
    }

    fn poly_point<'c, A>(&'c self, coordinate_mode: A, drawable: Drawable, gc: Gcontext, points: &[Point]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        poly_point(self, coordinate_mode, drawable, gc, points)
    }

    /// draw lines.
    ///
    /// Draws `points_len`-1 lines between each pair of points (point[i], point[i+1])
    /// in the `points` array. The lines are drawn in the order listed in the array.
    /// They join correctly at all intermediate points, and if the first and last
    /// points coincide, the first and last lines also join correctly. For any given
    /// line, a pixel is not drawn more than once. If thin (zero line-width) lines
    /// intersect, the intersecting pixels are drawn multiple times. If wide lines
    /// intersect, the intersecting pixels are drawn only once, as though the entire
    /// request were a single, filled shape.
    ///
    /// # Fields
    ///
    /// * `drawable` - The drawable to draw the line(s) on.
    /// * `gc` - The graphics context to use.
    /// * `points` - An array of points.
    /// * `points_len` - The number of `xcb_point_t` structures in `points`.
    ///
    /// # Errors
    ///
    /// * `Drawable` - TODO: reasons?
    /// * `GContext` - TODO: reasons?
    /// * `Match` - TODO: reasons?
    /// * `Value` - TODO: reasons?
    ///
    /// # Example
    ///
    /// ```text
    /// /*
    ///  * Draw a straight line.
    ///  *
    ///  */
    /// void my_example(xcb_connection_t *conn, xcb_drawable_t drawable, xcb_gcontext_t gc) {
    ///     xcb_poly_line(conn, XCB_COORD_MODE_ORIGIN, drawable, gc, 2,
    ///                   (xcb_point_t[]) { {10, 10}, {100, 10} });
    ///     xcb_flush(conn);
    /// }
    /// ```
    fn poly_line<'c, A>(&'c self, coordinate_mode: A, drawable: Drawable, gc: Gcontext, points: &[Point]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        poly_line(self, coordinate_mode, drawable, gc, points)
    }

    /// draw lines.
    ///
    /// Draws multiple, unconnected lines. For each segment, a line is drawn between
    /// (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
    /// `xcb_segment_t` structures and does not perform joining at coincident
    /// endpoints. For any given line, a pixel is not drawn more than once. If lines
    /// intersect, the intersecting pixels are drawn multiple times.
    ///
    /// TODO: include the xcb_segment_t data structure
    ///
    /// TODO: an example
    ///
    /// # Fields
    ///
    /// * `drawable` - A drawable (Window or Pixmap) to draw on.
    /// * `gc` - The graphics context to use.
    ///
    /// TODO: document which attributes of a gc are used
    /// * `segments` - An array of `xcb_segment_t` structures.
    /// * `segments_len` - The number of `xcb_segment_t` structures in `segments`.
    ///
    /// # Errors
    ///
    /// * `Drawable` - The specified `drawable` does not exist.
    /// * `GContext` - The specified `gc` does not exist.
    /// * `Match` - TODO: reasons?
    fn poly_segment<'c>(&'c self, drawable: Drawable, gc: Gcontext, segments: &[Segment]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_segment(self, drawable, gc, segments)
    }

    fn poly_rectangle<'c>(&'c self, drawable: Drawable, gc: Gcontext, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_rectangle(self, drawable, gc, rectangles)
    }

    fn poly_arc<'c>(&'c self, drawable: Drawable, gc: Gcontext, arcs: &[Arc]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_arc(self, drawable, gc, arcs)
    }

    fn fill_poly<'c, A, B>(&'c self, drawable: Drawable, gc: Gcontext, shape: A, coordinate_mode: B, points: &[Point]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        fill_poly(self, drawable, gc, shape, coordinate_mode, points)
    }

    /// Fills rectangles.
    ///
    /// Fills the specified rectangle(s) in the order listed in the array. For any
    /// given rectangle, each pixel is not drawn more than once. If rectangles
    /// intersect, the intersecting pixels are drawn multiple times.
    ///
    /// # Fields
    ///
    /// * `drawable` - The drawable (Window or Pixmap) to draw on.
    /// * `gc` - The graphics context to use.
    ///
    /// The following graphics context components are used: function, plane-mask,
    /// fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    ///
    /// The following graphics context mode-dependent components are used:
    /// foreground, background, tile, stipple, tile-stipple-x-origin, and
    /// tile-stipple-y-origin.
    /// * `rectangles` - The rectangles to fill.
    /// * `rectangles_len` - The number of `xcb_rectangle_t` structures in `rectangles`.
    ///
    /// # Errors
    ///
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `GContext` - The specified graphics context does not exist.
    /// * `Match` - TODO: reasons?
    fn poly_fill_rectangle<'c>(&'c self, drawable: Drawable, gc: Gcontext, rectangles: &[Rectangle]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_fill_rectangle(self, drawable, gc, rectangles)
    }

    fn poly_fill_arc<'c>(&'c self, drawable: Drawable, gc: Gcontext, arcs: &[Arc]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_fill_arc(self, drawable, gc, arcs)
    }

    fn put_image<'c, A>(&'c self, format: A, drawable: Drawable, gc: Gcontext, width: u16, height: u16, dst_x: i16, dst_y: i16, left_pad: u8, depth: u8, data: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>
    {
        put_image(self, format, drawable, gc, width, height, dst_x, dst_y, left_pad, depth, data)
    }

    fn get_image<A>(&self, format: A, drawable: Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32) -> Result<Cookie<'_, Self, GetImageReply>, ConnectionError>
    where A: Into<u8>
    {
        get_image(self, format, drawable, x, y, width, height, plane_mask)
    }

    fn poly_text8<'c>(&'c self, drawable: Drawable, gc: Gcontext, x: i16, y: i16, items: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_text8(self, drawable, gc, x, y, items)
    }

    fn poly_text16<'c>(&'c self, drawable: Drawable, gc: Gcontext, x: i16, y: i16, items: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        poly_text16(self, drawable, gc, x, y, items)
    }

    /// Draws text.
    ///
    /// Fills the destination rectangle with the background pixel from `gc`, then
    /// paints the text with the foreground pixel from `gc`. The upper-left corner of
    /// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
    /// the height is font-ascent + font-descent. The overall-width, font-ascent and
    /// font-descent are as returned by `xcb_query_text_extents` (TODO).
    ///
    /// Note that using X core fonts is deprecated (but still supported) in favor of
    /// client-side rendering using Xft.
    ///
    /// # Fields
    ///
    /// * `drawable` - The drawable (Window or Pixmap) to draw text on.
    /// * `gc` - The graphics context to use.
    ///
    /// The following graphics context components are used: plane-mask, foreground,
    /// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    /// * `string` - The string to draw. Only the first 255 characters are relevant due to the data
    /// type of `string_len`.
    /// * `string_len` - The length of the `string`. Note that this parameter limited by 255 due to
    /// using 8 bits!
    /// * `x` - The x coordinate of the first character, relative to the origin of `drawable`.
    /// * `y` - The y coordinate of the first character, relative to the origin of `drawable`.
    ///
    /// # Errors
    ///
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `GContext` - The specified graphics context does not exist.
    /// * `Match` - TODO: reasons?
    ///
    /// # See
    ///
    /// * ImageText16: request
    fn image_text8<'c>(&'c self, drawable: Drawable, gc: Gcontext, x: i16, y: i16, string: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        image_text8(self, drawable, gc, x, y, string)
    }

    /// Draws text.
    ///
    /// Fills the destination rectangle with the background pixel from `gc`, then
    /// paints the text with the foreground pixel from `gc`. The upper-left corner of
    /// the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
    /// the height is font-ascent + font-descent. The overall-width, font-ascent and
    /// font-descent are as returned by `xcb_query_text_extents` (TODO).
    ///
    /// Note that using X core fonts is deprecated (but still supported) in favor of
    /// client-side rendering using Xft.
    ///
    /// # Fields
    ///
    /// * `drawable` - The drawable (Window or Pixmap) to draw text on.
    /// * `gc` - The graphics context to use.
    ///
    /// The following graphics context components are used: plane-mask, foreground,
    /// background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
    /// * `string` - The string to draw. Only the first 255 characters are relevant due to the data
    /// type of `string_len`. Every character uses 2 bytes (hence the 16 in this
    /// request's name).
    /// * `string_len` - The length of the `string` in characters. Note that this parameter limited by
    /// 255 due to using 8 bits!
    /// * `x` - The x coordinate of the first character, relative to the origin of `drawable`.
    /// * `y` - The y coordinate of the first character, relative to the origin of `drawable`.
    ///
    /// # Errors
    ///
    /// * `Drawable` - The specified `drawable` (Window or Pixmap) does not exist.
    /// * `GContext` - The specified graphics context does not exist.
    /// * `Match` - TODO: reasons?
    ///
    /// # See
    ///
    /// * ImageText8: request
    fn image_text16<'c>(&'c self, drawable: Drawable, gc: Gcontext, x: i16, y: i16, string: &[Char2b]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        image_text16(self, drawable, gc, x, y, string)
    }

    fn create_colormap<A>(&self, alloc: A, mid: Colormap, window: Window, visual: Visualid) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        create_colormap(self, alloc, mid, window, visual)
    }

    fn free_colormap(&self, cmap: Colormap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_colormap(self, cmap)
    }

    fn copy_colormap_and_free(&self, mid: Colormap, src_cmap: Colormap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        copy_colormap_and_free(self, mid, src_cmap)
    }

    fn install_colormap(&self, cmap: Colormap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        install_colormap(self, cmap)
    }

    fn uninstall_colormap(&self, cmap: Colormap) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        uninstall_colormap(self, cmap)
    }

    fn list_installed_colormaps(&self, window: Window) -> Result<Cookie<'_, Self, ListInstalledColormapsReply>, ConnectionError>
    {
        list_installed_colormaps(self, window)
    }

    /// Allocate a color.
    ///
    /// Allocates a read-only colormap entry corresponding to the closest RGB value
    /// supported by the hardware. If you are using TrueColor, you can take a shortcut
    /// and directly calculate the color pixel value to avoid the round trip. But, for
    /// example, on 16-bit color setups (VNC), you can easily get the closest supported
    /// RGB value to the RGB value you are specifying.
    ///
    /// # Fields
    ///
    /// * `blue` - The blue value of your color.
    /// * `cmap` - TODO
    /// * `green` - The green value of your color.
    /// * `red` - The red value of your color.
    ///
    /// # Errors
    ///
    /// * `Colormap` - The specified colormap `cmap` does not exist.
    fn alloc_color(&self, cmap: Colormap, red: u16, green: u16, blue: u16) -> Result<Cookie<'_, Self, AllocColorReply>, ConnectionError>
    {
        alloc_color(self, cmap, red, green, blue)
    }

    fn alloc_named_color<'c>(&'c self, cmap: Colormap, name: &[u8]) -> Result<Cookie<'c, Self, AllocNamedColorReply>, ConnectionError>
    {
        alloc_named_color(self, cmap, name)
    }

    fn alloc_color_cells(&self, contiguous: bool, cmap: Colormap, colors: u16, planes: u16) -> Result<Cookie<'_, Self, AllocColorCellsReply>, ConnectionError>
    {
        alloc_color_cells(self, contiguous, cmap, colors, planes)
    }

    fn alloc_color_planes(&self, contiguous: bool, cmap: Colormap, colors: u16, reds: u16, greens: u16, blues: u16) -> Result<Cookie<'_, Self, AllocColorPlanesReply>, ConnectionError>
    {
        alloc_color_planes(self, contiguous, cmap, colors, reds, greens, blues)
    }

    fn free_colors<'c>(&'c self, cmap: Colormap, plane_mask: u32, pixels: &[u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        free_colors(self, cmap, plane_mask, pixels)
    }

    fn store_colors<'c>(&'c self, cmap: Colormap, items: &[Coloritem]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        store_colors(self, cmap, items)
    }

    fn store_named_color<'c>(&'c self, flags: u8, cmap: Colormap, pixel: u32, name: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        store_named_color(self, flags, cmap, pixel, name)
    }

    fn query_colors<'c>(&'c self, cmap: Colormap, pixels: &[u32]) -> Result<Cookie<'c, Self, QueryColorsReply>, ConnectionError>
    {
        query_colors(self, cmap, pixels)
    }

    fn lookup_color<'c>(&'c self, cmap: Colormap, name: &[u8]) -> Result<Cookie<'c, Self, LookupColorReply>, ConnectionError>
    {
        lookup_color(self, cmap, name)
    }

    fn create_cursor(&self, cid: Cursor, source: Pixmap, mask: Pixmap, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16, x: u16, y: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_cursor(self, cid, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue, x, y)
    }

    /// create cursor.
    ///
    /// Creates a cursor from a font glyph. X provides a set of standard cursor shapes
    /// in a special font named cursor. Applications are encouraged to use this
    /// interface for their cursors because the font can be customized for the
    /// individual display type.
    ///
    /// All pixels which are set to 1 in the source will use the foreground color (as
    /// specified by `fore_red`, `fore_green` and `fore_blue`). All pixels set to 0
    /// will use the background color (as specified by `back_red`, `back_green` and
    /// `back_blue`).
    ///
    /// # Fields
    ///
    /// * `back_blue` - The blue value of the background color.
    /// * `back_green` - The green value of the background color.
    /// * `back_red` - The red value of the background color.
    /// * `cid` - The ID with which you will refer to the cursor, created by `xcb_generate_id`.
    /// * `fore_blue` - The blue value of the foreground color.
    /// * `fore_green` - The green value of the foreground color.
    /// * `fore_red` - The red value of the foreground color.
    /// * `mask_char` - The glyph of `mask_font` to use as a mask: Pixels which are set to 1 define
    /// which source pixels are displayed. All pixels which are set to 0 are not
    /// displayed.
    /// * `mask_font` - In which font to look for the mask glyph.
    /// * `source_char` - The glyph of `source_font` to use.
    /// * `source_font` - In which font to look for the cursor glyph.
    ///
    /// # Errors
    ///
    /// * `Alloc` - The X server could not allocate the requested resources (no memory?).
    /// * `Font` - The specified `source_font` or `mask_font` does not exist.
    /// * `Value` - Either `source_char` or `mask_char` are not defined in `source_font` or `mask_font`, respectively.
    fn create_glyph_cursor(&self, cid: Cursor, source_font: Font, mask_font: Font, source_char: u16, mask_char: u16, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        create_glyph_cursor(self, cid, source_font, mask_font, source_char, mask_char, fore_red, fore_green, fore_blue, back_red, back_green, back_blue)
    }

    /// Deletes a cursor.
    ///
    /// Deletes the association between the cursor resource ID and the specified
    /// cursor. The cursor is freed when no other resource references it.
    ///
    /// # Fields
    ///
    /// * `cursor` - The cursor to destroy.
    ///
    /// # Errors
    ///
    /// * `Cursor` - The specified cursor does not exist.
    fn free_cursor(&self, cursor: Cursor) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        free_cursor(self, cursor)
    }

    fn recolor_cursor(&self, cursor: Cursor, fore_red: u16, fore_green: u16, fore_blue: u16, back_red: u16, back_green: u16, back_blue: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        recolor_cursor(self, cursor, fore_red, fore_green, fore_blue, back_red, back_green, back_blue)
    }

    fn query_best_size<A>(&self, class: A, drawable: Drawable, width: u16, height: u16) -> Result<Cookie<'_, Self, QueryBestSizeReply>, ConnectionError>
    where A: Into<u8>
    {
        query_best_size(self, class, drawable, width, height)
    }

    /// check if extension is present.
    ///
    /// Determines if the specified extension is present on this X11 server.
    ///
    /// Every extension has a unique `major_opcode` to identify requests, the minor
    /// opcodes and request formats are extension-specific. If the extension provides
    /// events and errors, the `first_event` and `first_error` fields in the reply are
    /// set accordingly.
    ///
    /// There should rarely be a need to use this request directly, XCB provides the
    /// `xcb_get_extension_data` function instead.
    ///
    /// # Fields
    ///
    /// * `name` - The name of the extension to query, for example "RANDR". This is case
    /// sensitive!
    /// * `name_len` - The length of `name` in bytes.
    ///
    /// # See
    ///
    /// * xcb_get_extension_data: function
    /// * xdpyinfo: program
    fn query_extension<'c>(&'c self, name: &[u8]) -> Result<Cookie<'c, Self, QueryExtensionReply>, ConnectionError>
    {
        query_extension(self, name)
    }

    fn list_extensions(&self) -> Result<Cookie<'_, Self, ListExtensionsReply>, ConnectionError>
    {
        list_extensions(self)
    }

    fn change_keyboard_mapping<'c>(&'c self, keycode_count: u8, first_keycode: Keycode, keysyms_per_keycode: u8, keysyms: &[Keysym]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_keyboard_mapping(self, keycode_count, first_keycode, keysyms_per_keycode, keysyms)
    }

    fn get_keyboard_mapping(&self, first_keycode: Keycode, count: u8) -> Result<Cookie<'_, Self, GetKeyboardMappingReply>, ConnectionError>
    {
        get_keyboard_mapping(self, first_keycode, count)
    }

    fn change_keyboard_control<'c>(&'c self, value_list: &ChangeKeyboardControlAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_keyboard_control(self, value_list)
    }

    fn get_keyboard_control(&self) -> Result<Cookie<'_, Self, GetKeyboardControlReply>, ConnectionError>
    {
        get_keyboard_control(self)
    }

    fn bell(&self, percent: i8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        bell(self, percent)
    }

    fn change_pointer_control(&self, acceleration_numerator: i16, acceleration_denominator: i16, threshold: i16, do_acceleration: bool, do_threshold: bool) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        change_pointer_control(self, acceleration_numerator, acceleration_denominator, threshold, do_acceleration, do_threshold)
    }

    fn get_pointer_control(&self) -> Result<Cookie<'_, Self, GetPointerControlReply>, ConnectionError>
    {
        get_pointer_control(self)
    }

    fn set_screen_saver<A, B>(&self, timeout: i16, interval: i16, prefer_blanking: A, allow_exposures: B) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        set_screen_saver(self, timeout, interval, prefer_blanking, allow_exposures)
    }

    fn get_screen_saver(&self) -> Result<Cookie<'_, Self, GetScreenSaverReply>, ConnectionError>
    {
        get_screen_saver(self)
    }

    fn change_hosts<'c, A, B>(&'c self, mode: A, family: B, address: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where A: Into<u8>, B: Into<u8>
    {
        change_hosts(self, mode, family, address)
    }

    fn list_hosts(&self) -> Result<Cookie<'_, Self, ListHostsReply>, ConnectionError>
    {
        list_hosts(self)
    }

    fn set_access_control<A>(&self, mode: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        set_access_control(self, mode)
    }

    fn set_close_down_mode<A>(&self, mode: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        set_close_down_mode(self, mode)
    }

    /// kills a client.
    ///
    /// Forces a close down of the client that created the specified `resource`.
    ///
    /// # Fields
    ///
    /// * `resource` - Any resource belonging to the client (for example a Window), used to identify
    /// the client connection.
    ///
    /// The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
    /// that have terminated in `RetainTemporary` (TODO) are destroyed.
    ///
    /// # Errors
    ///
    /// * `Value` - The specified `resource` does not exist.
    ///
    /// # See
    ///
    /// * xkill: program
    fn kill_client(&self, resource: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        kill_client(self, resource)
    }

    fn rotate_properties<'c>(&'c self, window: Window, delta: i16, atoms: &[Atom]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        rotate_properties(self, window, delta, atoms)
    }

    fn force_screen_saver<A>(&self, mode: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where A: Into<u8>
    {
        force_screen_saver(self, mode)
    }

    fn set_pointer_mapping<'c>(&'c self, map: &[u8]) -> Result<Cookie<'c, Self, SetPointerMappingReply>, ConnectionError>
    {
        set_pointer_mapping(self, map)
    }

    fn get_pointer_mapping(&self) -> Result<Cookie<'_, Self, GetPointerMappingReply>, ConnectionError>
    {
        get_pointer_mapping(self)
    }

    fn set_modifier_mapping<'c>(&'c self, keycodes: &[Keycode]) -> Result<Cookie<'c, Self, SetModifierMappingReply>, ConnectionError>
    {
        set_modifier_mapping(self, keycodes)
    }

    fn get_modifier_mapping(&self) -> Result<Cookie<'_, Self, GetModifierMappingReply>, ConnectionError>
    {
        get_modifier_mapping(self)
    }

    fn no_operation(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        no_operation(self)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
