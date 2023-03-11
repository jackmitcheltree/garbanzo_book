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

pub struct WgpuHandler {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration, 
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: GlyphBrush<()>
}

impl WgpuHandler {
    // Constructor method
    pub async fn new(window: Window) -> Self {
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
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        //input() returns a bool to indicate whether an event has been fully processed. 
        //If the method returns true, the main loop won't process the event any further

        false
    } //end input() def

    //
    pub fn update(&mut self) {
        //not in use
    } //end update() def

    //
    pub fn render(&mut self, text: &Vec<String>) -> Result<(), wgpu::SurfaceError> {

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
            render_text.push_str("\n");
        };

        println!("{:?}", render_text);

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