import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';
// import * as ko from './libs/knockout-3.5.1';
import {INPUT_EVENT_TYPE, MOVE_INPUT_MODE, Chessboard} from "./cm-chessboard/Chessboard.js"


async function init_wasm() {
    console.log("init wasm");
    await init();
}

async function display_greeting(name) {
    console.log("invoking wasm");
    let greeting = await wasm.get_concatenated_allowed_moves("");
    console.log("received from wasm: " + greeting);
    log(greeting);
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div")
    log.innerText = text
    output.appendChild(log)
}

const _allowedMovesAtWhiteStartClassic = JSON.parse('["b1-c3", "b1-a3", "g1-h3", "g1-f3", "a2-a3", "a2-a4", "b2-b3", "b2-b4", "c2-c3", "c2-c4", "d2-d3", "d2-d4", "e2-e3", "e2-e4", "f2-f3", "f2-f4", "g2-g3", "g2-g4", "h2-h3", "h2-h4"]')
const states = {
    LOADING: "loading",
    HUMAN_TURN: "human_turn",
    ENGINE_TURN: "engine_turn",
};

function GameModel() {
    let self = this;
    self.state = ko.observable(states.LOADING)
    self.allowedMoves = ko.observableArray(_allowedMovesAtWhiteStartClassic);
    self.movesPlayed = ko.observableArray([]);
    // this.fullName = ko.computed(function() {
    //     return this.firstName() + " " + this.lastName();
    // }, this);
    self.board = new Chessboard(
        document.getElementById("board"),
        {
            position: "start",
            moveInputMode: MOVE_INPUT_MODE.dragPiece,
            sprite: {url: "./assets/images/chessboard-sprite.svg"}
        }
    );
    self.board.enableMoveInput(event => {
        switch (event.type) {
            case INPUT_EVENT_TYPE.moveStart:
                let start_accepted = true;
                log(`moveStart: ${event.square}, accepted: ${start_accepted}`)
                return start_accepted;
            case INPUT_EVENT_TYPE.moveDone:
                let move_accepted = true;
                log(`moveDone: ${event.squareFrom}-${event.squareTo}, accepted: ${move_accepted}`)
                return move_accepted;
            case INPUT_EVENT_TYPE.moveCanceled:
                log(`moveCanceled`)
        }
    });
    // window.gameModel = self;
}

window.onload = function () {
    let gameModel = new GameModel();
    ko.applyBindings(gameModel);

    let greeting_button = document.getElementById('greeting_button');
    greeting_button.addEventListener("click", () => {
        display_greeting("Success!");
    }, false);

    init_wasm().then(_ => {
        gameModel.state(states.HUMAN_TURN);
    }, reason => {
        alert("Couldn't initialise wasm: " + reason);
    });
}