console.log("Hello from background service worker. 👋");

chrome.runtime.onInstalled.addListener(() => {
    // default
    chrome.action.disable();

  chrome.declarativeContent.onPageChanged.removeRules(undefined, () => {
    let exampleRule = {
      conditions: [
        new chrome.declarativeContent.PageStateMatcher({
          pageUrl: {hostSuffix: '.powerlanguage.co.uk'},
        })
      ],
      actions: [new chrome.declarativeContent.ShowAction()],
    };

    // Finally, apply our new array of rules
    let rules = [exampleRule];
    chrome.declarativeContent.onPageChanged.addRules(rules);

    console.log("Enabled toolbar action on puzzle sites.");
  });
});

function perform_next_guess() {
    console.log("@perform_next_guess", wasm);
    wasm.perform_next_guess();
}

chrome.action.onClicked.addListener(async (tab) => {
  chrome.scripting.executeScript({
    target: {tabId: tab.id},
    func: perform_next_guess,
    args: [],
  });
});