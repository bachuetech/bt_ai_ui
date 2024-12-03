// Function to handle the user input and call the API functions
async function submitRequest() {
    document.getElementById('chat-container').style.display = 'block';
  
    const input = document.getElementById('user-input').value;
    const selectedModel = getSelectedModel();
    var model_ctx = retrieveContext(selectedModel);
    data = { current_time: getCurrentTime(), current_date: getCurrentDate(),  model: selectedModel, prompt: input }
    if (model_ctx) {
      data = { current_time: getCurrentTime(), current_date: getCurrentDate(), model: selectedModel, prompt: input, context: model_ctx }
    }
  
  
    // Create user message element and append to chat history
    let chatHistory = document.getElementById('chat-history');
    let userMessageDiv = document.createElement('div');
    userMessageDiv.className = 'mb-2 user-message';
    userMessageDiv.innerText = 'You:\n' + input;
    chatHistory.appendChild(userMessageDiv);
  
    //autoScroller.observe(userMessageDiv); //ToDo Why is not scrolling?
    scrollToElement(userMessageDiv);
  
  
    // Create response container
    let responseDiv = document.createElement('div');
    responseDiv.className = 'response-message mb-2 text-start';
    responseDiv.style.minHeight = '3em'; // make sure div does not shrink if we cancel the request when no text has been generated yet
  
  
    //Thinking Spinner
    spinner = document.createElement('div');
    spinner.className = 'spinner-border text-light';
    spinner.setAttribute('role', 'status');
    responseDiv.appendChild(spinner);
    chatHistory.appendChild(responseDiv);
  
    //Thinking Spinner in Nav (left) panel
    let spinner_nav = document.createElement('div');
    spinner_nav.className = 'spinner-border text-light';
    spinner_nav.setAttribute('role', 'status');
    app_name_holder = document.getElementById('app_name');
    app_name_holder.insertAdjacentElement('afterend', spinner_nav);
    //*********************************** */
  
    // create button to stop text generation
    let interrupt = new AbortController();
    let stopButton = document.createElement('button');
    stopButton.className = 'btn btn-danger';
    stopButton.innerHTML = 'Stop';
    stopButton.onclick = (e) => {
      responseDiv.innerHTML = "Cancelled by you.";
      e.preventDefault();
      interrupt.abort('Stop button pressed');
    }
    // add button after sendButton
    const sendButton = document.getElementById('send-button');
    sendButton.insertAdjacentElement('beforebegin', stopButton);
  
    // change autoScroller to keep track of our new responseDiv
    autoScroller.observe(responseDiv);
  
    postRequest(chat_URL, data, interrupt.signal)
      .then(async response => {
        if (!response.ok) {
          throw new Error(`Network response was not ok Status: ${response.status}`);
        }
        return await response.json(); //response.text(); //
      })
      .then(json_response => {
        //json_data = JSON.stringify(data);
        if (json_response.done) {
          chatHistory.context = json_response.context;
          saveContext(selectedModel, chatHistory.context);
        }
  
        // add data to response
        if (json_response != undefined && json_response != "") {
          if (responseDiv.hidden_text == undefined) {
            responseDiv.hidden_text = "";
          }
          responseDiv.hidden_text += json_response.message.content;
          responseDiv.hidden_text = fixLatexIssue(formatNumbers(responseDiv.hidden_text));
          responseDiv.innerHTML = responseDiv.hidden_text;
  
  
          renderMathInElement(responseDiv, {
            displayMode: true,
            trustContext: true,
            delimiters: [
              { left: '$$', right: '$$', display: true },
              { left: '\\(', right: '\\)', display: false },
              { left: '\\[', right: '\\]', display: true },
              { left: '\\\\(', right: '\\\\\)', display: false },
              { left: '\\\\[', right: '\\\\\]', display: true }
            ],
            throwOnError: false
          });
          responseDiv.innerHTML = DOMPurify.sanitize(prettifyCode(cleanHTMLEntities(markdownToHTMLCleansing(marked.parse(responseDiv.innerHTML)))));
        }
      })
      //})
      .then(() => {
        stopButton.remove();
        PR.prettyPrint();
        responseDiv.innerHTML = DOMPurify.sanitize("<strong>" + selectedModel + ":</strong><br/>" + responseDiv.innerHTML); //ToDo: Add DOMPurify.sanitize( Control for prettyPrint porbably?
  
        // Copy button
        let copyButton = document.createElement("button");
        copyButton.className = "btn btn-secondary copy-button";
        copyButton.innerHTML = clipboardIcon;
        copyButton.onclick = function () {
          copyToClipboard(responseDiv.textContent);
        };
        responseDiv.appendChild(copyButton);
  
        // Speak button
        let speakButton = document.createElement("button");
        speakButton.className = "btn btn-secondary speak-button";
        speakButton.innerHTML = megaphoneIcon;
        speakButton.onclick = function () {
          t2s.readAloud(responseDiv.textContent);
          //copyToClipboard(responseDiv.textContent);
        };
        //responseDiv.appendChild(speakButton);
        responseDiv.appendChild(speakButton);
  
        spinner.remove();
        spinner_nav.remove();
        saveChat();
        //scrollToElement(responseDiv);
      })
      .catch(error => {
        if (error !== 'Stop button pressed') {
          console.error(error);
        }
        stopButton.remove();
        spinner_nav.remove();
        spinner.remove();
      });
  
    // Clear user input
    const element = document.getElementById('user-input');
    element.value = '';
    $(element).css("height", textBoxBaseHeight + "px");
  }
  
  // *************************************************************
  // Function to handle the user input and call the API functions
  async function submitStreamingRequest() {
    document.getElementById('chat-container').style.display = 'block';
  
    const input = document.getElementById('user-input').value;
    const selectedModel = getSelectedModel();
    const contextVOLD = document.getElementById('chat-history').context; //ToDo: Retrieve model for last resoruce to assign model if not found in localStorage. Is that even possible??
    var model_ctx = retrieveContext(selectedModel);
    const systemPrompt = document.getElementById('system-prompt').value;
    //const data = { model: selectedModel, prompt: input, context: context, system: systemPrompt };
    //var data = { model: selectedModel, prompt: input, system: systemPrompt };
    data = { current_time: getCurrentDateTime(), model: selectedModel, prompt: input, context: "", system: systemPrompt}
    if (model_ctx ){
      data = { current_time: getCurrentDateTime(), model: selectedModel, prompt: input, context: model_ctx, system: systemPrompt}
    }
  
  
    // Create user message element and append to chat history
    let chatHistory = document.getElementById('chat-history');
    let userMessageDiv = document.createElement('div');
    userMessageDiv.className = 'mb-2 user-message';
    userMessageDiv.innerText = 'You:\n'+input;
    chatHistory.appendChild(userMessageDiv);
  
    //autoScroller.observe(userMessageDiv); //ToDo Why is not scrolling?
    scrollToElement(userMessageDiv);
  
  
    // Create response container
    let responseDiv = document.createElement('div');
    responseDiv.className = 'response-message mb-2 text-start';
    responseDiv.style.minHeight = '3em'; // make sure div does not shrink if we cancel the request when no text has been generated yet
  
  
    //Thinking Spinner
    spinner = document.createElement('div');
    spinner.className = 'spinner-border text-light';
    spinner.setAttribute('role', 'status');
    responseDiv.appendChild(spinner);
    chatHistory.appendChild(responseDiv);
  
    //Thinking Spinner in Nav (left) panel
    let spinner_nav = document.createElement('div');
    spinner_nav.className = 'spinner-border text-light';
    spinner_nav.setAttribute('role', 'status');
    app_name_holder = document.getElementById('app_name');
    app_name_holder.insertAdjacentElement('afterend', spinner_nav);
    //*********************************** */
  
    // create button to stop text generation
    let interrupt = new AbortController();
    let stopButton = document.createElement('button');
    stopButton.className = 'btn btn-danger';
    stopButton.innerHTML = 'Stop';
    stopButton.onclick = (e) => {
      responseDiv.innerHTML = "Cancelled by you.";
      e.preventDefault();
      interrupt.abort('Stop button pressed');
    }
    // add button after sendButton
    const sendButton = document.getElementById('send-button');
    sendButton.insertAdjacentElement('beforebegin', stopButton);
  
    // change autoScroller to keep track of our new responseDiv
    autoScroller.observe(responseDiv);
  
    postRequest(data, interrupt.signal)
      .then(async response => {
          await getStreamingResponse(response, parsedResponse => {  
          let word = parsedResponse.response;
          if (parsedResponse.done) {
            chatHistory.context = parsedResponse.context;
            saveContext(selectedModel, chatHistory.context);
          }
  
          // add word to response
          if (word != undefined && word != "") {
            if (responseDiv.hidden_text == undefined){
              responseDiv.hidden_text = "";
            }
            responseDiv.hidden_text += word;
            responseDiv.hidden_text = fixLatexIssue(formatNumbers(responseDiv.hidden_text)); 
            responseDiv.innerHTML = responseDiv.hidden_text;
             renderMathInElement(responseDiv, {
              displayMode: true,
              trustContext: true,
              delimiters: [
                  {left: '$$', right: '$$', display: true},
                  {left: '\\(', right: '\\)', display: false},
                  {left: '\\[', right: '\\]', display: true},
                  {left: '\\\\(', right: '\\\\\)', display: false},
                  {left: '\\\\[', right: '\\\\\]', display: true}
              ],
              // • rendering keys, e.g.:
              throwOnError : false
            });
            responseDiv.innerHTML = DOMPurify.sanitize(prettifyCode(cleanHTMLEntities(markdownToHTMLCleansing(marked.parse(responseDiv.innerHTML)))));
          }
        });
      })
      .then(() => {
        stopButton.remove(); // Remove stop button from DOM now that all text has been generated
        PR.prettyPrint();
 
        responseDiv.innerHTML = DOMPurify.sanitize("<strong>"+selectedModel+":</strong><br/>"+responseDiv.innerHTML); //ToDo: Add DOMPurify.sanitize( Control for prettyPrint porbably?
  
            // Copy button
            let copyButton = document.createElement("button");
            copyButton.className = "btn btn-secondary copy-button";
            copyButton.innerHTML = clipboardIcon;
            copyButton.onclick = function () {
              copyToClipboard(responseDiv.textContent);
            };
            responseDiv.appendChild(copyButton);
  
            // Speak button
            let speakButton = document.createElement("button");
            speakButton.className = "btn btn-secondary speak-button";
            speakButton.innerHTML = megaphoneIcon;
            speakButton.onclick = function () {
              t2s.readAloud(responseDiv.textContent);
            };
            responseDiv.appendChild(speakButton);
  
        spinner.remove();
        spinner_nav.remove();
        saveChat();
      })
      .catch(error => {
        if (error !== 'Stop button pressed') {
          console.error(error);
        }
        stopButton.remove();
        spinner_nav.remove();
        spinner.remove();
      });
  
    // Clear user input
    const element = document.getElementById('user-input');
    element.value = '';
    $(element).css("height", textBoxBaseHeight + "px");
  }

  // Function to get the selected model
function getSelectedModel() {
    return document.getElementById('model-select').value;
  }