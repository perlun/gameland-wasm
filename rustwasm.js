class RustWasm {
  constructor() {
    this.ctx = null;
    this.img = null;
    this.pointer = null;
    this.frame = 0;
    this.width  = 320;
    this.height = 200;

    this.boundStep = this.step.bind(this)
  }

  prepare() {
    fetch("feistel.wasm", { cache: "no-cache" }).then(response =>
      response.arrayBuffer()
    ).then(bytes =>
      WebAssembly.instantiate(bytes, {})
    ).then(results => {
      let module = this.module = {};
      let mod = results.instance;
      module.alloc   = mod.exports.alloc;
      module.dealloc = mod.exports.dealloc;
      module.fill    = mod.exports.fill;
      module.clear   = mod.exports.clear;

      let byteSize = this.width * this.height * 4;
      this.pointer = module.alloc(byteSize);
      var buffer = new Uint8Array(mod.exports.memory.buffer, this.pointer, byteSize);

      var canvas = document.getElementById('screen');

      if (canvas.getContext) {
        this.ctx = canvas.getContext('2d');
        this.pointer = module.alloc(byteSize);

        var usub = new Uint8ClampedArray(mod.exports.memory.buffer, this.pointer, byteSize);
        this.img = new ImageData(usub, this.width, this.height);
      }
    });
  }

  step(timestamp) {
    if (!this.running) {
      return;
    }

    this.frame = this.module.fill(this.pointer, this.width, this.height, this.frame);
    // this.ctx.putImageData(this.img, 0, 0)
    console.log(this.frame);

    window.requestAnimationFrame(this.boundStep);
  }

  startAnimation() {
    this.running = true;

    let animationFunction = function() {
      this.ctx.clearRect(0, 0, this.width, this.height);
      this.module.clear(this.pointer, this.width, this.height);
      this.frame = 0;
      window.requestAnimationFrame(this.boundStep);
    }.bind(this);
    window.requestAnimationFrame(animationFunction);
  }

  stopAnimation() {
    this.running = false;
  }
}
