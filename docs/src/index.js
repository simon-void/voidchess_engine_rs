import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';
import {INPUT_EVENT_TYPE, MOVE_INPUT_MODE, Chessboard} from "./cm-chessboard/Chessboard.js"

const states = {
    LOADING: "loading",
    HUMAN_TURN: "human_turn",
    ENGINE_TURN: "engine_turn",
};
const moveTypes = {
    NORMAL: "normal",
    PAWN_PROMOTION: "pawn_promo",
    EN_PASSANT: "en_passant",
    SHORT_CASTLING: "short_castling",
    LONG_CASTLING: "long_castling",
};

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

async function getAllowedMovesAsMap(movesSoFar) {
    let movesJson = await wasm.get_concatenated_allowed_moves("");
    let moveArray = JSON.parse(movesJson);
    return arrayOfMovesToMoveMap(moveArray);
}

function arrayOfMovesToMoveMap(arrayOfMoveStr) {
    /**
     * @description
     * Takes an Array<V>, and a grouping function,
     * and returns a Map of the array grouped by the grouping function.
     *
     * @param list An array of type V.
     * @param keyGetter A Function that takes the the Array type V as an input, and returns a value of type K.
     *                  K is generally intended to be a property key of V.
     *
     * @returns Map of the array grouped by the grouping function.
     */
    //export function groupBy<K, V>(list: Array<V>, keyGetter: (input: V) => K): Map<K, Array<V>> {
    //    const map = new Map<K, Array<V>>();
    function groupBy(list, keyGetter) {
        const map = new Map();
        list.forEach((item) => {
            const key = keyGetter(item);
            const collection = map.get(key);
            if (!collection) {
                map.set(key, [item]);
            } else {
                collection.push(item);
            }
        });
        return map;
    }

    let arrayOfMoves = arrayOfMoveStr.map(moveStr=> {
        let moveType = moveTypes.NORMAL;
        switch (moveStr.substring(2,3)) {
            case "-": moveType = moveTypes.NORMAL; break;
            case "Q": moveType = moveTypes.PAWN_PROMOTION; break;
            case "K": moveType = moveTypes.PAWN_PROMOTION; break;
            case "R": moveType = moveTypes.PAWN_PROMOTION; break;
            case "B": moveType = moveTypes.PAWN_PROMOTION; break;
            case "e": moveType = moveTypes.EN_PASSANT; break;
            case "c": moveType = moveTypes.SHORT_CASTLING; break;
            case "C": moveType = moveTypes.LONG_CASTLING; break;
            default: alert(`illegal move type in: ${moveStr}`)
        }
        return {
            from: moveStr.substring(0,2),
            to: moveStr.substring(3,5),
            type: moveType,
            asStr: moveStr,
        };
    });
    return groupBy(arrayOfMoves, move => move.from);
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div")
    log.innerText = text
    output.appendChild(log)
}

const _allowedMovesAtWhiteStartClassic = arrayOfMovesToMoveMap(
    JSON.parse('["b1-c3", "b1-a3", "g1-h3", "g1-f3", "a2-a3", "a2-a4", "b2-b3", "b2-b4", "c2-c3", "c2-c4", "d2-d3", "d2-d4", "e2-e3", "e2-e4", "f2-f3", "f2-f4", "g2-g3", "g2-g4", "h2-h3", "h2-h4"]')
);

function BoardModel(gameModel) {
    let self = this;
    self.board = new Chessboard(
        document.getElementById("board"),
        {
            position: "empty",
            moveInputMode: MOVE_INPUT_MODE.dragPiece,
            sprite: {url: "./assets/images/chessboard-sprite.svg"},
            style: {
                cssClass: "default",
                showCoordinates: false, // show ranks and files
                showBorder: true, // display a border around the board
            }
        }
    );
    self.board.enableMoveInput(event => {
        switch (event.type) {
            case INPUT_EVENT_TYPE.moveStart:
                let start_accepted = gameModel.allowedMoves().has(event.square);
                log(`moveStart: ${event.square}, accepted: ${start_accepted}`)
                return start_accepted;
            case INPUT_EVENT_TYPE.moveDone:
                let moveOrNull = gameModel.allowedMoves().get(event.squareFrom).find(move=>move.to===event.squareTo);
                let move_accepted = moveOrNull != null;
                log(`moveDone: ${event.squareFrom}-${event.squareTo}, accepted: ${move_accepted}`)
                if(move_accepted) {
                    gameModel.informOfMove(moveOrNull)
                }
                return move_accepted;
            case INPUT_EVENT_TYPE.moveCanceled:
                log(`moveCanceled`)
        }
    });
}

function GameModel() {
    let self = this;
    self.state = ko.observable(states.LOADING)
    self.allowedMoves = ko.observable(_allowedMovesAtWhiteStartClassic);
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
                let start_accepted = self.allowedMoves().has(event.square);
                log(`moveStart: ${event.square}, accepted: ${start_accepted}`)
                return start_accepted;
            case INPUT_EVENT_TYPE.moveDone:
                let moveOrNull = self.allowedMoves().get(event.squareFrom).find(move=>move.to===event.squareTo);
                let move_accepted = moveOrNull != null && moveOrNull !=undefined;
                log(`moveDone: ${event.squareFrom}-${event.squareTo}, accepted: ${move_accepted}`)
                return move_accepted;
            case INPUT_EVENT_TYPE.moveCanceled:
                log(`moveCanceled`)
        }
    });
    self.informOfMove = function (move) {
        self.movesPlayed.push(move.asStr);
    };
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