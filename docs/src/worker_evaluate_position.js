import init from './engine/voidchess_engine_rs.js';
import * as wasm from './engine/voidchess_engine_rs.js';

let hasNotBeenInitialized = true;

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
    if (hasNotBeenInitialized) {
        console.log("initialize wasm")
        await init();
        hasNotBeenInitialized = false;
    }
    let gameConfig = arrayOfMoveStr.join(' ');
    let gameEvaluationJson = await wasm.evaluate_position_after(gameConfig);
    return JSON.parse(gameEvaluationJson);
}
