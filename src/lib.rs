//TODO:
// implement newline method for DocHandler
// implement txt file io - saving + handle when no .txt exists to grab
// implement saving functionality
// implement file creation
// implement GUI for above

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

//modules
mod iomod;
use iomod::*;

struct DocHandler {
    text: Vec<String>,     //text contents of doc
    line_data: Vec<usize>, //retains the len of each line in text
    num_lines: usize, //number of lines
    pointer_x: usize, //increases left going to right
    pointer_y: usize  //increases top going to bottom
}

impl DocHandler {
    //for initializing a completely new doc
    fn new() -> Self {
        let mut text : Vec<String> = vec![String::new()];
        let mut line_data : Vec<usize> = vec![0_usize];
        let mut num_lines : usize = 0;
        let mut pointer_x : usize = 0;
        let mut pointer_y : usize = 0;

        Self {
            text,
            line_data,
            num_lines,
            pointer_x,
            pointer_y
        }
    }//end new def

    //for loading in data from an existing .txt
    fn load(path : &PathBuf) -> Self {
        //read in text
        let mut text : Vec<String> = match iomod::load_file_txt(&path) {
            Ok(text) => text,
            Err(e) => panic!("Failed to load {:?} due to {:?}", path, e)
        };

        //get len of each line
        let mut line_data : Vec<usize> = Vec::new();
        for line in &text {
            line_data.push( line.len() )
        };

        //get number of lines
        let mut num_lines = line_data.len();

        //default cursor to last position in doc
        let mut pointer_x = line_data[num_lines];
        let mut pointer_y = num_lines;

        Self {
            text,
            line_data,
            num_lines,
            pointer_x,
            pointer_y
        }

    }//end load def

    fn delete(&mut self) {
        if self.pointer_x > 0 && self.pointer_y >= 0 {
            //move pointer a space left
            self.pointer_x -= 1;
            //truncate line to decremented size
            self.text[self.pointer_y].truncate(self.pointer_x);
            //update line data to reflect the line y has been shortened to len pointer_x
            self.line_data[self.pointer_y] = self.pointer_x;

        } else if self.pointer_x == 0 && self.pointer_y > 0 {
            //x is at furthest left point
            //remove the line the pointer is currently on and append the rest of the line
            //to the end of the line above should there be any text there
            self.text[self.pointer_y - 1].push_str( self.text.remove(self.pointer_y).as_str() );
            //update line data to reflect that line was deleted
            self.line_data.remove(self.pointer_y);
            //move pointer up a line
            self.pointer_y -= 1;
            //update num_lines to reflect that a line was deleted
            self.num_lines -= 1;
            //set x position to the end of the above line (before the append)
            self.pointer_x = self.line_data[self.pointer_y];
            //update line_data with new len of the above line
            let self.line_data[self.pointer_y] = self.text[self.pointer_y].len();

        };
    }//end delete

    fn update(&mut self, glyph : char) {
        self.text[self.pointer_y].insert(self.pointer_x, glyph);
        self.pointer_x += 1;
    }

    fn newline(&mut self) {
        self.pointer_y += 1;
        self.text.insert(self.poiner_y, String::from("\n"));
        self.text.insert(self.pointer_y, 1_usize);
        let self.pointer_x = 0;
        self.num_lines += 1;
    }
}

struct WgpuHandler {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration, 
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: GlyphBrush<()>
}

impl WgpuHandler {
    // Constructor method
    async fn new(window: Window) -> Self {
        // Fetching the size of the window from the winit Window obj
        let size = window.inner_size();

        // The instance is the primary handler for wgpu and our GPU
        // the primary job of the Instance is to create Adapter(s) and Surface(s)
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // # Safety
        // The surface needs to live as long as the window that created it.
        // WgpuHandler owns the window so this should be safe
        let surface = unsafe { instance.create_surface(&window) };

        // Use the instance to obtain an adapter obj with the desired parameters
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        // Use the adapter to obtain the device and queue objs
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // Debug label
                label: None,

                // Features that the device should support. 
                // If any feature is not supported by the adapter, 
                // creating a device will panic.

                // no requested features
                features: wgpu::Features::empty(),

                // Limits that the device should support. 
                // If any limit is “better” than the limit exposed by the adapter, 
                // creating a device will panic.

                // limits set to default - max compatability
                limits: wgpu::Limits::default(),   
            },

