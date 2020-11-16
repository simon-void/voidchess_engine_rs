import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';

let initPromise = init();
let hasNotBeenInitialized = true;

onmessage = function (messageEvent) {
    let gameConfig = messageEvent.data.gameConfig;
    let moveStr = messageEvent.data.moveStr;
    let workerIndex = messageEvent.data.workerIndex;
    evaluateMove(gameConfig, moveStr).then(gameEvalJson=>{
        postMessage({
            "gameEvalJson": gameEvalJson,
            "workerIndex": workerIndex,
        });
    }, reason => {
        postMessage({
            "type": "ERR",
            "msg":"worker problem: " + reason,
        })
    });
}

async function evaluateMove(gameConfig, moveStr) {
    if (hasNotBeenInitialized) {
        await initPromise;
        hasNotBeenInitialized = false;
    }
    let gameEvaluationJson = await wasm.evaluate_position_after(gameConfig);
    return gameEvaluationJson;
}
