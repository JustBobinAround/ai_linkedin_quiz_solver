chrome.runtime.onMessage.addListener(
  function(request, sender, sendResponse) {
    if (request.message === "fetchDocument") {
      const documentData = {
        title: document.title,
        url: window.location.href,
        content: document.documentElement.outerHTML
      };

      // Send the document data to popup.js
      chrome.runtime.sendMessage({ message: "documentData", data: documentData });
    }
  }
);
