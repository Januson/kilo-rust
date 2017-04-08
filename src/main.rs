extern crate libc;

use std::io::Read;
use std::char::from_u32;
use libc::*;

fn main() {
    let mut t = enable_raw_mode();
    let mut input = std::io::stdin()
        .bytes();

    loop {
        let i = input.next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u32)
            .unwrap();
        println!("{:?}", i);
        if i == 27 {
            break;
        }
        println!("You wrote letter: {}", from_u32(i).unwrap());
    }
    t.echo_on();
}

fn enable_raw_mode() -> Termios {
    let mut t = Termios::new();
    t.echo_off();
    t
}

#[repr(C)]
struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_cc: [c_uchar; 32],
    c_line: c_uchar,
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}

impl Termios {
    pub fn new() -> Termios {
        let mut t = Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_cc: [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
                0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
            c_line: 0,
            c_ispeed: 0,
            c_ospeed: 0,
        };
        unsafe {
            if tcgetattr(STDIN_FILENO, &mut t) == -1 {
                panic!("Could not call tcgetattr");
            }
        }
        t
    }

    /// Turn echo off
    pub fn echo_off(&mut self) -> () {
        self.c_lflag &= !ECHO;
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }

    /// Turn echo on
    pub fn echo_on(&mut self) -> () {
        self.c_lflag |= ECHO;
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }
}

#[link(name = "c")]
extern {
    fn tcgetattr(fd: c_int, termios: &mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios: &Termios) -> c_int;
}