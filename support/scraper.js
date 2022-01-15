let pretty_display = "";
let helple_input = "";

for (let row of document.getElementsByTagName("game-app")[0].shadowRoot.querySelectorAll("game-row")) { 
    if (!row._letters) {
        break;
    }

    for (let tile of row.shadowRoot.querySelectorAll("game-tile")) {
        switch (tile._state) {
            case "correct": 
                pretty_display += "🟩";
                helple_input += "X";
                break;
            case "present": 
                pretty_display += "🟨";
                helple_input += ".";
                break;
            case "absent": 
                pretty_display += "⬜️";
                helple_input += "-";
                break;
            default: 
                console.log("No idea what this is:" + tile.dataset["evaluation"], tile);
        }
    }
    pretty_display += "\n";
    helple_input += "\n";
}

console.log(pretty_display);
console.log(helple_input);