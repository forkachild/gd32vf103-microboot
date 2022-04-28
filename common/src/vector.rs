pub union Vector {
    function: unsafe extern "C" fn(),
    address: usize,
}

impl Vector {
    #[inline(always)]
    pub const fn from_function(function: unsafe extern "C" fn()) -> Self {
        Self {
            function,
        }
    }

    #[inline(always)]
    pub const fn from_address(address: usize) -> Self {
        Self {
            address,
        }
    }
}