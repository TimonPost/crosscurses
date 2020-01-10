# Crossplatform curses library.

[![Latest Version][crate-badge]][crate-link] 
[![docs][docs-badge]][docs-link]
![Lines of Code][loc-badge]
![MIT][license-badge]

### Disclaimer
This is a fork from [pancurses](https://github.com/ihalila/pancurses) used by [terminal](https://github.com/crossterm-rs/terminal). 
This fork was created because `terminal` needed a number of functions that were not merged or released. 
Pancurses seems to be inactive. However, I do want to release `terminal`.
That is why I created this fork library. I **strongly** advise you NOT TO USE it. But use the original pancurses library. 
If pancurses decides to continue its activity it is very likely that this fork will no longer be maintained.

# Crosscurses

crosscurses is a curses library for Rust that supports both Linux and Windows
by abstracting away the backend that it uses
([ncurses-rs](https://github.com/jeaye/ncurses-rs) and
[pdcurses-sys](https://github.com/ihalila/pdcurses-sys) respectively).

The aim is to provide a more Rustic interface over the usual curses functions
for ease of use while remaining close enough to curses to make porting easy.

## Requirements

#### Linux
ncurses-rs links with the native ncurses library so that needs to be installed
so that the linker can find it.

Check [ncurses-rs](https://github.com/jeaye/ncurses-rs) for more details.

#### Windows
pdcurses-sys compiles the native PDCurses library as part of the build process,
so you need to have a compatible C compiler available that matches the ABI of
the version of Rust you're using (so either gcc for the GNU ABI or cl for MSVC)

Check [pdcurses-sys](https://github.com/ihalila/pdcurses-sys) for more details.

## Usage
Cargo.toml
```toml
[dependencies]
crosscurses = "0.1"
```

main.rs
```rust
use crosscurses::{initscr, endwin};

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  window.getch();
  endwin();
}
```

## Pattern matching with getch()

```rust
use crosscurses::{initscr, endwin, Input, noecho};

fn main() {
  let window = initscr();
  window.printw("Type things, press delete to quit\n");
  window.refresh();
  window.keypad(true);
  noecho();
  loop {
      match window.getch() {
          Some(Input::Character(c)) => { window.addch(c); },
          Some(Input::KeyDC) => break,
          Some(input) => { window.addstr(&format!("{:?}", input)); },
          None => ()
      }
  }
  endwin();
}
```

## Handling mouse input

To receive mouse events you need to both enable keypad mode and set a mouse mask that corresponds
to the events you are interested in. Mouse events are received in the same way as keyboard events,
ie. by calling getch().

```rust
extern crate crosscurses;

use crosscurses::{ALL_MOUSE_EVENTS, endwin, getmouse, initscr, mousemask, Input};

fn main() {
    let window = initscr();

    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events

    window.printw("Click in the terminal, press q to exit\n");
    window.refresh();

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    window.mvprintw(1, 0,
                                    &format!("Mouse at {},{}", mouse_event.x, mouse_event.y),
                    );
                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}
```

You can also receive events for the mouse simply moving (as long as the terminal you're running on
supports it) by also specifying the REPORT_MOUSE_POSITION flag:
```rust
mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, std::ptr::null_mut());
```

## Terminal resizing

Whenever the terminal is resized by the user a Input::KeyResize event is raised. You should handle
this by calling ```resize_term(0, 0)``` to have curses adjust it's internal structures to match the
new size.

## PDCurses (Windows) details

pdcurses-sys supports two flavors of PDCurses, win32a and win32. win32a is the GDI mode while win32
runs in the Windows console. win32a has better support for colors and text effects.

By default the win32a flavor is used, but you can specify which one you want to use by using Cargo
flags. Simply specify the feature in Cargo.toml like so:

```rust
[dependencies.crosscurses]
version = "0.16"
features = ["win32a"]
```
or

```rust
[dependencies.crosscurses]
version = "0.16"
features = ["win32"]
```

### (Font, Paste) menu

PDCurses win32a has a menu that allows you to change the font and paste text into the window.
crosscurses disables the window by default, though the user can still right-click the title bar to 
access it. If you want to retain the PDCurses default behaviour of having the menu there set the 
feature ```"show_menu"```.

### Resizing

On win32a the default is to allow the user to freely resize the window. If you wish to disable
resizing set the feature ```"disable_resize"```

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)

[crate-badge]: https://img.shields.io/crates/v/crosscurses.svg
[crate-link]: https://crates.io/crates/crosscurses

[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[docs-badge]: https://docs.rs/crosscurses/badge.svg
[docs-link]: https://docs.rs/crosscurses/

[loc-badge]: https://tokei.rs/b1/github/TimonPost/crosscurses?category=code