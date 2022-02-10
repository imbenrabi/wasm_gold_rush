import init, { greet } from "../pkg/wasm_gold_rush";
init().then(function () {
    greet('Welcome to Gold rush!');
}).catch(error => {
    console.error(error);
});
