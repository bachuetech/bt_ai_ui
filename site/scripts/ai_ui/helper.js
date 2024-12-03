function copyToClipboard(text) {
    if (navigator.clipboard) {
        navigator.clipboard.writeText(text)
            .then(() => {
                console.log('Text copied to clipboard!');
            })
            .catch(err => {
                console.error('Failed to copy text: ', err);
            });
    } else {
        //Legacy compatibility
        // Create new element
        var tmpTxtArea = document.createElement('textarea');
        // Set value (string to be copied)
        tmpTxtArea.value = text;
        // Set non-editable to avoid focus and move outside of view
        tmpTxtArea.setAttribute('readonly', '');
        tmpTxtArea.style = { position: 'absolute', left: '-9999px' };
        document.body.appendChild(tmpTxtArea);
        // Select text inside element
        tmpTxtArea.select();
        // Copy text to clipboard
        document.execCommand('copy');
        // Remove temporary element
        document.body.removeChild(tmpTxtArea);
    }
}

function getChatName() {
    const date = new Date();
    const year = date.getFullYear();
    const monthNames = ["Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec"];
    const month = monthNames[date.getMonth()];
    const
        day = date.getDate().toString().padStart(2, '0'); // Pad day with leading zero if needed

    const now = new Date(); // Get the current date and time

    // Format the time
    const hours = String(now.getHours()).padStart(2, '0'); // Get hours
    const minutes = String(now.getMinutes()).padStart(2, '0'); // Get minutes
    //const seconds = String(now.getSeconds()).padStart(2, '0'); // Get seconds

    return `${month}${day}${year}-${hours}`;
}

function buildLocalStorageName(elementName) {
    return prefix_localstorage_name + current_version + '-' + elementName;
}

//Delete data stored in local memory  based on a prefix.
function deleteVariablesStartingWith(prefix) {
    const allKeys = Object.keys(localStorage);
    const matchingKeys = allKeys.filter(key => key.startsWith(prefix));
    matchingKeys.forEach(key => localStorage.removeItem(key));
}

function saveContext(modelName, lContext) {
    const e = buildLocalStorageName(context_storage) + '.' + modelName;
    localStorage.setItem(e, JSON.stringify(lContext));
}

function retrieveContext(modelName) {
    const e = buildLocalStorageName(context_storage) + '.' + modelName;
    const ctx = localStorage.getItem(e);
    if (ctx) {
        return JSON.parse(ctx);
    }
    //ToDo: Search History for last context as last resource
    //const chatHist = document.getElementById("chat-history");

    return null;
}

//Function to retrieve the models available in the server.
async function getAvailableModels(url){
    const response = await fetch(model_URL);
    const data = await response.json();
    return data;
  }

// Function to send a POST request to the API
function postRequest(url, data, signal) {
    //showNotiMsg("Array","context",JSON.stringify(data));
    //console.log(JSON.stringify(data));
    return fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
        signal: signal
    });
}

// Function to stream the response from the server
async function getStreamingResponse(response, callback) {
    const timeout = 300000; // 300 seconds timeout
    const reader = response.body.getReader();
    let partialLine = '';

    while (true) {
        const { value, done } = await Promise.race([
            reader.read(),
            new Promise((resolve) => setTimeout(resolve, timeout))
        ]);
        if (done) {
            break;
        }
        // Decode the received value and split by lines
        const textChunk = new TextDecoder().decode(value);
        const lines = (partialLine + textChunk).split('\n');
        partialLine = lines.pop(); // The last line might be incomplete

        for (const line of lines) {
            if (line.trim() === '') continue;
            const parsedResponse = JSON.parse(line);
            callback(parsedResponse); // Process each response word
        }

    }

    // Handle any remaining line
    if (partialLine.trim() !== '') {
        const parsedResponse = JSON.parse(partialLine);
        callback(parsedResponse);
    }
}


// Event listener for Ctrl + Enter or CMD + Enter
document.getElementById('user-input').addEventListener('keydown', function (e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      submitRequest();
    }
  });
  
  
  window.onload = () => {
    updateChatList();
    populateModels();
    adjustPadding();
    autoFocusInput();
  
    document.getElementById("delete-chat").addEventListener("click", deleteChat);
    document.getElementById("new-chat").addEventListener("click", startNewChat);
    document.getElementById("chat-select").addEventListener("change", loadSelectedChat);
  }
  
  function deleteChat() {
    const selectedChat = document.getElementById("chat-select").value;
    localStorage.removeItem(selectedChat);
    updateChatList();
  }
  
  // Function to save chat with a unique name
  function saveChat() {
    const chatName = getChatName(); //getCurrentDate(); //document.getElementById('userName').value;
  
    if (chatName === null || chatName.trim() === "") return;
    const history = document.getElementById("chat-history").innerHTML;
    const context = document.getElementById('chat-history').context; //ToDo: Replace with LocalStorage??? or Last Context Attached to Last model.
    //const systemPrompt = document.getElementById('system-prompt').value;
    const model = getSelectedModel();
  
    const eChatName = chat_storage+'.'+chatName;
    localStorage.setItem(eChatName, JSON.stringify({"history":history, "context":context, "model": model}));
    updateChatList();
  }
  
  // Function to load selected chat from dropdown
  function loadSelectedChat() {
    const selectedChat = document.getElementById("chat-select").value;
    const obj = JSON.parse(localStorage.getItem(selectedChat));
  
    document.getElementById("chat-history").innerHTML = obj.history;
    document.getElementById("chat-history").context = obj.context;
    document.getElementById("system-prompt").value = obj.system;
    document.getElementById("chat-container").style.display = 'block';
  }
  
  //Reset Chat. Clean ALL!
  function startNewChat() {
      document.getElementById("chat-history").innerHTML = null;
      deleteVariablesStartingWith( buildLocalStorageName(context_storage));
      document.getElementById('chat-container').style.display = 'none';
      updateChatList();
  }
  
  // Function to update chat list dropdown WIP!
  function updateChatList() {
    const chatList = document.getElementById("chat-select");
    chatList.innerHTML = '<option value="" disabled selected>Select a chat</option>';
  
    const allKeys = Object.keys(localStorage);
    const matchingKeys = allKeys.filter(key => key.startsWith(chat_storage));
    
    matchingKeys.forEach( key => {   const option = document.createElement("option");
    option.value = key;
    option.text = key;
    chatList.add(option);} );
  }
  

