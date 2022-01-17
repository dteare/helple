function console_log(msg) { console.log(msg); }

function perform_guess(word) {
    console.log(`Trying <${word}>`);

    let keyboard = document.getElementsByTagName("game-app")[0].shadowRoot.querySelectorAll("game-theme-manager")[0].querySelector("#game").querySelector("game-keyboard").shadowRoot;

    let delay = 0;
    [...word].forEach(c => {
        setTimeout(() => {
            pressKey(c);
        }, delay);

        delay += 200;
    });

    // TODO: expose as a setting
    setTimeout(() => {
        pressKey("↵");
    }, delay);

    function pressKey(c) {
        let key = keyboard.querySelector(`[data-key='${c}']`);
        key.click();
    }
}

function get_current_puzzle_state() {
    let state = { 
        guesses:game_board()
    };

    console.log("@get_current_puzzle_state (JS): ", state);

    return state;
}

function game_board() {
    let guesses = [];

    for (let row of document.getElementsByTagName("game-app")[0].shadowRoot.querySelectorAll("game-row")) { 
        let word = row.attributes.letters.value;
        let assignments = "";

        if (!word) {
            break;
        }

        for (let tile of row.shadowRoot.querySelectorAll("game-tile")) {
            switch (tile.attributes.evaluation.value) {
                case "correct": 
                    assignments += "X";
                    break;
                case "present": 
                    assignments += ".";
                    break;
                case "absent": 
                    assignments += "-";
                    break;
                default: 
                    console.warn("No idea what this is:", tile.attributes.evaluation);
                    console.warn("   for tile:", tile);
            }
        }
        guesses.push({ word:word, results:assignments});
    }
    return guesses;
}