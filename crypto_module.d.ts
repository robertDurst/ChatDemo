/* tslint:disable */
export function generate_prime(arg0: number, arg1: number, arg2: Uint8Array): string;

export function encrypt(arg0: string, arg1: string, arg2: string): string;

export function greet(arg0: string): void;

export class Keypair {
free(): void;
static  new(arg0: Uint8Array, arg1: Uint8Array): Keypair;

 public_key_display_wasm(): string;

 decrypt(arg0: string): string;

}
