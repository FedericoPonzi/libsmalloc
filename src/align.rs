/// Align `addr` downwards to the nearest multiple of `align`.
/// Panics if `align` is not a power of 2.
#[allow(unused)]
pub fn align_down(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align_up: alignment must be a power of 2")
    }
    addr.saturating_sub(addr % align)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// ## Safety
///
/// Panics if `align` is not a power of 2 (it's enforced in the Layout struct).
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align_up: alignment must be a power of 2")
    }
    let delta = align - addr % align;
    // If addr is already aligned to addr, `delta` will be set to align.
    // In that case, to zero-fy delta % align is used:
    let actual_delta = delta % align;
    addr.checked_add(actual_delta)
        .expect("Checked add in align up overflowed.")
}

#[cfg(test)]
mod tests {
    use super::{align_down, align_up};

    #[test]
    fn test_align_down() {
        assert_eq!(align_down(0, 2), 0);
        assert_eq!(align_down(0, 8), 0);
        assert_eq!(align_down(0, 1 << 5), 0);

        assert_eq!(align_down(1 << 10, 1 << 10), 1 << 10);
        assert_eq!(align_down(1 << 20, 1 << 10), 1 << 20);
        assert_eq!(align_down(1 << 23, 1 << 4), 1 << 23);

        assert_eq!(align_down(1, 1 << 4), 0);
        assert_eq!(align_down(10, 1 << 4), 0);

        assert_eq!(align_down(0xFFFF, 1 << 2), 0xFFFC);
        assert_eq!(align_down(0xFFFF, 1 << 3), 0xFFF8);
        assert_eq!(align_down(0xFFFF, 1 << 4), 0xFFF0);
        assert_eq!(align_down(0xFFFF, 1 << 5), 0xFFE0);
        assert_eq!(align_down(0xAFFFF, 1 << 8), 0xAFF00);
        assert_eq!(align_down(0xAFFFF, 1 << 12), 0xAF000);
        assert_eq!(align_down(0xAFFFF, 1 << 16), 0xA0000);
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 2), 0);
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(0, 1 << 5), 0);

        assert_eq!(align_up(1 << 10, 1 << 10), 1 << 10);
        assert_eq!(align_up(1 << 20, 1 << 10), 1 << 20);
        assert_eq!(align_up(1 << 23, 1 << 4), 1 << 23);

        assert_eq!(align_up(1, 1 << 4), 1 << 4);
        assert_eq!(align_up(10, 1 << 4), 1 << 4);

        assert_eq!(align_up(0xFFFF, 1 << 2), 0x10000);
        assert_eq!(align_up(0xFFFF, 1 << 3), 0x10000);
        assert_eq!(align_up(0xFFFF, 1 << 4), 0x10000);
        assert_eq!(align_up(0xAFFFF, 1 << 12), 0xB0000);

        assert_eq!(align_up(0xABCDAB, 1 << 2), 0xABCDAC);
        assert_eq!(align_up(0xABCDAB, 1 << 4), 0xABCDB0);
        assert_eq!(align_up(0xABCDAB, 1 << 8), 0xABCE00);
        assert_eq!(align_up(0xABCDAB, 1 << 12), 0xABD000);
        assert_eq!(align_up(0xABCDAB, 1 << 16), 0xAC0000);
    }
}