            // Some(&std::path::Path::new("trace")), // Trace path
            None

            ).await.unwrap();

        //
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,      // How SurfaceTextures will be used
            format: surface.get_supported_formats(&adapter)[0], // How SurfaceTextures will be stored on gpu
            width: size.width,                                  // Width of SurfaceTexture
            height: size.height,                                // Height of SurfaceTexture
            present_mode: wgpu::PresentMode::Fifo,              // How to sync surface with display
            alpha_mode: wgpu::CompositeAlphaMode::Auto,         // 
        };

        //
        surface.configure(&device, &config);

        // Create staging belt
        let mut staging_belt = wgpu::util::StagingBelt::new(1024);

        // Fetch font and store as FontArc obj for use initializing GlyphBrush obj
        // Font sourced from Google Fonts, font name = Bungee Shade, designer = David Jonathan Ross
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("RobotoMono-Regular.ttf")).unwrap(); // must unwrap since returns as Result otherwise

        // Initialize our GlyphBrush obj for use later
        let mut glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, config.format); // want to retreive the value found in config's format field

        // 

        // Return value
        // Self is a handler for whatever obj that the impl block is associated with
        // i.e. here Self {...} == WgpuHandler {...} 
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            staging_belt,
            glyph_brush
        }

    } //end new() def

    // Function to fetch window associated with wgpu instance
    pub fn window(&self) -> &Window {
        &self.window
    } //end window() def

    // Function to handle window scaling and resizing
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;                               // Update size field
            self.config.width = new_size.width;                 // Need to reconfigure surface so
            self.config.height = new_size.height;               // we update config w & h
            self.surface.configure(&self.device, &self.config); // Reconfigure surface
        }
    } //end resize() def

    // Function to handle specific window events within the main render loop
    fn input(&mut self, event: &WindowEvent) -> bool {
        //input() returns a bool to indicate whether an event has been fully processed. 
        //If the method returns true, the main loop won't process the event any further

        false
    } //end input() def

    //
    fn update(&mut self) {
        //not in use
    } //end update() def

    //
    fn render(&mut self, text: &mut Vec<String>) -> Result<(), wgpu::SurfaceError> {

        // get_current_surface_texture waits for a new SurfaceTexture obj to be
        // supplied by the surface. We will render to this SurfaceTexture obj.
        // For now we store it in output.
        let output = self.surface.get_current_texture()?; // for notes on ? syntax see 
                                                          // https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about

        // [The view variable def line] creates a TextureView with default settings. 
        // We need to do this because we want to control how the render code 
        // interacts with the texture.                                                 
        let view = output.texture.create_view( &wgpu::TextureViewDescriptor::default() );

        // We also need to create a CommandEncoder to create the actual commands to send to the gpu.
        // Most modern graphics frameworks expect commands to be stored in a command buffer 
        // before being sent to the gpu. The encoder builds a command buffer that we can then 
        // send to the gpu.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"), //debug label, presumably
        });

        //[To clear the screen] We need to use the encoder to create a RenderPass.
        // The RenderPass has all the methods for the actual drawing.
        { // not a typo
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { //start RenderPassDescriptor args
            
            label: Some("Render Pass"),

            color_attachments: &[Some(wgpu::RenderPassColorAttachment { //start RenderPassColorAttachment args
                
                // A RenderPassDescriptor only has three fields: label, color_attachments and depth_stencil_attachment.
                // The color_attachments describe where we are going to draw our color to.
                // We use the TextureView we created earlier to make sure that we render to the screen.

                // the view field which informs wgpu what texture to save the colors to.
                // In this case we specify the view that we created using surface.get_current_texture().
                // This means that any colors we draw to this attachment will get drawn to the screen.
                view: &view,

                // The resolve_target is the texture that will receive the resolved output.
                // This will be the same as view unless multisampling is enabled.
                // We don't need to specify this, so we leave it as None.
                resolve_target: None,

                // The ops field takes a wpgu::Operations object.
                // This tells wgpu what to do with the colors on the screen (specified by view).
                // The load field tells wgpu how to handle colors stored from the previous frame.
                // Currently, we are clearing the screen with white.
                // The store field tells wgpu whether we want to store the rendered results
                // to the Texture behind our TextureView (in this case it's the SurfaceTexture).
                // We use true as we do want to store our render results.
                ops: wgpu::Operations { //start Operations args

                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }), //close Clear()

                    store: true, 

                }, //end Operations args

            } //end RenderPassColorAttachement args
            ) //end Some block
            ], //end bracket on ref

            depth_stencil_attachment: None,

        } //end RenderPassDescriptor args
        ); //end _render_pass def 
        } //end not a typo

        let mut render_text = String::new();
        for line in text {
            render_text.push_str(&line);
        };

        // Prepare and configure the text you want to display
        // we are adding this Section obj to the GlyphBrush queue
        self.glyph_brush.queue(Section {

            screen_position: (40.0, 40.0),

            bounds: (self.size.width as f32 , self.size.height as f32),

            text: vec![Text::new( &render_text )
                                    .with_color([0.0, 0.0, 0.0, 1.0])
                                    .with_scale(25.0), ],

            ..Section::default()
        });

        // Add the objects queued with GlyphBrush to the staging belt
        self.glyph_brush.draw_queued(
            &self.device,
            &mut self.staging_belt,
            &mut encoder,
            &view,
            self.size.width,
            self.size.height).expect("Draw queued");

        // Lock the objs in the staging belt 
        // the staging belt can no longer be modified until the contents
        // have been copied by the GPU
        self.staging_belt.finish();

        // Tell wgpu to finish the command buffer and submit it to the gpu's render queue
        self.queue.submit( std::iter::once( encoder.finish() ) );
        output.present();

        // Once the queue has been submitted we can recall
        // the staging belt and regain access to it
        self.staging_belt.recall();

        Ok( () ) //Return value


    } //end render() def

}


pub async fn run() {

    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut wgpu_handler = WgpuHandler::new(window).await;

    let wkdir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("{:?}", e)
    };

    let path = wkdir.join("file.txt"); //later prompt user to provide path 

    let doc_handler = DocHandler::load(path);

    //println!("{:?}", something);

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



                        //Return
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Return), .. },  ..
                        } => { doc_handler.newline() },

                        //Delete
                        WindowEvent::KeyboardInput {input: KeyboardInput {state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Delete), .. },  ..
                        } => { doc_handler.delete() },


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
                match wgpu_handler.render(&mut doc_handler.text) {
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
