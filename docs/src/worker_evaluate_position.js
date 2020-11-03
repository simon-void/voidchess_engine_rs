import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';

onmessage = function (messageEvent) {
    evaluatePositionAfter(messageEvent.data).then(gameEval=>{
        postMessage(gameEval);
    }, reason => {
        postMessage({
            "type": "ERR",
            "msg":"worker problem: " + reason,
        })
    });
}

async function evaluatePositionAfter(arrayOfMoveStr) {
    let initDone = init();
    let gameConfig = arrayOfMoveStr.join(' ');
    await initDone;
    let gameEvaluationJson = await evaluate_position_after(gameConfig);
    return JSON.parse(gameEvaluationJson);
}

// positionEvaluator.postMessage([...self.moveStrPlayed()]);
// positionEvaluator.onmessage = function (messageEvent) {
//     let gameEval = messageEvent.data;
//     if (gameEval.type == gameEvalTypes.ERROR) {
//         log(gameEval.msg);
//     }
//     if (gameEval.type == gameEvalTypes.GAME_ENDED) {
//         self.evaluation(gameEval.msg);
//         self.state(states.GAME_ENDED);
//     }
//     if (gameEval.type == gameEvalTypes.MOVE_TO_PLAY) {
//         self.moveStrPlayed.push(gameEval.move);
//         self.evaluation(gameEval.eval);
//         let fen = gameEval.fen;
//         self.boardModel.board.setPosition(fen);
//
//         getAllowedMovesAsMap(self.moveStrPlayed()).then(
//             newAllowedMoves => {
//                 if (newAllowedMoves.size == 0) {
//                     log("no moves left")
//                 }
//                 self.allowedMoves(newAllowedMoves);
//             }, reason => {
//                 alert(`couldn't compute allowed moves because of ${reason}`)
//             }
//         )
//
//         self.state(states.HUMAN_TURN);
//     }
// };