// popup.js

const widgetContainer = document.getElementById("widget-container");
const loadingSpinner = document.getElementById("loading-spinner");

const socket = new WebSocket("ws://127.0.0.1:9001");

socket.addEventListener("open", (event) => {
  console.log("WebSocket connection opened!");
});

const content = document.createElement('button');

content.innerText = "ANSWER PLS";
content.style.fontSize = '24px';
content.style.display = 'inline-block';
content.style.padding = '10px 20px';
content.style.fontSize = '18px';
content.style.fontWeight = 'bold';
content.style.textAlign = 'center';
content.style.textDecoration = 'none';
content.style.cursor = 'pointer';
content.style.backgroundColor = '#3498db';
content.style.color = '#ffffff';
content.style.border = '2px solid #3498db';
content.style.borderRadius = '5px';
content.style.transition = 'background-color 0.3s, color 0.3s, border 0.3s';



content.addEventListener('click', () => {
  chrome.tabs.query({ active: true, currentWindow: true }, function(tabs) {
    chrome.tabs.sendMessage(tabs[0].id, { message: "fetchDocument" });
  });
});

widgetContainer.appendChild(content);


socket.addEventListener("message", (event) => {
    document.getElementById('loading-spinner').style.display = 'none';
    const message = document.createElement('p');
    message.innerText = event.data;
    message.style.fontSize = '24px';
    widgetContainer.appendChild(message);
    widgetContainer.scrollTop = widgetContainer.scrollHeight;
});

socket.addEventListener("close", (event) => {
  if (event.wasClean) {
    console.log(`Connection closed cleanly, code=${event.code}, reason=${event.reason}`);
  } else {
    console.error("Connection abruptly closed");
  }
});

socket.addEventListener("error", (error) => {
  console.error(`WebSocket Error: ${error}`);
});


chrome.runtime.onMessage.addListener(function(request, sender, sendResponse) {
  if (request.message === "documentData") {
    document.getElementById('loading-spinner').style.display = 'inline-block';
    socket.send(request.data.content);
  }
});

