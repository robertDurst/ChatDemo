/* tslint:disable */
import * as wasm from './crypto_module_bg';

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {
    
    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}
/**
* given a public key (e, n), encrypts message m for this public key using RSA.
* @param {number} arg0
* @param {string} arg1
* @returns {string}
*/
export function encrypt(arg0, arg1) {
    const [ptr1, len1] = passStringToWasm(arg1);
    const retptr = globalArgumentPtr();
    try {
        wasm.encrypt(retptr, arg0, ptr1, len1);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
        
        
    } finally {
        wasm.__wbindgen_free(ptr1, len1 * 1);
        
    }
    
}

/**
* stores the information for a given RSA keypair.
*/
export class Keypair {
    
    static __construct(ptr) {
        return new Keypair(ptr);
    }
    
    constructor(ptr) {
        this.ptr = ptr;
    }
    
    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_keypair_free(ptr);
    }
    /**
    * randomly generates a new keypair based on two seeds.
    * @param {Uint8Array} arg0
    * @param {Uint8Array} arg1
    * @returns {Keypair}
    */
    static new(arg0, arg1) {
        const [ptr0, len0] = passArray8ToWasm(arg0);
        const [ptr1, len1] = passArray8ToWasm(arg1);
        try {
            return Keypair.__construct(wasm.keypair_new(ptr0, len0, ptr1, len1));
            
        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);
            wasm.__wbindgen_free(ptr1, len1 * 1);
            
        }
        
    }
    /**
    * nicely outputs a formatted public key for use in the javascript code.
    * improved since 0.2.0. Now outputs just n as a radix 32 string similar
    * to how it is done here: http://gauss.ececs.uc.edu/Courses/c653/project/radix_32.html
    * @returns {string}
    */
    public_key_display_wasm() {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const retptr = globalArgumentPtr();
        wasm.keypair_public_key_display_wasm(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
        
    }
    /**
    * given a ciphertext, attempts to decrypt based on the private key and modulo from this keypair. Performs
    * simple decryption based on RSA algorithm.
    * @param {string} arg0
    * @returns {number}
    */
    decrypt(arg0) {
        if (this.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        const [ptr0, len0] = passStringToWasm(arg0);
        try {
            return wasm.keypair_decrypt(this.ptr, ptr0, len0);
            
        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);
            
        }
        
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

