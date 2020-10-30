import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';
import {INPUT_EVENT_TYPE, MOVE_INPUT_MODE, Chessboard} from "./cm-chessboard/Chessboard.js"

window.board = new Chessboard(document.getElementById("board"), {
    position: "start",
    moveInputMode: MOVE_INPUT_MODE.dragPiece,
    sprite: {url: "./assets/images/chessboard-sprite.svg"}
})
window.board.enableMoveInput(inputHandler)
function inputHandler(event) {
    switch (event.type) {
        case INPUT_EVENT_TYPE.moveStart:
            log(`moveStart: ${event.square}`)
            return true
        case INPUT_EVENT_TYPE.moveDone:
            log(`moveDone: ${event.squareFrom}-${event.squareTo}`)
            return true
        case INPUT_EVENT_TYPE.moveCanceled:
            log(`moveCanceled`)
    }
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div")
    log.innerText = text
    output.appendChild(log)
}

async function display_greeting(name) {
    console.log("init wasm");
    await init();
    console.log("invoking wasm");
    let greeting = await wasm.get_greeting_for(name);
    console.log("received from wasm: " + greeting);
    setText(greeting);
}

function setText(text) {
    let msg_div=document.getElementById("msg_box");
    msg_div.innerText = text;
    console.log("set text to: " + text);
}

window.onload = function () {
    function update_greeting(event){
        display_greeting("Success!");
    }
    let greeting_button = document.getElementById('greeting_button');
    greeting_button.addEventListener("click", update_greeting,false);
}