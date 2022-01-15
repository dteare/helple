let keyboard = document
  .getElementsByTagName("game-app")[0]
  .shadowRoot.querySelectorAll("game-theme-manager")[0]
  .querySelector("#game")
  .querySelector("game-keyboard").shadowRoot;

function guess(word) {
  word = word.toLowerCase();

  let delay = 0;
  [...word].forEach((c) => {
    console.log(c);
    setTimeout(() => {
      pressKey(c);
    }, delay);

    delay += 200;
  });

  setTimeout(() => {
    pressKey("↵");
    printResults();
  }, delay);
}

function pressKey(c) {
  let key = keyboard.querySelector(`[data-key='${c}']`);

  console.log(key);
  key.click();
}

function printResults() {
  let pretty_display = "";
  let helple_input = "";

  for (let row of document
    .getElementsByTagName("game-app")[0]
    .shadowRoot.querySelectorAll("game-row")) {
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
          console.log(
            "No idea what this is:" + tile.dataset["evaluation"],
            tile
          );
      }
    }
    pretty_display += "\n";
    helple_input += "\n";
  }

  console.log(pretty_display);
  console.log(helple_input);
}
