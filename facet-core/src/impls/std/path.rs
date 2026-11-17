use alloc::borrow::Cow;
use alloc::string::String;

use crate::{
    Def, Facet, ProxyDef, PtrConst, PtrMut, PtrUninit, Shape, ShapeBuilder, TryFromOutcome, Type,
    UserType, VTableDirect, VTableIndirect, vtable_direct, vtable_indirect,
};

/// Try to convert from &str or String to PathBuf
///
/// # Safety
/// `dst` must be valid for writes, `src` must point to valid data of type described by `src_shape`
unsafe fn pathbuf_try_from(
    dst: *mut std::path::PathBuf,
    src_shape: &'static Shape,
    src: PtrConst,
) -> TryFromOutcome {
    // Check if source is &str (Copy type, use get)
    if src_shape.id == <&str as Facet>::SHAPE.id {
        let str_ref: &str = unsafe { src.get::<&str>() };
        unsafe { dst.write(std::path::PathBuf::from(str_ref)) };
        return TryFromOutcome::Converted;
    }

    // Check if source is String (consume via read)
    if src_shape.id == <alloc::string::String as Facet>::SHAPE.id {
        let string: alloc::string::String = unsafe { src.read::<alloc::string::String>() };
        unsafe { dst.write(std::path::PathBuf::from(string)) };
        return TryFromOutcome::Converted;
    }

    TryFromOutcome::Unsupported
}

/// Parse a PathBuf from a string
///
/// # Safety
/// `target` must be valid for writes
unsafe fn pathbuf_parse(s: &str, target: *mut std::path::PathBuf) -> Result<(), crate::ParseError> {
    // PathBuf::from never fails - any string is a valid path
    unsafe { target.write(std::path::PathBuf::from(s)) };
    Ok(())
}

/// Convert a path to its UTF-8 string representation without lossy replacement.
///
/// # Safety
/// `target_ptr` must point to an initialized `PathBuf`. `proxy_ptr` must be
/// aligned, uninitialized storage suitable for a `Cow<'_, str>`. The target
/// must remain alive and immutably borrowed while the proxy or its borrows exist.
unsafe fn pathbuf_proxy_convert_out(
    target_ptr: PtrConst,
    proxy_ptr: PtrUninit,
) -> Result<PtrMut, String> {
    // SAFETY: the proxy contract supplies an initialized PathBuf of this shape.
    let path = unsafe { target_ptr.get::<std::path::PathBuf>() };
    let text = path
        .to_str()
        .ok_or_else(|| String::from("path contains non-UTF-8 data"))?;
    // SAFETY: the proxy shape is Cow<str> and its storage is uninitialized.
    // The conversion contract retains the target for this borrowed proxy.
    Ok(unsafe { proxy_ptr.put(Cow::Borrowed(text)) })
}

/// Consume the string proxy and construct an owned path.
///
/// # Safety
/// `proxy_ptr` must point to an initialized `Cow<'_, str>` that may be consumed.
/// `target_ptr` must be aligned, uninitialized storage suitable for a `PathBuf`.
unsafe fn pathbuf_proxy_convert_in(
    proxy_ptr: PtrConst,
    target_ptr: PtrUninit,
) -> Result<PtrMut, String> {
    // SAFETY: the proxy contract transfers ownership of its initialized Cow<str>.
    let text = unsafe { proxy_ptr.read::<Cow<'_, str>>() };
    // SAFETY: the target shape is PathBuf and its storage is uninitialized.
    Ok(unsafe { target_ptr.put(std::path::PathBuf::from(text.into_owned())) })
}

const PATHBUF_PROXY: ProxyDef = ProxyDef {
    // Only metadata uses 'static; the conversion writes a borrow of the target.
    shape: <Cow<'static, str> as Facet>::SHAPE,
    convert_in: pathbuf_proxy_convert_in,
    convert_out: pathbuf_proxy_convert_out,
};

unsafe impl Facet<'_> for std::path::PathBuf {
    const SHAPE: &'static Shape = &const {
        const VTABLE: VTableDirect = vtable_direct!(std::path::PathBuf =>
            Debug,
            Hash,
            PartialEq,
            PartialOrd,
            Ord,
            [parse = pathbuf_parse],
            [try_from = pathbuf_try_from],
        );

        ShapeBuilder::for_sized::<std::path::PathBuf>("PathBuf")
            .module_path("std::path")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_direct(&VTABLE)
            .proxy(&PATHBUF_PROXY)
            .eq()
            .send()
            .sync()
            .build()
    };
}

unsafe impl Facet<'_> for std::path::Path {
    const SHAPE: &'static Shape = &const {
        const VTABLE: VTableIndirect = vtable_indirect!(std::path::Path =>
            Debug,
            Hash,
            PartialEq,
            PartialOrd,
            Ord,
        );

        ShapeBuilder::for_unsized::<std::path::Path>("Path")
            .module_path("std::path")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_indirect(&VTABLE)
            .eq()
            .send()
            .sync()
            .build()
    };
}
