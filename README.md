# AI LinkedIn Quiz Solver Extension

This Firefox extension, named "LinkedIn Quiz Solver," is designed to assist you
in obtaining answers to LinkedIn quizzes. Please note that the use of this
extension may violate LinkedIn's terms of service. See the disclaimer; don't
get mad if you get banned...

## Installation

Follow the steps below to install the "LinkedIn Quiz Solver" extension using
Firefox's extension debug option:

### Prerequisites

- [Firefox Browser](https://www.mozilla.org/en-US/firefox/download/) installed on your machine.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed for running the server.

### Installation Steps

1. Download the extension code or clone the repository to your local machine.

```bash
git clone https://github.com/JustBobinAround/ai_linkedin_quiz_solver.git
```

2. Open the Firefox browser and navigate to the URL `about:debugging#/runtime/this-firefox`

3. Click the "Load Temporary Add-on..." button.

4. In the file dialog that opens, navigate to the directory where you
   downloaded or cloned the repo and then the extension folder. Select the
   `manifest.json` file.

5. The "LinkedIn Quiz Solver" extension should now be loaded and active.

### Running the Server

#### Important Note

Before running the server, ensure that you have exported your OpenAI API key
to an environment variable. Set the API key using the following command:

```bash
export OPENAI_API_KEY="your-api-key"
```

Replace `"your-api-key"` with your actual OpenAI API key. The server will not work
without it.


The extension relies on a server for communication. Follow these steps to run
the server:

1. Open a terminal and navigate to the directory where you downloaded or cloned
   the extension.

2. Inside the repo, run the server using Cargo.

```bash
cargo run
```

3. The server should now be running on `ws://127.0.0.1:9001`. Ensure that the
   extension's manifest file (`manifest.json`) has the correct host permission
   specified.

## Usage

Once the extension is installed, active, and the server is running, follow
these steps:

1. Open LinkedIn in your Firefox browser.

2. Navigate to the quiz you want to solve.

3. Click on the extension icon in the toolbar, and a popup should appear.

4. Click on the ANSWER PLS button. This will parse the quiz and send a request to openai.

4. The extension will provide the answers to the quiz questions **most of the time**.

**OpenAi's api has been timing out lately, it may take a few tries to get an answer**

**Some quizzes have images or complex code, don't be surprised if you fail a test**

## Important Note

Use this extension responsibly and in compliance with all applicable laws and
regulations. The authors and contributors of this extension are not responsible
for any misuse or violations of terms of service on third-party platforms.

## Disclaimer

This extension is provided "as is" with no warranties. The authors and
contributors are not responsible for any consequences resulting from the use or
misuse of this extension.

---

**Note:** Ensure that you have the necessary technical skills and understanding
of the potential consequences before using this extension. Use it at your own
risk.

