use core::marker::PhantomData;

pub trait Readable {
    type Type;
    fn read() -> Self::Type;
}

pub trait Writable {
    type Type;
    fn write(value: Self::Type);
}

pub trait Register {
    type Type;
    fn address() -> usize;
    fn reset_value() -> Self::Type;
}

pub trait AllowRead: Register {}

pub trait AllowWrite: Register {}

impl<T> Readable for T
    where
        T: Register + AllowRead,
        T::Type: 'static + Sized + Copy,
{
    type Type = T::Type;

    #[inline(always)]
    fn read() -> Self::Type {
        unsafe { core::ptr::read_volatile(T::address() as *const T::Type) }
    }
}

impl<T> Writable for T
    where
        T: Register + AllowWrite,
        T::Type: 'static + Sized + Copy,
{
    type Type = T::Type;

    #[inline(always)]
    fn write(value: Self::Type) {
        unsafe {
            core::ptr::write_volatile(T::address() as *mut T::Type, value);
        }
    }
}

#[repr(transparent)]
pub struct Accessor<T, U> {
    bits: T,
    _type: PhantomData<U>,
}

impl<T, U> Accessor<T, U>
    where
        T: 'static + Sized + Copy,
        U: Readable<Type=T>,
{
    #[inline(always)]
    pub fn read() -> Self {
        Self {
            bits: U::read(),
            _type: PhantomData,
        }
    }

    #[inline(always)]
    pub fn refresh(&mut self) {
        self.bits = U::read();
    }
}

impl<T, U> Accessor<T, U>
    where
        T: 'static + Sized + Copy,
        U: Writable<Type=T>,
{
    #[inline(always)]
    pub fn write_immediate(value: T) {
        U::write(value);
    }

    #[inline(always)]
    pub fn write(&self) {
        U::write(self.bits);
    }
}

impl<T, U> Accessor<T, U>
    where
        T: 'static + Sized + Copy,
        U: AllowWrite<Type=T>,
{
    #[inline(always)]
    pub fn reset() {
        U::write(U::reset_value());
    }
}
