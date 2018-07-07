
                (function() {
                    var wasm;
                    const __exports = {};
                    

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null ||
        cachegetFloat32Memory.buffer !== wasm.memory.buffer)
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    return cachegetFloat32Memory;
}

let cachegetUint64Memory = null;
function getUint64Memory() {
    if (cachegetUint64Memory === null ||
        cachegetUint64Memory.buffer !== wasm.memory.buffer)
        cachegetUint64Memory = new BigUint64Array(wasm.memory.buffer);
    return cachegetUint64Memory;
}

function passArrayF32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getFloat32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

class DriftMeter {

                static __construct(ptr) {
                    return new DriftMeter(ptr);
                }

                constructor(ptr) {
                    this.ptr = ptr;
                }

            free() {
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.__wbg_driftmeter_free(ptr);
            }
        static new() {
    return DriftMeter.__construct(wasm.driftmeter_new());
}
calc_offset(arg0) {
    const [ptr0, len0] = passArrayF32ToWasm(arg0);
    try {
        return wasm.driftmeter_calc_offset(this.ptr, ptr0, len0);
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 4);
    }
}
hpcp_ptr() {
    return wasm.driftmeter_hpcp_ptr(this.ptr);
}
fft_window() {
    return wasm.driftmeter_fft_window(this.ptr);
}
}
__exports.DriftMeter = DriftMeter;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_sinf = function(x) { return Math.sin(x); };

__exports.__wbindgen_cosf = function(x) { return Math.cos(x); };

                    function init(wasm_path) {
                        return fetch(wasm_path)
                            .then(response => response.arrayBuffer())
                            .then(buffer => WebAssembly.instantiate(buffer, { './drift_meter': __exports }))
                            .then(({instance}) => {
                                wasm = init.wasm = instance.exports;
                                return;
                            });
                    };
                    self.wasm_bindgen = Object.assign(init, __exports);
                })();
            