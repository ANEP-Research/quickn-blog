import init, { run_app } from './pkg/blog_frontend.js';
async function main() {
   await init('/static/wasm/blog_frontend_bg.wasm');
   run_app();
}
main()