import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';

let initPromise = init();
let hasNotBeenInitialized = true;

const positionEvaluator = new Worker("src/worker_evaluate_position.js", { type: "module" });

const isFirefox = navigator.userAgent.includes("Firefox");

/**
 * @param {Array<string>} moveStrPlayedArray
 * @param {Array<string>} possibleMoveArray
 * @param {(String): void}
 * @returns {Promise} GameEvaluation
 */
export function evaluateGame(moveStrPlayedArray, possibleMoveArray, updateStatus){
    let gameConfigStr = moveStrPlayedArray.join(' ');
    if( isFirefox ) {
        return evaluateGameEvalByPositionOnMainThread(gameConfigStr, possibleMoveArray, updateStatus);
    } else {
        return evaluateGameEvalByPositionWorker(gameConfigStr, possibleMoveArray, updateStatus);
    }
}

function evaluateGameEvalByPositionWorker(gameConfigStr, _possibleMoveArray, updateStatus){
    updateStatus("computing ...")
    return new Promise(resolve => {
        positionEvaluator.onmessage = function (messageEvent) {
            let gameEval = messageEvent.data;
            updateStatus("waiting for player")
            resolve(gameEval);
        }
        positionEvaluator.postMessage(gameConfigStr);
    });
}

async function evaluateGameEvalByPositionOnMainThread(gameConfigStr, _possibleMoveArray, updateStatus){
    updateStatus("computing ...")
    if (hasNotBeenInitialized) {
        await initPromise;
        hasNotBeenInitialized = false;
    }
    await new Promise(function(resolve) {
        setTimeout(resolve, 310);
    });
    let gameEvaluationStr = await wasm.evaluate_position_after(gameConfigStr);
    let gameEval = JSON.parse(gameEvaluationStr);
    updateStatus("waiting for player")
    return gameEval;
}