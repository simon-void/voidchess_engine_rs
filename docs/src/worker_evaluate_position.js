import init from './engine/voidchess_engine_wasm.js';
import * as wasm from './engine/voidchess_engine_wasm.js';

let initPromise = init();
let hasNotBeenInitialized = true;

onmessage = function (messageEvent) {
    evaluatePositionAfter(messageEvent.data.gameConfig).then(gameEvalJson=>{
        postMessage({
            "gameEvalJson": gameEvalJson,
        });
    }, reason => {
        postMessage({
            "type": "ERR",
            "msg":"worker problem: " + reason,
        })
    });
}

async function evaluatePositionAfter(gameConfig) {
    if (hasNotBeenInitialized) {
        await initPromise;
        hasNotBeenInitialized = false;
    }
    let gameEvaluationJson = await wasm.evaluate_position_after(gameConfig);
    return gameEvaluationJson;
}
