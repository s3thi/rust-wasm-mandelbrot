# rust-wasm-mandelbrot

Draw the Mandelbrot fractal using Rust compiled to WebAssembly!

## Requirements

- Node v11.2
- NPM 6.4
- Rust 1.30.0-beta.16
- wasm-pack 0.5.1 (see https://github.com/rustwasm/wasm-pack)

## Running This Project

First, make sure all your dependencies are installed and at the correct version.

Then, clone this repository:

    $ git clone https://github.com/s3thi/rust-wasm-mandelbrot

Build the WASM module using `wasm-pack`:

    $ cd rust-wasm-mandelbrot/
    $ wasm-pack build

The generated WASM module, along with some glue JavaScript code and TypeScript type definitions, will be placed in the `pkg/` directory. Make it available as an NPM module:

    $ cd pkg/
    $ npm link

Then, go into the `www/` directory and install all the Node dependencies:

    $ cd ../www/
    $ npm i

Link the NPM package containing the WASM module:

    $ npm link rust-wasm-mandelbrot

Run the project:

    $ npm start

If everything went well, going to http://localhost:8080 should give you an image of the Mandelbrot fractal!
