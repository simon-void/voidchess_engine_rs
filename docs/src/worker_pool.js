import init, * as wasm from './engine/voidchess_engine_wasm.js';

let initPromise = init();
let hasNotBeenInitialized = true;

const isFirefox = navigator.userAgent.includes("Firefox");
const nrOfWorkersToUse = Math.max(1, (navigator.hardwareConcurrency || 1) - 1);

const positionEvaluatorWorker = new Worker("src/worker_evaluate_position.js", { type: "module" });
const moveEvaluatorWorkerArray = Array(nrOfWorkersToUse).fill().map((_, i) =>
    new Worker("src/worker_evaluate_move.js", { type: "module" })
);
const usesOnlySingleWorker = moveEvaluatorWorkerArray.length===1;

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
        if( usesOnlySingleWorker ) {
            return evaluateGameEvalByPositionWorker(gameConfigStr, possibleMoveArray, updateStatus);
        } else {
            return evaluateGameEvalByMoveWorker(gameConfigStr, possibleMoveArray, updateStatus);
        }
    }
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

function evaluateGameEvalByPositionWorker(gameConfigStr, _possibleMoveArray, updateStatus){
    updateStatus("computing ...")
    return new Promise(resolve => {
        positionEvaluatorWorker.onmessage = function (messageEvent) {
            let gameEval = JSON.parse(messageEvent.data.gameEvalJson);
            updateStatus("waiting for player")
            resolve(gameEval);
        }
        positionEvaluatorWorker.postMessage({
            "gameConfig": gameConfigStr
        });
    });
}

function evaluateGameEvalByMoveWorker(gameConfigStr, possibleMoveArray, updateStatus){
    const nrOfMoves = possibleMoveArray.length;
    let computedGameEvalJsonArray = []; // array of GameEvaluation
    updateStatus(`0/${nrOfMoves}`);
    return new Promise(resolve => {
        let workersToStart = Math.min(moveEvaluatorWorkerArray.length, possibleMoveArray.length);
        console.log(`start ${workersToStart} workers to compute next move from ${possibleMoveArray.length} possible ones`);
        moveEvaluatorWorkerArray.forEach((_, workerIndex)=>{
            if (workerIndex < workersToStart) {
                startWorker(
                    gameConfigStr,
                    workerIndex,
                    nrOfMoves,
                    possibleMoveArray,
                    computedGameEvalJsonArray,
                    updateStatus,
                    resolve,
                );
            }
        });
    });
}

/**
 * @param {string} gameConfigStr the game so far (moves divided by space)
 * @param {number} workerIndex
 * @param {number} nrOfMoves
 * @param {Array<string>} movesLeftToComputeArray
 * @param {Array<string>} computedGameEvalJsonArray
 * @param updateStatus invoke with a string to indicate progress
 * @param resolve resolves the main promise that is expected to return the next gameEval to play (or end the game)
 */
function startWorker(
    gameConfigStr,
    workerIndex,
    nrOfMoves,
    movesLeftToComputeArray,
    computedGameEvalJsonArray,
    updateStatus,
    resolve,
) {
    let moveStr = movesLeftToComputeArray.pop();
    console.log(`start worker ${workerIndex} on move ${moveStr}. (${movesLeftToComputeArray.length} other move(s) left to compute)`);
    let moveEvaluatorWorker = moveEvaluatorWorkerArray[workerIndex];
    moveEvaluatorWorker.onmessage = function (messageEvent) {
        let gameEvalJson = messageEvent.data.gameEvalJson;
        console.log(`result for move ${moveStr} -> eval: ${gameEvalJson}`);
        computedGameEvalJsonArray.push(gameEvalJson);
        let nrOfEvaluatedMoves = computedGameEvalJsonArray.length;
        updateStatus(`${nrOfEvaluatedMoves}/${nrOfMoves}`)
        // either start a new worker (if moves are left to compute) or select/resolve the move to play
        if(movesLeftToComputeArray.length!==0) {
            startWorker(
                gameConfigStr,
                workerIndex,
                nrOfMoves,
                movesLeftToComputeArray,
                computedGameEvalJsonArray,
                updateStatus,
                resolve,
            )
        } else {
            if(nrOfEvaluatedMoves===nrOfMoves) {
                console.log(`pick one of these moves: ${computedGameEvalJsonArray}`)
                pickMoveToPlay(computedGameEvalJsonArray).then(chosenGameEval => {
                    resolve(chosenGameEval);
                });
            }
        }
    }
    moveEvaluatorWorker.postMessage({
        "gameConfig": gameConfigStr,
        "moveStr": moveStr,
        "workerIndex": workerIndex,
    });
}

async function pickMoveToPlay(gameEvalArray){
    if (hasNotBeenInitialized) {
        await initPromise;
        hasNotBeenInitialized = false;
    }
    let gameEvalArrayStr = gameEvalArray.join('|');
    let gameEvaluationStr = await wasm.pick_move_to_play(gameEvalArrayStr);
    console.log(`picked game eval: ${gameEvaluationStr}`)
    return JSON.parse(gameEvaluationStr);
}
