let word = "RUSTY".toLowerCase();

let keyboard = document.getElementsByTagName("game-app")[0].shadowRoot.querySelectorAll("game-theme-manager")[0].querySelector("#game").querySelector("game-keyboard").shadowRoot;

let delay = 0;
[...word].forEach(c => {
    console.log(c);
    setTimeout(() => {
        pressKey(c);
    }, delay);

    delay += 200;
});

setTimeout(() => {
    pressKey("↵");
}, delay);


function pressKey(c) {
    let key = keyboard.querySelector(`[data-key='${c}']`);

    console.log(key);
    key.click();
}