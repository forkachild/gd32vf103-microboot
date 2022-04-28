macro_rules! csr_accessor {
    ($name:ident, $internal_name:ident, $address:literal) => {
        #[doc(hidden)]
        pub struct $internal_name;
        pub type $name = crate::accessor::Accessor<usize, $internal_name>;

        impl $internal_name {
            const fn address() -> usize {
                $address
            }
        }
    };
}

macro_rules! csr_readable {
    ($internal_name:ident) => {
        impl crate::accessor::Readable for $internal_name {
            type Type = usize;

            #[inline(always)]
            fn read() -> Self::Type {
                use core::arch::asm;
                let bits: Self::Type;
                unsafe {
                    asm!(
                        "csrrs {}, {}, zero",
                        out (reg) bits,
                        const Self::address(),
                        options(pure, nomem, nostack)
                    );
                }
                bits
            }
        }
    };
}

macro_rules! csr_writable {
    ($internal_name:ident) => {
        impl crate::accessor::Writable for $internal_name {
            type Type = usize;

            #[inline(always)]
            fn write(value: Self::Type) {
                use core::arch::asm;
                unsafe {
                    asm!(
                        "csrrw zero, {}, {}",
                        const Self::address(),
                        in (reg) value,
                        options(nomem, nostack)
                    );
                }
            }
        }
    };
}
