//TODO:
// implement txt file io - saving + handle when no .txt exists to grab
// implement saving functionality
// implement file creation
// start display
// implement GUI for above - see if there is an OS way to handle saving dialogue

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};

use wgpu_glyph::{
    ab_glyph,
    GlyphBrushBuilder,
    GlyphBrush,
    Section,
    Text
};

use std::env;
use std::path::Path;
use std::path::PathBuf;

//modules
mod iomod;
mod doc_handler;
mod wgpu_handler;

use crate::iomod::*;
use crate::doc_handler::*;
use crate::wgpu_handler::*;

pub async fn run() {

    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut wgpu_handler = WgpuHandler::new(window).await;

    let wkdir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("{:?}", e)
    };

    let path = wkdir.join("text.txt"); //later prompt user to provide path 

    let mut doc_handler = DocHandler::load(&path);



    event_loop.run(move |event, _, control_flow| {

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        // control_flow.set_poll();

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        control_flow.set_wait();

        // Match events detected by the Window obj and perform the corresponding action using the
        // ControFlow obj
        // println!("{:?}", event);

        match event { //match block 1

            Event::WindowEvent {
                ref event, //for notes on this syntax see The Rust Programming Language, 18.3 Pattern Syntax, Ignoring Remaining Parts of a Value with ..
                window_id,  //https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

            // Additional check to make sure that we are handling the correct window
            } if window_id == wgpu_handler.window().id() => { //start WindowEvent block

                // Filter for specific events we want to handle in render loop
                if wgpu_handler.input(event) == false { //start if statment block

                    match event { //match block 2

                        // Close the window when CloseRequested event is detected
                        WindowEvent::CloseRequested => {
                            println!("The close button was pressed, stopping.");
                            control_flow.set_exit();
                        },

                        // Close the window when the Escape key is pressed
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => {
                            println!("The escape button was pressed, stopping.");
                            control_flow.set_exit();
                        },

                        //Compressed version of above for conscision
                        //Key1
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key1), .. },  ..
                        } => { doc_handler.update('1') },
                        //Key2
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key2), .. },  ..
                        } => { doc_handler.update('2') },
                        //Key3
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key3), .. },  ..
                        } => { doc_handler.update('3') },
                        //Key4
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key4), .. },  ..
                        } => { doc_handler.update('4') },
                        //Key5
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key5), .. },  ..
                        } => { doc_handler.update('5') },
                        //Key6
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key6), .. },  ..
                        } => { doc_handler.update('6') },
                        //Key7
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key7), .. },  ..
                        } => { doc_handler.update('7') },
                        //Key8
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key8), .. },  ..
                        } => { doc_handler.update('8') },
                        //Key9
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key9), .. },  ..
                        } => { doc_handler.update('9') },
                        //Key0
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Key0), .. },  ..
                        } => { doc_handler.update('0') },
                        //A
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::A), .. },  ..
                        } => { doc_handler.update('a') },
                        //B
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::B), .. },  ..
                        } => { doc_handler.update('b') },
                        //C
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::C), .. },  ..
                        } => { doc_handler.update('c') },
                        //D
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::D), .. },  ..
                        } => { doc_handler.update('d') },
                        //E
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::E), .. },  ..
                        } => { doc_handler.update('e') },
                        //F
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::F), .. },  ..
                        } => { doc_handler.update('f') },
                        //G
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::G), .. },  ..
                        } => { doc_handler.update('g') },
                        //H
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::H), .. },  ..
                        } => { doc_handler.update('h') },
                        //I
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::I), .. },  ..
                        } => { doc_handler.update('i') },
                        //J
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::J), .. },  ..
                        } => { doc_handler.update('j') },
                        //K
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::K), .. },  ..
                        } => { doc_handler.update('k') },
                        //L
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::L), .. },  ..
                        } => { doc_handler.update('l') },
                        //M
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::M), .. },  ..
                        } => { doc_handler.update('m') },
                        //N
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::N), .. },  ..
                        } => { doc_handler.update('n') },
                        //O
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::O), .. },  ..
                        } => { doc_handler.update('o') },
                        //P
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::P), .. },  ..
                        } => { doc_handler.update('p') },
                        //Q
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Q), .. },  ..
                        } => { doc_handler.update('q') },
                        //R
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::R), .. },  ..
                        } => { doc_handler.update('r') },
                        //S
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::S), .. },  ..
                        } => { doc_handler.update('s') },
                        //T
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::T), .. },  ..
                        } => { doc_handler.update('t') },
                        //U
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::U), .. },  ..
                        } => { doc_handler.update('u') },
                        //V
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::V), .. },  ..
                        } => { doc_handler.update('v') },
                        //W
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::W), .. },  ..
                        } => { doc_handler.update('w') },
                        //X
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::X), .. },  ..
                        } => { doc_handler.update('x') },
                        //Y
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Y), .. },  ..
                        } => { doc_handler.update('y') },
                        //Z
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Z), .. },  ..
                        } => { doc_handler.update('z') },


                        //Punctuation
                        //Colon
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Colon), .. },  ..
                        } => { doc_handler.update(':') },
                        //Comma
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Comma), .. },  ..
                        } => { doc_handler.update(',') },
                        //Period
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Period), .. },  ..
                        } => { doc_handler.update('.') },
                        //Semicolon
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Semicolon), .. },  ..
                        } => { doc_handler.update(';') },
                        //Space
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Space), .. },  ..
                        } => { doc_handler.update(' ') },

                        //Cursor Movement
                        //Up
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Up), .. },  ..
                        } => { doc_handler.update_cursor("up") },
                        //Down
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Down), .. },  ..
                        } => { doc_handler.update_cursor("down") },
                        //Left
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Left), .. },  ..
                        } => { doc_handler.update_cursor("left") },
                        //Right
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Right), .. },  ..
                        } => { doc_handler.update_cursor("right") },

                        //Misc Inputs
                        //Return
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Return), .. },  ..
                        } => { doc_handler.newline() },

                        //Backspace
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Back), .. },  ..
                        } => { doc_handler.backspace() },


                        //GL Bindings
                        // Resize the surface when window is resized
                        WindowEvent::Resized(physical_size) => {
                            wgpu_handler.resize(*physical_size);
                        },

                        // Resize the surface when scale factor is changed
                        WindowEvent::ScaleFactorChanged {new_inner_size, .. } => {
                            wgpu_handler.resize(**new_inner_size);
                        },

                        //For all other WindowEvents do nothing
                        _ => {}
                    }//end match block 2
                }//end if statement block
            }//end WindowEvent block

            //
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                // RedrawRequested will only evaluate once unless we request it here

                wgpu_handler.window().request_redraw();
            },

            //
            Event::RedrawRequested(window_id) if window_id == wgpu_handler.window().id() => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                wgpu_handler.update(); //currently does nothing

                // For notes on error handling with match blocks see The Rust Programming Language > 9.2 Recoverable Errors with Result > Mathing on Different Errors
                // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
                match wgpu_handler.render(&doc_handler.text) {
                    Ok(_) => {},

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => wgpu_handler.resize(wgpu_handler.size),

                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),

                    //Err(e) => eprintln!("{:?}", e), //catch all for other errors to print to std output
                }


            },

            // For all other inputs, do nothing
            _ => {}

        }//end match block 1

    });

}//end main()
