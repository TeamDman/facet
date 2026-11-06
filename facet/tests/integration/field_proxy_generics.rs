use facet::{Facet, Field, ProxyDef, PtrConst, PtrUninit, Type, UserType};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Clone, Debug, PartialEq, Facet)]
struct Packet<'a, T: Clone + 'static, const N: usize> {
    label: &'a str,
    values: [T; N],
}

#[derive(Facet)]
struct PacketProxy<'a, T: Clone + 'static, const N: usize> {
    label: &'a str,
    values: [T; N],
}

impl<'a, T: Clone + 'static, const N: usize> From<&Packet<'a, T, N>> for PacketProxy<'a, T, N> {
    fn from(packet: &Packet<'a, T, N>) -> Self {
        Self {
            label: packet.label,
            values: packet.values.clone(),
        }
    }
}

impl<'a, T: Clone + 'static, const N: usize> TryFrom<PacketProxy<'a, T, N>> for Packet<'a, T, N> {
    type Error = &'static str;

    fn try_from(proxy: PacketProxy<'a, T, N>) -> Result<Self, Self::Error> {
        if proxy.label.is_empty() {
            return Err("packet label must not be empty");
        }
        Ok(Self {
            label: proxy.label,
            values: proxy.values,
        })
    }
}

#[derive(Facet)]
struct Proxied<'a, T: Clone + 'static, const N: usize> {
    #[facet(proxy = PacketProxy<'a, T, N>)]
    plain: Packet<'a, T, N>,
    #[facet(json::proxy = PacketProxy<'a, T, N>)]
    json: Packet<'a, T, N>,
    #[facet(opaque, proxy = PacketProxy<'a, T, N>)]
    opaque: Packet<'a, T, N>,
    #[facet(opaque, json::proxy = PacketProxy<'a, T, N>)]
    opaque_json: Packet<'a, T, N>,
}

#[derive(Facet)]
#[repr(u8)]
#[allow(dead_code)]
enum ProxiedEnum<'a, T: Clone + 'static, const N: usize> {
    Tuple(#[facet(proxy = PacketProxy<'a, T, N>)] Packet<'a, T, N>),
    Record {
        #[facet(json::proxy = PacketProxy<'a, T, N>)]
        packet: Packet<'a, T, N>,
    },
}

fn fields<'a, T: Facet<'a>>() -> &'static [Field] {
    let Type::User(UserType::Struct(structure)) = T::SHAPE.ty else {
        panic!("expected a struct");
    };
    structure.fields
}

// The callers obtain each proxy from a field whose real type is Packet<T, N>.
// This exercises the generated erased conversions, not the From impl directly.
fn roundtrip<'a, T: Clone + 'static, const N: usize>(
    proxy: &ProxyDef,
    packet: &Packet<'a, T, N>,
) -> Result<Packet<'a, T, N>, String> {
    let mut encoded = MaybeUninit::<PacketProxy<'a, T, N>>::uninit();
    // SAFETY: The source is the initialized field type and the destination has
    // the exact proxy layout. The borrowed label stays live in the caller.
    unsafe {
        (proxy.convert_out)(
            PtrConst::new(packet),
            PtrUninit::from_maybe_uninit(&mut encoded),
        )?;
    }
    let encoded = ManuallyDrop::new(unsafe { encoded.assume_init() });
    let mut decoded = MaybeUninit::<Packet<'a, T, N>>::uninit();
    // SAFETY: convert_in consumes the initialized proxy, so its previous storage
    // must not be dropped again. The destination is the exact field type.
    unsafe {
        (proxy.convert_in)(
            PtrConst::new(&*encoded),
            PtrUninit::from_maybe_uninit(&mut decoded),
        )?;
        Ok(decoded.assume_init())
    }
}

#[test]
fn generic_field_proxies_preserve_scoped_borrows_and_concrete_values() {
    let label = String::from("local request");
    let packet = Packet {
        label: &label,
        values: [String::from("first"), String::from("second")],
    };
    for field in fields::<Proxied<'_, String, 2>>() {
        let proxy = field.effective_proxy(Some("json")).unwrap();
        let decoded = roundtrip(proxy, &packet).unwrap();
        assert_eq!(decoded, packet);
        assert!(std::ptr::eq(decoded.label.as_ptr(), label.as_ptr()));
    }

    // A different type and array length must use its own monomorphized callbacks.
    let numbers = Packet {
        label: &label,
        values: [11_u32, 22, 33],
    };
    for field in fields::<Proxied<'_, u32, 3>>() {
        let decoded = roundtrip(field.effective_proxy(Some("json")).unwrap(), &numbers).unwrap();
        assert_eq!(decoded, numbers);
        assert!(std::ptr::eq(decoded.label.as_ptr(), label.as_ptr()));
    }
}

#[test]
fn generic_enum_field_proxies_execute_both_conversion_paths() {
    let label = String::from("enum request");
    let packet = Packet {
        label: &label,
        values: [String::from("payload")],
    };
    let Type::User(UserType::Enum(enumeration)) = ProxiedEnum::<'_, String, 1>::SHAPE.ty else {
        panic!("expected an enum");
    };
    for variant in enumeration.variants {
        let field = &variant.data.fields[0];
        let decoded = roundtrip(field.effective_proxy(Some("json")).unwrap(), &packet).unwrap();
        assert_eq!(decoded, packet);
        assert!(std::ptr::eq(decoded.label.as_ptr(), label.as_ptr()));
    }
}

#[test]
fn generic_field_proxy_propagates_conversion_rejection() {
    let packet = Packet {
        label: "",
        values: [String::from("owned payload")],
    };
    for field in fields::<Proxied<'_, String, 1>>() {
        let error = roundtrip(field.effective_proxy(Some("json")).unwrap(), &packet).unwrap_err();
        assert_eq!(error, "packet label must not be empty");
    }
}
