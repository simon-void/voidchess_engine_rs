import init, * as wasm from './engine/voidchess_engine_wasm.js';
import {evaluateGame} from './worker_pool.js';
import {Chessboard, INPUT_EVENT_TYPE, MOVE_INPUT_MODE} from "./cm-chessboard/Chessboard.js"

console.log(`number of cores: ${navigator.hardwareConcurrency}`)

const states = {
    LOADING: "loading",
    HUMAN_TURN: "human_turn",
    ENGINE_TURN: "engine_turn",
    GAME_ENDED: "game ended",
};
const moveTypes = {
    NORMAL: "normal",
    PAWN_PROMOTION: "pawn_promo",
    EN_PASSANT: "en_passant",
    SHORT_CASTLING: "short_castling",
    LONG_CASTLING: "long_castling",
};
const gameEvalTypes = {
    GAME_ENDED: "GameEnded",
    MOVE_TO_PLAY: "MoveToPlay",
    ERROR: "Err",
};

let initPromise = init();

async function init_wasm() {
    await initPromise;
}

async function getFenResult(arrayOfMoveStr) {
    let fen = await wasm.get_fen(arrayOfMoveStr.join(' '));
    return JSON.parse(fen);
}

/**
 * @param {Array<string>} arrayOfMoveStr
 * @returns {Promise<Array<string>>}
 */
async function getAllowedMovesAsArray(arrayOfMoveStr) {
    let movesJson = await wasm.get_concatenated_allowed_moves(arrayOfMoveStr.join(' '));
    return JSON.parse(movesJson);
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
    const log = document.createElement("div");
    log.innerText = text;
    output.prepend(log);
}

const _allowedMoveStrArrayClassic = ["b1-c3", "b1-a3", "g1-h3", "g1-f3", "a2-a3", "a2-a4", "b2-b3", "b2-b4", "c2-c3", "c2-c4", "d2-d3", "d2-d4", "e2-e3", "e2-e4", "f2-f3", "f2-f4", "g2-g3", "g2-g4", "h2-h3", "h2-h4"];

function BoardModel(gameModel) {
    let self = this;
    self.board = new Chessboard(
        document.getElementById("board"),
        {
            position: "start",
            moveInputMode: MOVE_INPUT_MODE.dragPiece,
            sprite: {url: "./assets/images/chessboard-sprite.svg"},
            style: {
                // cssClass: "default",
                showCoordinates: true, // show ranks and files
                showBorder: true, // display a border around the board
            }
        }
    );
    self.board.enableMoveInput(event => {
        switch (event.type) {
            case INPUT_EVENT_TYPE.moveStart:
                return gameModel.allowedMoveMap().has(event.square);
            case INPUT_EVENT_TYPE.moveDone:
                let moveOrNull = gameModel.allowedMoveMap().get(event.squareFrom).find(move=>move.to===event.squareTo);
                let move_accepted = moveOrNull != null;
                if(move_accepted) {
                    setTimeout(()=>{
                        self.takeCareOfSpecialMoves(moveOrNull);
                        gameModel.informOfMove(moveOrNull)
                    },0);
                }
                return move_accepted;
            case INPUT_EVENT_TYPE.moveCanceled:
                //log(`moveCanceled`)
        }
    });
    self.takeCareOfSpecialMoves = function (move) {
        if(move.type!==moveTypes.NORMAL) {
            let moves_plus_ongoing_move = [...gameModel.moveStrPlayed(), move.asStr];
            getFenResult(moves_plus_ongoing_move).then(fenResult => {
                    if (fenResult.is_ok) {
                        let fen = fenResult.value;
                        self.board.setPosition(fen);
                    } else {
                        log(fenResult.value);
                    }
                }, reason => {
                    log(`error when invoking getFenResult: ${reason}`);
                }
            )
        }
    }
}

function GameModel() {
    let self = this;
    self.evaluation = ko.observable("waiting for your move");
    self.state = ko.observable(states.LOADING)
    self.allowedMoveStrArray = ko.observableArray(_allowedMoveStrArrayClassic);
    self.allowedMoveMap = ko.computed(()=>{
        return arrayOfMovesToMoveMap(self.allowedMoveStrArray());
    });
    self.moveStrPlayed = ko.observableArray([]);
    // this.fullName = ko.computed(function() {
    //     return this.firstName() + " " + this.lastName();
    // }, this);
    self.boardModel = new BoardModel(self);
    self.informOfMove = function (move) {
        let possibleMoves = [...self.allowedMoveStrArray()];
        self.allowedMoveStrArray([]);
        self.moveStrPlayed.push(move.asStr);
        self.state(states.ENGINE_TURN);

        evaluateGame(
            [...self.moveStrPlayed()],
            possibleMoves,
            self.evaluation,
        ).then(
            (gameEval) => {
                if (gameEval.result_type === gameEvalTypes.ERROR) {
                    log(gameEval.msg);
                }
                if (gameEval.result_type === gameEvalTypes.GAME_ENDED) {
                    self.evaluation(gameEval.msg);
                    self.state(states.GAME_ENDED);
                }
                if (gameEval.result_type === gameEvalTypes.MOVE_TO_PLAY) {
                    self.moveStrPlayed.push(gameEval.move_to_play);
                    self.evaluation(gameEval.eval);
                    let fen = gameEval.fen;
                    self.boardModel.board.setPosition(fen);

                    getAllowedMovesAsArray(self.moveStrPlayed()).then(
                        newAllowedMovesArray => {
                            if (newAllowedMovesArray.length === 0) {
                                log("no moves left")
                            }
                            self.allowedMoveStrArray(newAllowedMovesArray);
                        }, reason => {
                            alert(`couldn't compute allowed moves because of ${reason}`)
                        }
                    )

                    self.state(states.HUMAN_TURN);
                }
            }
        );
    };
}

window.onload = function () {
    let gameModel = new GameModel();
    ko.applyBindings(gameModel);

    init_wasm().then(_ => {
        gameModel.state(states.HUMAN_TURN);
    }, reason => {
        alert("Couldn't initialise wasm: " + reason);
    });
}