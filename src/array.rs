pub trait Array {
    type Item;

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn as_slice(&self) -> &[Self::Item];

    fn as_mut_slice(&mut self) -> &mut [Self::Item];

    fn as_ptr(&self) -> *const Self::Item;

    fn as_mut_ptr(&mut self) -> *mut Self::Item;
}

pub trait ArrayMap<ItemOut>: Array {
    type ArrayOut: Array<Item = ItemOut>;

    fn map<F: FnMut(&Self::Item) -> ItemOut>(&self, f: F) -> Self::ArrayOut;
}

impl<T> Array for [T] {
    type Item = T;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        &*self
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut *self
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }
}

impl<T, U> ArrayMap<U> for [T] {
    type ArrayOut = Vec<U>;

    #[inline]
    fn map<F: FnMut(&Self::Item) -> U>(&self, f: F) -> Self::ArrayOut {
        self.iter().map(f).collect()
    }
}

impl<T> Array for Vec<T>  {
    type Item = T;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn len(&self) -> usize {
        Vec::len(self)
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        Vec::as_slice(self)
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        Vec::as_mut_slice(self)
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }
}

impl<T, U> ArrayMap<U> for Vec<T> {
    type ArrayOut = Vec<U>;

    #[inline]
    fn map<F: FnMut(&Self::Item) -> U>(&self, f: F) -> Self::ArrayOut {
        self.iter().map(f).collect()
    }
}

macro_rules! impl_array {
    ($($N:expr,)+) => {
        $(
            impl<T> Array for [T; $N] {
                type Item = T;

                #[inline]
                fn is_empty(&self) -> bool {
                    self.len() == 0
                }

                #[inline]
                fn len(&self) -> usize {
                    <[T]>::len(self)
                }

                #[inline]
                fn as_slice(&self) -> &[T] {
                    self
                }

                #[inline]
                fn as_mut_slice(&mut self) -> &mut [T] {
                    self
                }

                #[inline]
                fn as_ptr(&self) -> *const T {
                    <[T]>::as_ptr(self)
                }

                #[inline]
                fn as_mut_ptr(&mut self) -> *mut T {
                    <[T]>::as_mut_ptr(self)
                }
            }

            impl<T, U> ArrayMap<U> for [T; $N] {
                type ArrayOut = [U; $N];

                #[inline]
                fn map<F: FnMut(&Self::Item) -> U>(&self, mut f: F) -> Self::ArrayOut {
                    use std::mem;
                    use std::ptr;

                    unsafe {
                        struct Initializer<A: Array> {
                            index: usize,
                            array: mem::ManuallyDrop<A>,
                        }

                        impl<A: Array> Initializer<A> {
                            #[inline]
                            pub fn push(&mut self, item: A::Item) {
                                assert!(self.index < self.array.len());
                                (self.array.as_mut_slice())[self.index] = item;
                                self.index += 1;
                            }

                            #[inline]
                            pub fn finish(self) -> A {
                                assert_eq!(self.index, self.array.len());
                                unsafe {
                                    let mut initializer = mem::ManuallyDrop::new(self);
                                    let array = mem::replace(&mut initializer.array, mem::uninitialized());
                                    mem::ManuallyDrop::into_inner(array)
                                }
                            }
                        }

                        impl<A: Array> Drop for Initializer<A> {
                            #[inline]
                            fn drop(&mut self) {
                                // Drop all values initialized so far.
                                unsafe {
                                    let s = self.array.as_mut_slice();
                                    for i in 0..self.index {
                                        ptr::drop_in_place(&mut s[i])
                                    }
                                }
                            }
                        }

                        let mut init = Initializer::<Self::ArrayOut> {
                            index: 0,
                            array: mem::uninitialized(),
                        };

                        for item in self {
                            init.push(f(item))
                        }

                        Initializer::finish(init)
                    }
                }
            }
        )+
    };
}

impl_array! {
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32,
}
