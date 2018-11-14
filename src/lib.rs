#![feature(maybe_uninit)]

#[cfg(feature = "jpeg-decoder")]
extern crate jpeg_decoder as jpeg;

use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

pub mod sys;

type ContextRef = Rc<RefCell<DropableContext>>;

struct DropableContext(sys::PuzzleContext);

pub struct Context(ContextRef);

pub struct Cvec(ContextRef, sys::PuzzleCvec);

impl Context {
    pub fn new() -> Context {
        let context = unsafe {
            let mut mem = MaybeUninit::<sys::PuzzleContext>::uninitialized();
            sys::puzzle_init_context(mem.as_mut_ptr());
            DropableContext(mem.into_inner())
        };
        Context(Rc::new(RefCell::new(context)))
    }

    pub fn set_max_size(&mut self, width: u32, height: u32) {
        assert!(unsafe { sys::puzzle_set_max_width(&mut self.0.borrow_mut().0, width) } == 0);
        assert!(unsafe { sys::puzzle_set_max_height(&mut self.0.borrow_mut().0, height) } == 0);
    }

    #[cfg(feature = "gd")]
    pub fn cvec_from_file(&mut self, path: &str) -> Result<Cvec, ()> {
        let mut cvec = self.init_cvec();
        let path_c = std::ffi::CString::new(path).unwrap();
        let ret = unsafe {
            sys::puzzle_fill_cvec_from_file(
                &mut cvec.0.borrow_mut().0,
                &mut cvec.1,
                path_c.as_ptr(),
            )
        };
        if ret == 0 {
            Ok(cvec)
        } else {
            Err(())
        }
    }

    #[cfg(feature = "jpeg-decoder")]
    pub fn cvec_from_jpeg_file(&mut self, path: &str) -> Result<Cvec, ()> {
        let file = std::fs::File::open(path).map_err(|_| ())?;
        self.cvec_from_jpeg_scaled(std::io::BufReader::new(file), 1)
    }

    #[cfg(feature = "jpeg-decoder")]
    pub fn cvec_from_jpeg_scaled<R: std::io::Read>(
        &mut self,
        image: R,
        scale: usize,
    ) -> Result<Cvec, ()> {
        let mut decoder = jpeg::Decoder::new(image);
        let pixels = decoder.decode().map_err(|_| ())?;
        let metadata = decoder.info().ok_or(())?;
        if metadata.pixel_format != jpeg::PixelFormat::RGB24 {
            return Err(());
        }

        let width = metadata.width as usize / scale;
        let height = metadata.height as usize / scale;
        if width == 0 || height == 0 {
            return Err(());
        }

        let mut data = std::vec::Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                let offset = 3
                    * (metadata.width as usize * (height - y - 1) * scale
                        + (width - x - 1) * scale);
                data.push(rgb24_to_luminance(&pixels[offset..offset + 3]));
            }
        }

        let mut cvec = self.init_cvec();
        let ret = unsafe {
            sys::puzzle_fill_cvec_from_view(
                &mut cvec.0.borrow_mut().0,
                &mut cvec.1,
                width as u32,
                height as u32,
                data.as_ptr(),
            )
        };
        if ret == 0 {
            Ok(cvec)
        } else {
            Err(())
        }
    }

    #[allow(dead_code)]
    fn init_cvec(&mut self) -> Cvec {
        let cvec_sys = unsafe {
            let mut mem = MaybeUninit::<sys::PuzzleCvec>::uninitialized();
            sys::puzzle_init_cvec(&mut self.0.borrow_mut().0, mem.as_mut_ptr());
            mem.into_inner()
        };
        Cvec(Rc::clone(&self.0), cvec_sys)
    }
}

impl Drop for DropableContext {
    fn drop(&mut self) {
        unsafe { sys::puzzle_free_context(&mut self.0) }
    }
}

impl Cvec {
    pub fn distance(&self, other: &Cvec) -> f64 {
        assert!(Rc::ptr_eq(&self.0, &other.0));
        unsafe {
            sys::puzzle_vector_normalized_distance(&mut self.0.borrow_mut().0, &self.1, &other.1, 1)
        }
    }
}

impl Drop for Cvec {
    fn drop(&mut self) {
        unsafe { sys::puzzle_free_cvec(&mut self.0.borrow_mut().0, &mut self.1) }
    }
}

#[allow(dead_code)]
fn rgb24_to_luminance(pixel: &[u8]) -> u8 {
    ((pixel[0] as u16 * 77 + pixel[1] as u16 * 151 + pixel[2] as u16 * 28 + 128) / 256) as u8
}
