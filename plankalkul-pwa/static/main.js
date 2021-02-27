import init, { run_app } from './pkg/plankalkul_pwa.js';
async function main() {
   await init('./pkg/plankalkul_pwa_bg.wasm');
   run_app();
}
main()