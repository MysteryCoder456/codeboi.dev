import init from "./pi_estimator.js";
const canvas = document.querySelector("#pi-canvas");

setTimeout(() => {
    console.log("e o");
    canvas.setAttribute("style", "width: 100%; aspect-ratio: calc(16/9);");
}, 1000);

init();
