#![allow(dead_code)]
#![feature(conservative_impl_trait)]

//extern crate linjs;

//use linjs::JSCompartment;
//use linjs::JSContext;
//use linjs::JSManaged;
//use linjs::JSManageable;

use std::marker::PhantomData;

struct JSManaged<'a,C,T> { marker: PhantomData<(&'a(),C,T)> }
struct JSContext<C> { marker: PhantomData<C> }
struct JSAllocator<C> { marker: PhantomData<C> }
struct JSInitializer<C> { marker: PhantomData<C> }
struct JSRoots<C> { marker: PhantomData<C> }

unsafe trait JSManageable<'a> {
    type Aged:;
}

trait JSAllocatorMethods<C> {
    fn manage_cyclic_rooted<'a, T, F>(&'a mut self, f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>, &'a mut JSAllocator<C>) -> T,
        C: 'a,
        T: JSManageable<'a>;

    fn manage_cyclic<'a, T, F>(&'a mut self, f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>) -> T,
        C: 'a,
        T: JSManageable<'a>;

    fn roots(&mut self) -> JSRoots<C>;

    fn manage<'a, T>(&'a mut self, native: T) -> JSManaged<'a, C, T::Aged> where
        C: 'a,
        T: JSManageable<'a>,
    {
        self.manage_cyclic(|_| native)
    }
}

impl<C> JSAllocatorMethods<C> for JSContext<C> {
    fn manage_cyclic_rooted<'a, T, F>(&'a mut self, f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>, &'a mut JSAllocator<C>) -> T,
        C: 'a,
        T: JSManageable<'a>,
    {
        self.allocator_mut().manage_cyclic_rooted(f)
    }

    fn manage_cyclic<'a, T, F>(&'a mut self, f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>) -> T,
        C: 'a,
        T: JSManageable<'a>,
    {
        self.allocator_mut().manage_cyclic(f)
    }

    fn roots(&mut self) -> JSRoots<C> {
        unimplemented!()
    }
}

impl<C> JSInitializer<C> {
    fn new_global_object<'a, T, F>(self, f: F) -> (JSManaged<'a, C, T::Aged>, JSContext<C>) where
        F: FnOnce(JSManaged<'a, C, T::Aged>, &'a mut JSAllocator<C>) -> T,
        C: 'a,
        T: JSManageable<'a>,
    {
        unimplemented!()
    }
}

impl<C> JSAllocatorMethods<C> for JSAllocator<C> {
    fn manage_cyclic_rooted<'a, T, F>(&'a mut self, _f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>, &'a mut JSAllocator<C>) -> T,
        C: 'a,
        T: JSManageable<'a>,
    {
        unimplemented!()
    }

    fn manage_cyclic<'a, T, F>(&'a mut self, _f: F) -> JSManaged<'a, C, T::Aged> where
        F: FnOnce(JSManaged<'a, C, T::Aged>) -> T,
        C: 'a,
        T: JSManageable<'a>,
    {
        unimplemented!()
    }

    fn roots(&mut self) -> JSRoots<C> {
        unimplemented!()
    }
}

impl<C> JSContext<C> {
    fn allocator_mut(&mut self) -> &mut JSAllocator<C> {
        unimplemented!()
    }
    fn execute(&mut self, command: &str) {
        unimplemented!()
    }
}

impl<C> JSInitializer<C> {
    fn allocator_mut(&mut self) -> &mut JSAllocator<C> {
        unimplemented!()
    }
    fn set_global<T>(&mut self, global: JSManaged<C, T>) -> &mut JSContext<C> {
        unimplemented!()
    }
    fn roots(&mut self) -> JSRoots<C> {
        unimplemented!()
    }
}

impl<C> JSRoots<C> {
    fn manage<'a, T>(&'a self, native: T) -> JSManaged<'a, C, T::Aged> where
        T: JSManageable<'a>
    {
        unimplemented!()
    }
}

impl<'a, C, T> JSManaged<'a, C, T> {
    fn get(self, _cx: &'a JSContext<C>) -> &'a T {
        unimplemented!()
    }        

    fn get_mut(self, _cx: &'a mut JSContext<C>) -> &'a mut T {
        unimplemented!()
    }
}

impl<'a, C, T> Clone for JSManaged<'a, C, T> {
    fn clone(&self) -> JSManaged<'a, C, T> {
        unimplemented!()
    }
}
impl<'a, C, T> Copy for JSManaged<'a, C, T> {}

// -------------------------------------------------------------------

type Window<'a, C> = JSManaged<'a, C, NativeWindow<'a, C>>;

struct NativeWindow<'a, C> {
    console: Console<'a, C>,
    document: Element<'a, C>,
}

unsafe impl<'a, 'b, C> JSManageable<'b> for NativeWindow<'a, C> {
   type Aged = NativeWindow<'b, C>;
}

// -------------------------------------------------------------------

type Console<'a, C> = JSManaged<'a, C, NativeConsole<'a, C>>;

struct NativeConsole<'a, C> {
    window: Window<'a, C>,
}

impl<'a, C> Console<'a, C> {
    fn log(msg: String) {
        println!("LOG: {}.", msg);
    }
}

unsafe impl<'a, 'b, C> JSManageable<'b> for NativeConsole<'a, C> {
   type Aged = NativeConsole<'b, C>;
}

// -------------------------------------------------------------------

type Element<'a, C> = JSManaged<'a, C, NativeElement<'a, C>>;

struct NativeElement<'a, C> {
    window: Window<'a, C>,
    parent: Option<Element<'a, C>>,
    children: Vec<Element<'a, C>>,
}

unsafe impl<'a, 'b, C> JSManageable<'b> for NativeElement<'a, C> {
   type Aged = NativeElement<'b, C>;
}

impl<'a, C> Element<'a, C> {
    fn add_child(self, cx: &'a mut JSContext<C>, other: Element<'a, C>) {
        self.get_mut(cx).children.push(other);
        other.get_mut(cx).parent = Some(self);
    }
    fn children(self, cx: &'a JSContext<C>) -> &'a [Element<'a, C>] {
        &*self.get(cx).children
    }
}

// -------------------------------------------------------------------

trait JSRunnable {
    fn run<C>(self, cx: JSInitializer<C>);

    fn start(&mut self) {
        unimplemented!()
    }
}

struct JSRuntime;

const JS_RUNTIME: JSRuntime = JSRuntime;

impl JSRuntime {
    fn run<R>(&self, runnable: R) where
        R: JSRunnable
    {
        unimplemented!()
    }
}

// -------------------------------------------------------------------

struct Main;

impl JSRunnable for Main {
    fn run<C>(self, mut cx: JSInitializer<C>) {
        let roots = cx.roots();
        let (window, mut cx) = cx.new_global_object(|window, cx| {
            NativeWindow {
                console: roots.manage(NativeConsole {
                    window: window,
                }),
                document: cx.manage(NativeElement {
                    window: window,
                    parent: None,
                    children: Vec::new(),
                }),
            }
        });
        cx.execute("console.log('hi');");
    }
}

fn main() {
    JS_RUNTIME.run(Main);
}
